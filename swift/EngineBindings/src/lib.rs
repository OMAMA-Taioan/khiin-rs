use std::ffi::c_char;
use std::ffi::c_void;
use std::ffi::CStr;
use std::slice;

use khiin::Engine;

#[no_mangle]
pub extern "C" fn Rust_khiin_engine_load(
    db_filename: *const c_char,
) -> *mut c_void {
    let db_filename = unsafe { CStr::from_ptr(db_filename) };
    let db_filename = db_filename.to_string_lossy().into_owned();

    // TODO: Error handling, we might need to use an outparam and return a
    // success value similar to the send command function
    let engine = khiin::Engine::new(&db_filename).unwrap();
    let ptr = Box::into_raw(Box::new(engine));
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn Rust_khiin_engine_send_command(
    engine_ptr: *mut c_void,
    cmd_input: *const u8,
    len_input: usize,
    cmd_output: *mut *mut u8,
    len_output: *mut usize,
) -> i32 {
    let engine: &mut Engine = unsafe { &mut *(engine_ptr as *mut Engine) };
    let bytes = unsafe { slice::from_raw_parts(cmd_input, len_input) };
    let mut res_bytes = match engine.send_command_bytes(bytes) {
        Ok(bytes) => bytes,
        Err(_) => return 1,
    };

    unsafe {
        *len_output = res_bytes.len();
        *cmd_output = res_bytes.as_mut_ptr();
    }

    std::mem::forget(res_bytes);
    0
}

#[no_mangle]
pub extern "C" fn Rust_khiin_engine_shutdown(
    engine_ptr: *mut c_void,
) {
    let engine_ptr = engine_ptr as *mut Engine;
    unsafe { drop(Box::from_raw(engine_ptr)); }
}
