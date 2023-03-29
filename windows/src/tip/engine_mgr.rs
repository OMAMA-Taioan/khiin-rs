use protobuf::MessageField;

use crate::protos::command::Command;
use crate::protos::command::KeyEvent;
use crate::protos::command::Request;
use crate::tip::key_event::KeyEvent as WinKeyEvent;

// just a stub
pub fn test_key() -> bool {
    false
}

pub fn translate_key_event(input: WinKeyEvent) -> KeyEvent {
    let mut proto = KeyEvent::new();
    proto.key_code = input.keycode as i32;
    proto
}

pub struct EngineMgr;

impl EngineMgr {
    pub fn new() -> Self {
        EngineMgr
    }

    pub fn on_key(&self, key_event: WinKeyEvent) -> Command {
        let key_event = translate_key_event(key_event);
        let mut req = Request::new();
        req.key_event = MessageField::some(key_event);
        let mut cmd = Command::new();
        cmd.request = MessageField::some(req);
        cmd
    }
}
