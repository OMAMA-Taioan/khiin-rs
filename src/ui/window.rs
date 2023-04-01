use std::ffi::c_void;
use std::mem::size_of;
use std::mem::transmute;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Dwm::DwmSetWindowAttribute;
use windows::Win32::Graphics::Dwm::DWMWA_WINDOW_CORNER_PREFERENCE;
use windows::Win32::Graphics::Dwm::DWMWCP_ROUND;
use windows::Win32::Graphics::Dwm::DWM_WINDOW_CORNER_PREFERENCE;
use windows::Win32::Graphics::Gdi::GetStockObject;
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::Graphics::Gdi::HGDIOBJ;
use windows::Win32::Graphics::Gdi::NULL_BRUSH;
use windows::Win32::UI::Controls::WM_MOUSELEAVE;
use windows::Win32::UI::HiDpi::SetThreadDpiAwarenessContext;
use windows::Win32::UI::HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2;
use windows::Win32::UI::WindowsAndMessaging::CreateWindowExW;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::DestroyWindow;
use windows::Win32::UI::WindowsAndMessaging::GetClassInfoExW;
use windows::Win32::UI::WindowsAndMessaging::RegisterClassExW;
use windows::Win32::UI::WindowsAndMessaging::UnregisterClassW;
use windows::Win32::UI::WindowsAndMessaging::CS_HREDRAW;
use windows::Win32::UI::WindowsAndMessaging::CS_IME;
use windows::Win32::UI::WindowsAndMessaging::CS_VREDRAW;
use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;
use windows::Win32::UI::WindowsAndMessaging::HCURSOR;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::HMENU;
use windows::Win32::UI::WindowsAndMessaging::HWND_DESKTOP;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WM_CREATE;
use windows::Win32::UI::WindowsAndMessaging::WM_DISPLAYCHANGE;
use windows::Win32::UI::WindowsAndMessaging::WM_DPICHANGED;
use windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_MOUSEACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::WM_MOUSEMOVE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
use windows::Win32::UI::WindowsAndMessaging::WM_PAINT;
use windows::Win32::UI::WindowsAndMessaging::WM_SIZE;
use windows::Win32::UI::WindowsAndMessaging::WM_WINDOWPOSCHANGING;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;

use crate::geometry::Point;
use crate::pcwstr;
use crate::ui::render_factory::RenderFactory;
use crate::winerr;

unsafe fn set_rounded_corners(hwnd: HWND, pref: DWM_WINDOW_CORNER_PREFERENCE) {
    let pref = Box::into_raw(Box::new(pref)) as *mut c_void;
    let _tmp = DwmSetWindowAttribute(
        hwnd,
        DWMWA_WINDOW_CORNER_PREFERENCE,
        pref,
        std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
    );
}

// These were previously in GuiWindow class
// in c++ version
pub struct WindowData {
    pub handle: HWND,
    pub showing: bool,
    pub tracking_mouse: bool,
    pub max_width: u32,
    pub max_height: u32,
    pub dpi_parent: u32,
    pub dpi: u32,
    pub scale: f32,
    pub factory: RenderFactory,
    pub target: ID2D1DCRenderTarget,
}

pub trait Window {
    const WINDOW_CLASS_NAME: &'static str;

    fn create(
        &mut self,
        module: HMODULE,
        window_name: &str,
        dwstyle: u32,
        dwexstyle: u32,
    ) -> Result<()> {
        unsafe {
            let class_name = pcwstr!(Self::WINDOW_CLASS_NAME);
            if !Self::register_class(module) {
                return winerr!(E_FAIL);
            }

            let window_name = if window_name.is_empty() {
                PCWSTR::null()
            } else {
                pcwstr!(window_name)
            };

            let previous_dpi_awareness = SetThreadDpiAwarenessContext(
                DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
            );

            CreateWindowExW(
                WINDOW_EX_STYLE(dwexstyle),
                class_name,
                window_name,
                WINDOW_STYLE(dwstyle),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND_DESKTOP,
                HMENU::default(),
                module,
                Some(self as *mut _ as _),
            );

            SetThreadDpiAwarenessContext(previous_dpi_awareness);

            if self.hwnd() == HWND::default() {
                winerr!(E_FAIL)
            } else {
                Ok(())
            }
        }
    }

    fn destroy(&self) {
        let hwnd = self.hwnd();
        if hwnd != HWND::default() {
            unsafe {
                DestroyWindow(hwnd);
            }
        }
    }

    fn register_class(module: HMODULE) -> bool {
        unsafe {
            let class_name = pcwstr!(Self::WINDOW_CLASS_NAME);

            let mut wc = WNDCLASSEXW::default();

            if GetClassInfoExW(module, class_name, &mut wc) != BOOL::from(false)
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
                lpszClassName: class_name,
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

    fn unregister_class(module: HMODULE) -> bool {
        unsafe {
            let class_name = pcwstr!(Self::WINDOW_CLASS_NAME);
            UnregisterClassW(class_name, module).0 != 0
        }
    }

    fn on_message(&self, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match umsg {
                WM_NCCREATE => {
                    set_rounded_corners(self.hwnd(), DWMWCP_ROUND);
                }
                WM_CREATE => {
                    if self.on_create().is_ok() {
                        return LRESULT(0);
                    }
                    return LRESULT(1);
                }
                WM_DISPLAYCHANGE => {
                    self.on_display_change();
                }
                WM_DPICHANGED => {
                    self.on_dpi_changed();
                    return LRESULT(0);
                }
                WM_MOUSEACTIVATE => {
                    // self.on_mouse_activate();
                }
                WM_MOUSEMOVE => {
                    self.on_mouse_move();
                }
                WM_MOUSELEAVE => {
                    self.on_mouse_leave();
                }
                WM_LBUTTONDOWN => {
                    if self.on_click() {
                        return LRESULT(0);
                    }
                }
                WM_PAINT => {
                    self.render();
                    return LRESULT(0);
                }
                WM_SIZE => {
                    self.on_resize();
                }
                WM_WINDOWPOSCHANGING => {
                    self.on_window_pos_changing();
                }
                _ => (),
            };

            DefWindowProcW(self.hwnd(), umsg, wparam, lparam)
        }
    }

    extern "system" fn wndproc(
        hwnd: HWND,
        umsg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT;

    fn set_hwnd(&mut self, hwnd: HWND);
    fn hwnd(&self) -> HWND;
    fn showing(&self) -> bool;
    fn show(&mut self, pt: Point<i32>);
    fn hide(&mut self);
    fn on_create(&self) -> Result<()>;
    fn on_display_change(&self);
    fn on_dpi_changed(&self);
    // fn on_mouse_activate(&self);
    fn on_mouse_move(&self);
    fn on_mouse_leave(&self);
    fn on_click(&self) -> bool;
    fn render(&self);
    fn on_resize(&self);
    fn on_window_pos_changing(&self);
}
