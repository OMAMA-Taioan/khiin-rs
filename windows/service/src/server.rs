use anyhow::Result;
use futures::io::BufReader;
use futures::AsyncWriteExt;
use interprocess::local_socket::tokio::LocalSocketListener;
use interprocess::local_socket::tokio::OwnedReadHalf;
use interprocess::local_socket::tokio::OwnedWriteHalf;
use khiin_protos::command::CommandType;
use protobuf::Message;
use std::future::Future;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::sync::Semaphore;
use tokio::time::sleep;
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;

use khiin_protos::command::Command;
use khiin_protos::helpers::parse_u32_delimited_bytes_async;

use crate::engine_handler::EngineHandler;
use crate::engine_handler::EngineMessage;

const MAX_CONNECTIONS: usize = 1;
const NO_CONNECTION_TIMEOUT: u64 = 300;

struct Shutdown {
    is_shutdown: bool,
    notify: broadcast::Sender<()>,
}

impl Shutdown {
    fn new(notify: broadcast::Sender<()>) -> Self {
        Self {
            is_shutdown: false,
            notify,
        }
    }

    async fn recv(&mut self) {
        if self.is_shutdown {
            return;
        }

        let _ = self.notify.subscribe().recv().await;

        self.is_shutdown = true;
    }
}

struct SocketHandler {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    cancel_token: CancellationToken,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
    engine_tx: mpsc::Sender<EngineMessage>,
}

impl SocketHandler {
    async fn run(&mut self) -> Result<()> {
        while !self.shutdown.is_shutdown {
            let maybe_proto = tokio::select! {
                res = parse_u32_delimited_bytes_async::<Command, _>(
                    &mut self.reader
                ) => {
                    res.ok()
                },
                _ = self.shutdown.recv() => {
                    return Ok(());
                }
            };

            let proto = match maybe_proto {
                Some(proto) => proto,
                None => return Ok(()),
            };

            if proto.request.type_.enum_value_or_default()
                == CommandType::CMD_SHUTDOWN
            {
                self.cancel_token.cancel();
                self.shutdown.is_shutdown = true;
                break;
            }

            let bytes = proto.write_to_bytes()?;

            let (tx, rx) = oneshot::channel();

            self.engine_tx.send((bytes, tx)).await?;

            if let Ok(mut bytes) = rx.await {
                let len = (bytes.len() as u32).to_le_bytes();
                bytes.splice(..0, len.iter().cloned());
                self.writer.write_all(&bytes).await?;
            }
        }

        Ok(())
    }
}

struct SockerListener {
    listener: LocalSocketListener,
    limit_connections: Arc<Semaphore>,
    notify_shutdown: broadcast::Sender<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
    cancel_token: CancellationToken,
    engine_tx: mpsc::Sender<EngineMessage>,
}

impl SockerListener {
    async fn run(&mut self) -> Result<()> {
        let timeout = Duration::from_secs(NO_CONNECTION_TIMEOUT);
        let task_count = Arc::new(AtomicUsize::new(0));

        loop {
            let cancel_token = self.cancel_token.clone();

            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();

            let shutdown = Shutdown::new(self.notify_shutdown.clone());

            let conn = tokio::select! {
                conn = self.listener.accept() => conn?,
                _ = sleep(timeout) => {
                    if task_count.load(Ordering::SeqCst) == 0 {
                        return Ok(());
                    }

                    continue;
                },
                _ = cancel_token.cancelled() => {
                    log::debug!("Shutdown request received.");
                    return Ok(());
                },
            };

            let (reader, writer) = conn.into_split();

            let mut handler = SocketHandler {
                reader: BufReader::new(reader),
                writer,
                cancel_token,
                shutdown,
                _shutdown_complete: self.shutdown_complete_tx.clone(),
                engine_tx: self.engine_tx.clone(),
            };

            log::debug!("Spawning thread");
            let count = task_count.clone();
            tokio::spawn(async move {
                count.fetch_add(1, Ordering::SeqCst);

                if let Err(err) = handler.run().await {
                    log::error!("Connection error: {}", err);
                }

                drop(permit);
                log::debug!("Ending thread");
                count.fetch_sub(1, Ordering::SeqCst);
            });
        }
    }
}

pub async fn run(listener: LocalSocketListener, shutdown: impl Future) {
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);
    let (engine_tx, engine_rx) = mpsc::channel(1);

    let mut engine_handler = EngineHandler::new(engine_rx);

    tokio::spawn(async move {
        engine_handler.run().await;
    });

    let mut server = SockerListener {
        listener,
        limit_connections: Arc::new(Semaphore::new(MAX_CONNECTIONS)),
        notify_shutdown,
        shutdown_complete_tx,
        cancel_token: CancellationToken::new(),
        engine_tx,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                log::error!("Failed to accept: {}", err);
            }
        },
        _ = shutdown => {
            log::debug!("Shutting down!");
        }
    }

    let SockerListener {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;
    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;
    log::debug!("Shutdown complete.");
}
