use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;
use std::sync::Arc;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Dwm::DWMWCP_ROUND;
use windows::Win32::Graphics::Gdi::GetMonitorInfoW;
use windows::Win32::Graphics::Gdi::MonitorFromWindow;
use windows::Win32::Graphics::Gdi::MONITORINFO;
use windows::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST;
use windows::Win32::UI::Controls::WM_MOUSELEAVE;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::GetParent;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::USER_DEFAULT_SCREEN_DPI;
use windows::Win32::UI::WindowsAndMessaging::WM_CREATE;
use windows::Win32::UI::WindowsAndMessaging::WM_DISPLAYCHANGE;
use windows::Win32::UI::WindowsAndMessaging::WM_DPICHANGED;
use windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_MOUSEMOVE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
use windows::Win32::UI::WindowsAndMessaging::WM_PAINT;
use windows::Win32::UI::WindowsAndMessaging::WM_SHOWWINDOW;
use windows::Win32::UI::WindowsAndMessaging::WM_SIZE;
use windows::Win32::UI::WindowsAndMessaging::WM_WINDOWPOSCHANGING;

use crate::geometry::Point;
use crate::geometry::Rect;
use crate::ui::dpi::dpi_aware;
use crate::ui::dpi::Density;
use crate::ui::dwm::set_rounded_corners;
use crate::ui::render_factory::RenderFactory;
use crate::utils::get_x_param;
use crate::utils::get_y_param;
use crate::utils::hi_word;
use crate::utils::lo_word;
use crate::winerr;

// These were previously in GuiWindow class
// in c++ version
#[derive(Clone)]
pub struct WindowData {
    pub handle: Option<HWND>,
    pub showing: bool,
    pub tracking_mouse: bool,
    pub max_width: i32,
    pub max_height: i32,
    pub dpi_parent: u32,
    pub dpi: u32,
    pub scale: f32,
    pub factory: Arc<RenderFactory>,
    pub target: ID2D1DCRenderTarget,
    pub origin: Point<i32>,
}

impl WindowData {
    pub fn new(factory: Arc<RenderFactory>) -> Result<Self> {
        Ok(Self {
            handle: None,
            showing: false,
            tracking_mouse: false,
            max_width: 0,
            max_height: 0,
            dpi_parent: 0,
            dpi: 0,
            scale: 0.0,
            origin: Point::default(),
            target: factory.create_dc_render_target()?,
            factory,
        })
    }
}

pub trait WindowHandler {
    const WINDOW_CLASS_NAME: &'static str;

    fn window_data(&self) -> Rc<RefCell<WindowData>>;

    fn on_message(
        &mut self,
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<()> {
        match message {
            WM_NCCREATE => set_rounded_corners(handle, DWMWCP_ROUND),
            WM_CREATE => self.on_create(handle),
            WM_DISPLAYCHANGE => self.on_monitor_change(handle),
            WM_DPICHANGED => {
                let dpi = hi_word(wparam.0 as u32);
                let rect: &RECT = unsafe { transmute(lparam) };
                self.on_dpi_changed(handle, dpi, rect.into())
            }
            WM_MOUSEMOVE => {
                let x = get_x_param(lparam);
                let y = get_y_param(lparam);
                self.on_mouse_move(handle, Point { x, y })
            }
            WM_MOUSELEAVE => self.on_mouse_leave(),
            WM_LBUTTONDOWN => self.on_click(),
            WM_SHOWWINDOW => match wparam.0 {
                0 => self.on_hide_window(),
                _ => self.on_show_window(),
            },
            WM_PAINT => self.render(handle),
            WM_SIZE => self
                .on_resize(lo_word(lparam.0 as u32), hi_word(lparam.0 as u32)),
            WM_WINDOWPOSCHANGING => self.on_monitor_change(handle),
            _ => winerr!(E_FAIL),
        }
    }

    fn reset_render_target(&self) -> Result<()> {
        if let Ok(mut window) = self.window_data().try_borrow_mut() {
            let target = window.factory.create_dc_render_target()?;
            window.target = target.clone();
        }

        Ok(())
    }

    fn set_handle(&self, handle: Option<HWND>) -> Result<()>;

    fn handle(&self) -> Result<HWND> {
        if let Ok(window) = self.window_data().try_borrow() {
            if let Some(handle) = window.handle {
                return Ok(handle);
            }
        }
        winerr!(E_FAIL)
    }

    fn set_origin(&self, pt: Point<i32>) -> Result<()> {
        if let Ok(mut window) = self.window_data().try_borrow_mut() {
            let dpi = window.dpi;
            if !dpi_aware() {
                window.origin = Point {
                    x: pt.x.to_dp(dpi) as i32,
                    y: pt.y.to_dp(dpi) as i32,
                };
            } else {
                window.origin = pt;
            }
            Ok(())
        } else {
            winerr!(E_FAIL)
        }
    }

    fn show(&self, _pt: Point<i32>) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn on_show_window(&self) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn hide(&self) -> Result<()> {
        let handle = self.handle()?;
        unsafe {
            ShowWindow(handle, SW_HIDE);
            ReleaseCapture();
        }
        Ok(())
    }

    fn on_hide_window(&self) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn on_create(&self, handle: HWND) -> Result<()> {
        {
            if let Ok(mut window) = self.window_data().try_borrow_mut() {
                window.dpi = unsafe { GetDpiForWindow(handle) };
                window.dpi_parent =
                    unsafe { GetDpiForWindow(GetParent(handle)) };
            }
        }
        self.on_monitor_change(handle)?;
        Ok(())
    }

    fn on_monitor_change(&self, handle: HWND) -> Result<()> {
        let hmon = unsafe {
            MonitorFromWindow(GetParent(handle), MONITOR_DEFAULTTONEAREST)
        };
        let mut info = MONITORINFO::default();
        info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        unsafe {
            GetMonitorInfoW(hmon, &mut info);
        }
        {
            if let Ok(mut window) = self.window_data().try_borrow_mut() {
                window.max_width = info.rcMonitor.right;
                window.max_height = info.rcMonitor.bottom;
                Ok(())
            } else {
                winerr!(E_FAIL)
            }
        }
    }

    fn set_dpi(&self, dpi: u16) -> Result<()> {
        if let Ok(mut window) = self.window_data().try_borrow_mut() {
            unsafe {
                window.target.SetDpi(dpi as f32, dpi as f32);
            }
            window.scale =
                window.dpi_parent as f32 / USER_DEFAULT_SCREEN_DPI as f32;
        }
        Ok(())
    }

    fn on_dpi_changed(
        &self,
        handle: HWND,
        dpi: u16,
        new_size: Rect<i32>,
    ) -> Result<()> {
        self.set_dpi(dpi)?;
        let Rect {
            origin: Point { x, y },
            width: w,
            height: h,
        } = new_size;
        unsafe {
            SetWindowPos(
                handle,
                None,
                x,
                y,
                w,
                h,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );
        }
        Ok(())
    }

    fn on_mouse_move(&self, handle: HWND, pt: Point<i32>) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn on_mouse_leave(&self) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn on_click(&self) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn on_resize(&self, width: u16, height: u16) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn render(&self, handle: HWND) -> Result<()>;
}
