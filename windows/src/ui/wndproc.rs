use std::ffi::c_void;
use std::mem::size_of;
use std::mem::transmute;
use std::sync::Arc;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Dwm::DWMWCP_ROUND;
use windows::Win32::Graphics::Gdi::GetStockObject;
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::Graphics::Gdi::HGDIOBJ;
use windows::Win32::Graphics::Gdi::NULL_BRUSH;
use windows::Win32::UI::HiDpi::SetThreadDpiAwarenessContext;
use windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2;
use windows::Win32::UI::WindowsAndMessaging::CreateWindowExW;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::DestroyWindow;
use windows::Win32::UI::WindowsAndMessaging::GetClassInfoExW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::RegisterClassExW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::UnregisterClassW;
use windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
use windows::Win32::UI::WindowsAndMessaging::CS_HREDRAW;
use windows::Win32::UI::WindowsAndMessaging::CS_IME;
use windows::Win32::UI::WindowsAndMessaging::CS_VREDRAW;
use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;
use windows::Win32::UI::WindowsAndMessaging::GWLP_USERDATA;
use windows::Win32::UI::WindowsAndMessaging::HCURSOR;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::HMENU;
use windows::Win32::UI::WindowsAndMessaging::HWND_DESKTOP;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;

use crate::ui::window::WindowHandler;
use crate::utils::ToPcwstr;
use crate::winerr;

use super::dwm::set_rounded_corners;

pub trait Wndproc<T>: WindowHandler
where
    T: WindowHandler,
{
    // Should be called when the app starts
    fn register_class(module: HMODULE) -> bool {
        unsafe {
            let class_name = Self::WINDOW_CLASS_NAME.to_pcwstr();
            let mut wc = WNDCLASSEXW::default();

            if GetClassInfoExW(module, *class_name, &mut wc)
                != BOOL::from(false)
            {
                // already registered
                return true;
            }

            let wc = WNDCLASSEXW {
                cbSize: size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW | CS_IME,
                lpfnWndProc: Some(Self::wndproc),
                cbClsExtra: 0,
                hInstance: module,
                lpszClassName: *class_name,
                hIcon: HICON::default(),
                hIconSm: HICON::default(),
                hCursor: HCURSOR::default(),
                lpszMenuName: PCWSTR::null(),
                hbrBackground: transmute::<HGDIOBJ, HBRUSH>(GetStockObject(
                    NULL_BRUSH,
                )),
                cbWndExtra: 0,
            };

            RegisterClassExW(&wc) != 0
        }
    }

    // Should be called when the app is deactivated
    fn unregister_class(module: HMODULE) -> bool {
        unsafe {
            let class_name = Self::WINDOW_CLASS_NAME.to_pcwstr();
            let mut wc = WNDCLASSEXW::default();

            if GetClassInfoExW(module, *class_name, &mut wc)
                == BOOL::from(false)
            {
                // already unregistered
                return true;
            }

            UnregisterClassW(*class_name, module).0 != 0
        }
    }

    fn create(
        this: Arc<Self>,
        module: HMODULE,
        window_name: &str,
        dwstyle: u32,
        dwexstyle: u32,
    ) -> Result<()> {
        unsafe {
            let class_name = Self::WINDOW_CLASS_NAME.to_pcwstr();
            if !Self::register_class(module) {
                return winerr!(E_FAIL);
            }

            let window_name = if window_name.is_empty() {
                "".to_pcwstr()
            } else {
                window_name.to_pcwstr()
            };

            let previous_dpi_awareness = SetThreadDpiAwarenessContext(
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            );

            let this_ptr = Arc::into_raw(this.clone());

            let handle = CreateWindowExW(
                WINDOW_EX_STYLE(dwexstyle),
                *class_name,
                *window_name,
                WINDOW_STYLE(dwstyle),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND_DESKTOP,
                HMENU::default(),
                module,
                Some(this_ptr as *mut c_void),
            );

            this.set_handle(Some(handle))?;

            SetThreadDpiAwarenessContext(previous_dpi_awareness);

            if this.handle().unwrap() == HWND::default() {
                winerr!(E_FAIL)
            } else {
                Ok(())
            }
        }
    }

    fn destroy(&self) -> Result<()> {
        let handle = self.handle()?;
        self.set_handle(None)?;

        unsafe {
            DestroyWindow(handle);
        }
        Ok(())
    }

    // This is the external window procedure that will be called
    // by Windows. The main goal here is to catch the very first window
    // message, WM_NCCREATE, in order to save a pointer to this object
    // for subsequent messages, and to route those messages to the
    // "on_message" method of the Window trait.
    extern "system" fn wndproc(
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let lpcs: &CREATESTRUCTW = transmute(lparam);
                SetWindowLongPtrW(
                    handle,
                    GWLP_USERDATA,
                    lpcs.lpCreateParams as _,
                );
                set_rounded_corners(handle, DWMWCP_ROUND).ok();
                return DefWindowProcW(handle, message, wparam, lparam);
            }

            let userdata = GetWindowLongPtrW(handle, GWLP_USERDATA);
            let this = std::ptr::NonNull::<T>::new(userdata as _);
            let handled = this.map_or(false, |mut s| {
                s.as_mut()
                    .on_message(handle, message, wparam, lparam)
                    .is_ok()
            });
            if handled {
                LRESULT::default()
            } else {
                DefWindowProcW(handle, message, wparam, lparam)
            }
        }
    }
}
