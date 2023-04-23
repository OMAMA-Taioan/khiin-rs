use core::slice;
use std::ffi::c_char;
use std::ffi::CString;
use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn rust_bytes_to_string(
    data: *const u8,
    len: usize,
) -> *mut c_void {
    let bytes = unsafe { slice::from_raw_parts(data, len) };
    let message = String::from_utf8_lossy(bytes);
    let engine = khiin::Engine::new(&message).unwrap();

    let ptr = Box::into_raw(Box::new(engine));
    ptr as *mut c_void
}

#[no_mangle]
pub extern "C" fn rust_string_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(s));
    }
}
