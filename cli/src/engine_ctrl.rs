use anyhow::Result;

use crossterm::event::KeyEvent as CTKeyEvent;
use khiin::Engine;
use khiin_protos::command::Command;
use khiin_protos::command::CommandType;
use khiin_protos::command::Request;
use protobuf::Message;

use crate::keys::translate_keys;

pub struct EngineCtrl {
    engine: Engine,
}

impl EngineCtrl {
    pub fn new(db_path: String) -> Result<Self> {
        let engine = Engine::new(db_path.as_str()).unwrap();
        Ok(Self { engine })
    }

    pub fn send_key(&mut self, key: CTKeyEvent) -> Result<Command> {
        let cmd = build_command(key);
        self.send_command(cmd)
    }

    pub fn reset(&mut self) -> Result<Command> {
        let mut cmd = Command::new();
        let mut req = Request::new();
        req.type_ = CommandType::CMD_RESET.into();
        cmd.request = Some(req).into();
        self.send_command(cmd)
    }

    fn send_command(&mut self, cmd: Command) -> Result<Command> {
        let bytes = cmd.write_to_bytes()?;
        let bytes = self.engine.send_command_bytes(&bytes)?;
        let cmd = Command::parse_from_bytes(&bytes)?;
        Ok(cmd)
    }
}

fn build_command(key: CTKeyEvent) -> Command {
    let key_event = translate_keys(key);

    let mut req = Request::new();
    req.key_event = Some(key_event).into();
    req.type_ = CommandType::CMD_SEND_KEY.into();

    let mut cmd = Command::new();
    cmd.request = Some(req).into();
    cmd
}
