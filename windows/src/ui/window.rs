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
use windows::Win32::UI::WindowsAndMessaging::WM_SHOWWINDOW;
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
#[derive(Clone)]
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
    fn set_window_data(&self, new_window: WindowData) -> Result<()>;

    fn on_message(
        &mut self,
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> bool {
        match message {
            WM_NCCREATE => {
                set_rounded_corners(handle, DWMWCP_ROUND);
            }
            WM_CREATE => {
                return self.on_create().is_ok();
            }
            WM_DISPLAYCHANGE => {
                self.on_display_change();
            }
            WM_DPICHANGED => {
                self.on_dpi_changed();
                return true;
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
                    return true;
                }
            }
            WM_SHOWWINDOW => {
                if wparam.0 == 0 {
                    if self.on_hide_window().is_ok() {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    if self.on_show_window().is_ok() {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
            WM_PAINT => {
                self.render();
                return true;
            }
            WM_SIZE => {
                self.on_resize();
            }
            WM_WINDOWPOSCHANGING => {
                self.on_window_pos_changing();
            }
            _ => (),
        };

        false
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

    fn show(&self, pt: Point<i32>) -> Result<()>;

    fn on_show_window(&self) -> Result<()>;

    fn on_hide_window(&self) -> Result<()>;

    // fn show(&mut self, pt: Point<i32>) -> Result<()> {
    //     let mut window = (*self.window_data()).clone();
    //     let handle = window.handle.unwrap();

    //     let dpi = window.dpi;

    //     window.origin = if dpi_aware() {
    //         pt
    //     } else {
    //         Point {
    //             x: pt.x.to_dp(dpi) as i32,
    //             y: pt.y.to_dp(dpi) as i32,
    //         }
    //     };
    //     window.showing = true;
    //     window.tracking_mouse = true;

    //     if handle != HWND(0) {
    //         unsafe {
    //             ShowWindow(handle, SW_SHOWNA);
    //         }
    //         self.set_window_data(window)
    //     } else {
    //         winerr!(E_FAIL)
    //     }
    // }

    fn hide(&self) -> Result<()> {
        // let window = self.window_data();
        // if !window.showing {
        //     return Ok(());
        // }

        // let mut window = (*window).clone();
        // let handle = window.handle.unwrap();
        // let tracking = window.tracking_mouse;
        let handle = self.handle()?;

        unsafe {
            ShowWindow(handle, SW_HIDE);
        }
        Ok(())
        // window.showing = false;

        // if tracking {
        //     unsafe {
        //         ReleaseCapture();
        //     }
        //     window.tracking_mouse = false;
        // }

        // self.set_window_data(window)
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
