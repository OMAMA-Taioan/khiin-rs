use std::cell::RefCell;
use std::mem::transmute;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
// use std::sync::mpsc::channel;
// use std::sync::mpsc::Sender;
use std::sync::Arc;
// use std::thread::JoinHandle;

use futures::io::BufReader;
use futures::join;
use futures::AsyncWriteExt;
use interprocess::local_socket::tokio::LocalSocketStream;
use interprocess::local_socket::NameTypeSupport;
use khiin_protos::helpers::parse_u32_delimited_bytes_async;
use khiin_protos::helpers::WriteDelim;
use protobuf::Message;
use std::process;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use windows::core::w;
use windows::core::Error;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
use windows::Win32::UI::WindowsAndMessaging::MB_DEFBUTTON2;

use khiin_protos::command::*;

use crate::dll::DllModule;
use crate::engine::message_handler::WM_KHIIN_COMMAND;
use crate::fail;
use crate::utils::GetPath;
use crate::winerr;

pub struct EngineCoordinator {
    runtime: Runtime,
    command_tx: mpsc::Sender<Command>,
    thread: RefCell<Option<JoinHandle<Result<()>>>>,
    shutdown: oneshot::Sender<()>,
}

struct Handler {
    command_rx: mpsc::Receiver<Command>,
    shutdown_rx: oneshot::Receiver<()>,
    callback_handle: HWND,
}

impl Handler {
    async fn run(&mut self) -> Result<()> {
        let conn = self.connect().await?;
        let (mut reader, mut writer) = conn.into_split();

        loop {
            let cmd = tokio::select! {
                cmd = self.command_rx.recv() => {
                    if cmd.is_none() {
                        continue;
                    }

                    cmd.unwrap()
                },
                msg = &mut self.shutdown_rx => {
                    break;
                }
            };

            let bytes = cmd.write_u32_delimited_bytes().unwrap();

            writer
                .write_all(&bytes)
                .await
                .map_err(|_| Error::from(E_FAIL))?;

            let mut reader = BufReader::new(&mut reader);
            let reply =
                parse_u32_delimited_bytes_async::<Command, _>(&mut reader)
                    .await
                    .map_err(|e| {
                        log::debug!("Error: {}", e);
                        return Error::from(E_FAIL);
                    })?;

            self.return_command(reply);
        }

        let result = Result::Ok(());
        result
    }

    fn get_socket_name(&self, id: String) -> String {
        use NameTypeSupport::*;
        let mut suffix = "".to_string();
        if !id.is_empty() {
            suffix.push_str(".");
            suffix.push_str(&id);
        }
        match NameTypeSupport::query() {
            OnlyPaths => format!("/tmp/khiin.sock{}", suffix),
            OnlyNamespaced | Both => format!("@khiin.sock{}", suffix),
        }
        
    }

    async fn connect(&self) -> Result<LocalSocketStream> {
        let id = rand::random::<u32>().to_string();
        let socket = self.get_socket_name(id.clone());

        let mut attempts = 0;
        let max_attempts = 100;
        let attempt_delay = 10;

        loop {
            self.launch_service(id.clone());

            tokio::time::sleep(std::time::Duration::from_millis(10)).await;

            let conn = match LocalSocketStream::connect(socket.as_str()).await {
                Ok(conn) => return Ok(conn),
                Err(e) => {
                    // let message = format!("{} Connection error: {}", socket, e);
                    // let wide_message: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
                    // unsafe {
                    //     MessageBoxW(
                    //         HWND::default(),
                    //         PCWSTR(wide_message.as_ptr()),
                    //         w!("OK"),
                    //         MB_DEFBUTTON2,
                    //     );
                    // }
                    if attempts >= max_attempts {
                        log::debug!("Connection error: {}", e);
                        return Err(Error::from(E_FAIL));
                    }
                    attempts += 1;
                },
            };

            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }

    fn launch_service(&self, service_id: String) -> Result<()> {
        let dll_path = DllModule::global().module.get_path()?;
        let mut svc_exe = PathBuf::from(dll_path);
        svc_exe.set_file_name("khiin_service.exe");

        // const CREATE_NO_WINDOW: u32 = 0x08000000;
        const DETACHED_PROCESS: u32 = 0x00000008;
        let mut cmd = process::Command::new(svc_exe)
            .arg(service_id)
            .creation_flags(DETACHED_PROCESS)
            .spawn()
            .map_err(|e| {
                log::error!("Failed to start service: {}", e);
                Error::from(E_FAIL)
            });

        Ok(())
    }

    fn return_command(&self, command: Command) {
        let raw_ptr = Arc::into_raw(Arc::new(command));
        let wparam = WPARAM(0);

        unsafe {
            let lparam: LPARAM = transmute(raw_ptr);

            PostMessageW(
                self.callback_handle,
                WM_KHIIN_COMMAND,
                wparam,
                lparam,
            );
        }
    }
}

impl EngineCoordinator {
    pub fn new(callback_handle: HWND) -> Result<Self> {
        let mut runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (command_tx, mut command_rx) = mpsc::channel::<Command>(1);

        let mut handler = Handler {
            command_rx,
            shutdown_rx,
            callback_handle,
        };

        let thread = runtime.spawn(async move {
            if let Err(e) = handler.run().await {
                log::error!("Handler error: {}", e);
                return Err(Error::from(E_FAIL));
            }

            Ok(())
        });

        Ok(Self {
            runtime,
            command_tx,
            thread: RefCell::new(Some(thread)),
            shutdown: shutdown_tx,
        })
    }

    pub fn send_command(&self, cmd: Command) -> Result<()> {
        let tx = self.command_tx.clone();

        self.runtime.spawn(async move {
            tx.send(cmd).await.map_err(|_| Error::from(E_FAIL))?;
            Result::Ok(())
        });

        Ok(())
    }

    pub fn shutdown(self) -> Result<()> {
        self.shutdown.send(());
        Ok(())
    }
}
