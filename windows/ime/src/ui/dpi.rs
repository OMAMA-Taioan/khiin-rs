use windows::Win32::Foundation::BOOL;
use windows::Win32::System::WindowsProgramming::MulDiv;
use windows::Win32::UI::HiDpi::AreDpiAwarenessContextsEqual;
use windows::Win32::UI::HiDpi::GetThreadDpiAwarenessContext;
use windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE;
use windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2;
use windows::Win32::UI::WindowsAndMessaging::USER_DEFAULT_SCREEN_DPI;

static DEFAULT_DPI: i32 = USER_DEFAULT_SCREEN_DPI as i32;

pub fn dpi_aware() -> bool {
    unsafe {
        let TRUE = BOOL::from(true);
        let context = GetThreadDpiAwarenessContext();

        if AreDpiAwarenessContextsEqual(
            context,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
        ) == TRUE
        {
            return true;
        }

        if AreDpiAwarenessContextsEqual(
            context,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        ) == TRUE
        {
            return true;
        }

        false
    }
}

pub trait Density {
    fn to_dp(&self, dpi: u32) -> i32;
    fn to_px(&self, dpi: u32) -> i32;
}

impl Density for i32 {
    fn to_dp(&self, dpi: u32) -> i32 {
        unsafe { MulDiv(*self, DEFAULT_DPI, dpi as i32) }
    }
    
    fn to_px(&self, dpi: u32) -> i32 {
        unsafe { MulDiv(*self, dpi as i32, DEFAULT_DPI) }
    }
}

impl Density for u32 {
    fn to_dp(&self, dpi: u32) -> i32 {
        unsafe { MulDiv(*self as i32, DEFAULT_DPI, dpi as i32) }
    }
    
    fn to_px(&self, dpi: u32) -> i32 {
        unsafe { MulDiv(*self as i32, dpi as i32, DEFAULT_DPI) }
    }
}
