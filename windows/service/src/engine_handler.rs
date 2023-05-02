use anyhow::anyhow;
use anyhow::Result;
use khiin::Engine;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub type EngineMessage = (Vec<u8>, oneshot::Sender<Vec<u8>>);

pub struct EngineHandler {
    rx: mpsc::Receiver<(Vec<u8>, oneshot::Sender<Vec<u8>>)>,
}

impl EngineHandler {
    pub fn new(rx: mpsc::Receiver<EngineMessage>) -> Self {
        Self { rx }
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut dbfile = std::env::current_exe().unwrap();
        dbfile.set_file_name("khiin.db");

        let engine = Engine::new(dbfile.to_str().unwrap());

        if engine.is_none() {
            return Err(anyhow!("Unable to start engine"));
        }

        let mut engine = engine.unwrap();

        while let Some((command_bytes, sender)) = self.rx.recv().await {
            let bytes = engine.send_command_bytes(&command_bytes)?;
            sender.send(bytes).map_err(|_| {
                log::error!("Unable to send bytes back from engine");
                return anyhow!("Unable to send bytes back from engine");
            })?;
        }

        Ok(())
    }
}
