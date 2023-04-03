use std::cell::RefCell;
use std::rc::Rc;
use windows::core::Result;
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
    pub origin: Point<i32>,
}

pub trait WindowHandler {
    const WINDOW_CLASS_NAME: &'static str;

    fn window_data(&self) -> Rc<RefCell<WindowData>>;

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

    fn set_hwnd(&mut self, hwnd: HWND) {
        let window = self.window_data();
        let mut window = window.borrow_mut();
        window.handle = hwnd;
    }

    fn hwnd(&self) -> HWND {
        self.window_data().borrow().handle
    }

    fn showing(&self) -> bool {
        self.window_data().borrow().showing
    }

    fn show(&mut self, pt: Point<i32>) {
        let dpi = self.window_data().borrow().dpi;
        let window = self.window_data();
        let mut window = window.borrow_mut();
        window.origin = if dpi_aware() {
            pt
        } else {
            Point {
                x: pt.x.to_dp(dpi),
                y: pt.y.to_dp(dpi),
            }
        };

        unsafe {
            ShowWindow(self.hwnd(), SW_SHOWNA);
        }

        window.showing = true;
        window.tracking_mouse = true;
    }

    fn hide(&mut self) {
        let window = self.window_data();
        let mut window = window.borrow_mut();
        if !window.showing {
            return;
        }
        unsafe {
            ShowWindow(self.hwnd(), SW_HIDE);
        }
        if window.tracking_mouse {
            unsafe {
                ReleaseCapture();
            }
            window.tracking_mouse = false;
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

    fn render(&self) {
        // TODO
        return;
    }

    fn on_resize(&self) {
        // TODO
        return;
    }

    fn on_window_pos_changing(&self) {
        // TODO
        return;
    }
}
