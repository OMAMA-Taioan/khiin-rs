use core::slice;
use std::ffi::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn rust_bytes_to_string(
    data: *const u8,
    len: usize,
) -> *mut c_char {
    let bytes = unsafe { slice::from_raw_parts(data, len) };
    let message = String::from_utf8_lossy(bytes);
    let c_string = CString::new(message.into_owned()).unwrap();
    c_string.into_raw()
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
