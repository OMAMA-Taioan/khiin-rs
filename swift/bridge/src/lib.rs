use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::slice;

use khiin::Engine;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type EngineController;

        #[swift_bridge(associated_to = EngineController)]
        fn new(db_filename: String) -> Option<EngineController>;

        #[swift_bridge(associated_to = EngineController, swift_name = "sendCommand")]
        fn send_command(
            &self,
            cmd_input: *const u8,
            len_input: usize,
            cmd_output: *mut *mut u8,
            len_output: *mut usize,
        ) -> i32;
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
        cmd_input: *const u8,
        len_input: usize,
        cmd_output: *mut *mut u8,
        len_output: *mut usize,
    ) -> i32 {
        let engine: &mut Engine =
            unsafe { &mut *(self.engine_ptr as *mut Engine) };

        let bytes = unsafe { slice::from_raw_parts(cmd_input, len_input) };
        let mut res_bytes = match engine.send_command_bytes(&bytes) {
            Ok(bytes) => bytes,
            Err(_) => return 1,
        };

        unsafe {
            *len_output = res_bytes.len();
            *cmd_output = res_bytes.as_mut_ptr();
        };
        
        std::mem::forget(res_bytes);
        0
    }
}

// #[no_mangle]
// pub extern "C" fn Rust_khiin_engine_send_command(
//     engine_ptr: *mut c_void,
// cmd_input: *const u8,
// len_input: usize,
// cmd_output: *mut *mut u8,
// len_output: *mut usize,
// ) -> i32 {
//     let engine: &mut Engine = unsafe { &mut *(engine_ptr as *mut Engine) };
//     let bytes = unsafe { slice::from_raw_parts(cmd_input, len_input) };
//     let mut res_bytes = match engine.send_command_bytes(bytes) {
//         Ok(bytes) => bytes,
//         Err(_) => return 1,
//     };

//     unsafe {
//         *len_output = res_bytes.len();
//         *cmd_output = res_bytes.as_mut_ptr();
//     }

//     std::mem::forget(res_bytes);
//     0
// }
