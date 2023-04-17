use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::GetMonitorInfoW;
use windows::Win32::Graphics::Gdi::MonitorFromWindow;
use windows::Win32::Graphics::Gdi::MONITORINFO;
use windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::UI::WindowsAndMessaging::GetParent;
use windows::Win32::UI::WindowsAndMessaging::USER_DEFAULT_SCREEN_DPI;

use crate::geometry::Point;
use crate::geometry::Rect;
use crate::geometry::Size;
use crate::ui::dpi::dpi_aware;

pub trait Hwnd {
    fn dpi(&self) -> u32;
    fn max_size(&self) -> Size<i32>;
    fn contains_pt(&self, pt: Point<i32>) -> bool;
}

impl Hwnd for HWND {
    fn contains_pt(&self, pt: Point<i32>) -> bool {
        let mut rect = RECT::default();
        let err = unsafe { GetClientRect(*self, &mut rect) };
        if err != FALSE {
            let rect: Rect<i32> = rect.into();
            rect.contains(pt)
        } else {
            false
        }
    }

    fn dpi(&self) -> u32 {
        if !dpi_aware() {
            return USER_DEFAULT_SCREEN_DPI;
        }
        
        unsafe {
            let dpi = GetDpiForWindow(GetParent(*self));

            if dpi == 0 {
                GetDpiForWindow(*self)
            } else {
                dpi
            }
        }
    }

    fn max_size(&self) -> Size<i32> {
        let hmon = unsafe {
            MonitorFromWindow(GetParent(*self), MONITOR_DEFAULTTONEAREST)
        };
        let mut info = MONITORINFO::default();
        info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        unsafe {
            GetMonitorInfoW(hmon, &mut info);
        }

        Size {
            w: info.rcMonitor.right,
            h: info.rcMonitor.bottom,
        }
    }
}
