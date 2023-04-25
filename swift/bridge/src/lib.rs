use std::ffi::c_void;

use khiin::Engine;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type EngineController;

        #[swift_bridge(associated_to = EngineController)]
        fn new(db_filename: String) -> Option<EngineController>;

        #[swift_bridge(swift_name = "sendCommand")]
        fn send_command(
            &self,
            cmd_input: &[u8],
        ) -> Option<Vec<u8>>;
    }
}

pub struct EngineController {
    engine_ptr: *mut c_void,
}

impl EngineController {
    fn new(db_filename: String) -> Option<Self> {
        if let Some(engine) = khiin::Engine::new(&db_filename) {
            let ptr = Box::into_raw(Box::new(engine));
            let controller = EngineController {
                engine_ptr: ptr as *mut c_void,
            };
            return Some(controller);
        }

        None
    }

    fn send_command(
        &self,
        cmd_input: &[u8],
    ) -> Option<Vec<u8>> {
        let engine: &mut Engine =
            unsafe { &mut *(self.engine_ptr as *mut Engine) };

        engine.send_command_bytes(cmd_input).ok()
    }
}
