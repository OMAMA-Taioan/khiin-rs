use std::ffi::c_void;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::DwmSetWindowAttribute;
use windows::Win32::Graphics::Dwm::DWMWA_WINDOW_CORNER_PREFERENCE;
use windows::Win32::Graphics::Dwm::DWM_WINDOW_CORNER_PREFERENCE;

pub fn set_rounded_corners(hwnd: HWND, pref: DWM_WINDOW_CORNER_PREFERENCE) {
    unsafe {
        let pref = Box::into_raw(Box::new(pref)) as *mut c_void;
        let _tmp = DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            pref,
            std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        );
    }
}
