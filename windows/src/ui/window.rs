use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Dwm::DWMWCP_ROUND;
use windows::Win32::UI::Controls::WM_MOUSELEAVE;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
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

use crate::geometry::Point;
use crate::ui::dpi::dpi_aware;
use crate::ui::dpi::Density;
use crate::ui::dwm::set_rounded_corners;
use crate::ui::render_factory::RenderFactory;
use crate::winerr;

// These were previously in GuiWindow class
// in c++ version
pub struct WindowData {
    pub handle: Option<HWND>,
    pub showing: bool,
    pub tracking_mouse: bool,
    pub max_width: i32,
    pub max_height: i32,
    pub dpi_parent: i32,
    pub dpi: u32,
    pub scale: f32,
    pub factory: Arc<RenderFactory>,
    pub target: ID2D1DCRenderTarget,
    pub origin: Point<i32>,
}

pub trait WindowHandler {
    const WINDOW_CLASS_NAME: &'static str;

    fn window_data(&self) -> Rc<RefCell<WindowData>>;

    fn on_message(
        &self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            let handle = self.window_data().borrow().handle;

            if handle.is_none() {
                return DefWindowProcW(HWND(0), message, wparam, lparam);
            }

            let handle = handle.unwrap();

            match message {
                WM_NCCREATE => {
                    set_rounded_corners(handle, DWMWCP_ROUND);
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

            DefWindowProcW(handle, message, wparam, lparam)
        }
    }

    fn set_handle(&self, handle: HWND) -> Result<()> {
        match self.window_data().try_borrow_mut() {
            Ok(mut window) => {
                window.handle = Some(handle);
                Ok(())
            }
            _ => winerr!(E_FAIL),
        }

        // match self.window_data().write() {
        //     Ok(mut window) => {
        //         window.handle = handle;
        //         Ok(())
        //     }
        //     _ => winerr!(E_FAIL),
        // }
    }

    fn handle(&self) -> Result<HWND> {
        match self.window_data().try_borrow() {
            Ok(window) => window.handle.ok_or(Error::from(E_FAIL)),
            _ => winerr!(E_FAIL),
        }
    }

    fn show(&self, pt: Point<i32>) -> Result<()> {
        let mut handle = HWND(0);

        if let Ok(mut window) = self.window_data().try_borrow_mut() {
            let dpi = window.dpi;
            window.origin = if dpi_aware() {
                pt
            } else {
                Point {
                    x: pt.x.to_dp(dpi) as i32,
                    y: pt.y.to_dp(dpi) as i32,
                }
            };

            window.showing = true;
            window.tracking_mouse = true;
            handle = window.handle.unwrap();
        }

        if handle != HWND(0) {
            unsafe {
                ShowWindow(handle, SW_SHOWNA);
            }
            Ok(())
        } else {
            winerr!(E_FAIL)
        }
    }

    fn hide(&self) -> Result<()> {
        let mut handle = HWND(0);
        let mut tracking = false;

        match self.window_data().try_borrow() {
            Ok(window) => {
                if !window.showing {
                    return Ok(());
                }
                handle = window.handle.unwrap();
                tracking = window.tracking_mouse;
            },
            _ => return Err(Error::from(E_FAIL))
        }

        unsafe {
            ShowWindow(handle, SW_HIDE);
            if tracking {
                ReleaseCapture();
            }
        }

        match self.window_data().try_borrow_mut() {
            Ok(mut window) => {
                window.showing = false;
                window.tracking_mouse = false;
                Ok(())
            },
            _ => winerr!(E_FAIL)
        }
    }

    fn on_create(&self) -> Result<()> {
        Ok(())
    }

    fn on_display_change(&self) {
        // TODO
        return;
    }

    fn on_dpi_changed(&self) {
        // TODO
        return;
    }

    // fn on_mouse_activate(&self) {
    //     todo!()
    // }

    fn on_mouse_move(&self) {
        // TODO
        return;
    }

    fn on_mouse_leave(&self) {
        // TODO
        return;
    }

    fn on_click(&self) -> bool {
        true // TODO
    }

    fn render(&self) -> Result<()>;

    fn on_resize(&self) {
        // TODO
        return;
    }

    fn on_window_pos_changing(&self) {
        // TODO
        return;
    }
}
