use std::ffi::c_void;

use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Dwm::DwmSetWindowAttribute;
use windows::Win32::Graphics::Dwm::DWMWA_WINDOW_CORNER_PREFERENCE;
use windows::Win32::Graphics::Dwm::DWM_WINDOW_CORNER_PREFERENCE;

pub fn set_rounded_corners(
    hwnd: HWND,
    pref: DWM_WINDOW_CORNER_PREFERENCE,
) -> Result<()> {
    unsafe {
        let mut pref = pref.0;
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &mut pref as *mut _ as *mut c_void,
            std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        )
    }
}
