use windows::Win32::Foundation::FALSE;
use windows::Win32::UI::HiDpi::AreDpiAwarenessContextsEqual;
use windows::Win32::UI::HiDpi::GetThreadDpiAwarenessContext;
use windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE;
use windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2;
use windows::Win32::UI::WindowsAndMessaging::USER_DEFAULT_SCREEN_DPI;

static DEFAULT_DPI: i32 = USER_DEFAULT_SCREEN_DPI as i32;

pub fn dpi_aware() -> bool {
    unsafe {
        let context = GetThreadDpiAwarenessContext();
        let mut is_dpi_aware: bool = false;

        if AreDpiAwarenessContextsEqual(
            context,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
        ) != FALSE
        {
            is_dpi_aware = true;
        }

        if AreDpiAwarenessContextsEqual(
            context,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        ) != FALSE
        {
            is_dpi_aware = true;
        }

        is_dpi_aware
    }
}

pub trait Density {
    fn to_dip(&self, dpi: u32) -> i32;
    fn to_px(&self, dpi: u32) -> i32;
}

impl Density for i32 {
    fn to_dip(&self, dpi: u32) -> i32 {
        let px = *self as f32;
        let dpi = dpi as f32;
        let base_dpi = DEFAULT_DPI as f32;

        (px * base_dpi / dpi).ceil() as i32
    }

    fn to_px(&self, dpi: u32) -> i32 {
        let dips = *self as f32;
        let dpi = dpi as f32;
        let base_dpi = DEFAULT_DPI as f32;

        (dips * dpi / base_dpi).ceil() as i32
    }
}

impl Density for u32 {
    fn to_dip(&self, dpi: u32) -> i32 {
        (*self as i32).to_dip(dpi)
    }

    fn to_px(&self, dpi: u32) -> i32 {
        (*self as i32).to_px(dpi)
    }
}
