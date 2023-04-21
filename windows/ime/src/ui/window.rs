use std::cell::RefCell;
use std::mem::transmute;
use std::rc::Rc;
use std::sync::Arc;

use log::debug as d;
use windows::core::Result;
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::RECT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Dwm::DWMWCP_ROUND;
use windows::Win32::UI::Controls::WM_MOUSELEAVE;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::WM_DPICHANGED;
use windows::Win32::UI::WindowsAndMessaging::WM_LBUTTONDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_MOUSEMOVE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
use windows::Win32::UI::WindowsAndMessaging::WM_PAINT;
use windows::Win32::UI::WindowsAndMessaging::WM_SHOWWINDOW;

use crate::fail;
use crate::geometry::Point;
use crate::geometry::Rect;
use crate::ui::dwm::set_rounded_corners;
use crate::ui::render_factory::RenderFactory;
use crate::utils::hi_word;
use crate::winerr;

#[derive(Clone)]
pub struct WindowData {
    pub factory: Arc<RenderFactory>,
    pub target: ID2D1DCRenderTarget,
    pub handle: Option<HWND>,
    pub showing: bool,
    pub tracking_mouse: bool,
}

impl WindowData {
    pub fn new(factory: Arc<RenderFactory>) -> Result<Self> {
        Ok(Self {
            handle: None,
            showing: false,
            tracking_mouse: false,
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
            WM_DPICHANGED => {
                let dpi = hi_word(wparam.0 as u32);
                let rect: &RECT = unsafe { transmute(lparam) };
                log::debug!("Dpi changed to: {}", dpi);
                self.on_dpi_changed(handle, dpi, rect.into())
            },
            WM_MOUSEMOVE => self.on_mouse_move(handle, lparam.into()),
            WM_MOUSELEAVE => self.on_mouse_leave(handle),
            WM_LBUTTONDOWN => self.on_click(handle, lparam.into()),
            WM_SHOWWINDOW => match wparam.0 {
                0 => self.on_hide_window(),
                _ => self.on_show_window(),
            },
            WM_PAINT => self.render(handle),
            _ => Err(fail!()),
        }
    }

    fn reset_render_target(&self) -> Result<()> {
        if let Ok(mut window) = self.window_data().try_borrow_mut() {
            let target = window.factory.create_dc_render_target()?;
            window.target = target.clone();
        }

        Ok(())
    }

    fn on_dpi_changed(
        &self,
        handle: HWND,
        dpi: u16,
        new_size: Rect<i32>,
    ) -> Result<()> {
        unsafe {
            self.window_data()
                .try_borrow()
                .map_err(|_| fail!())?
                .target
                .SetDpi(dpi as f32, dpi as f32);
        }
        Ok(())
    }

    fn handle(&self) -> Result<HWND> {
        self.window_data()
            .try_borrow()
            .map_err(|_| fail!())?
            .handle
            .ok_or(fail!())
    }

    // Optional
    fn on_show_window(&self) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    // Optional
    fn on_hide_window(&self) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    // Optional
    fn on_mouse_leave(&self, handle: HWND) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn set_handle(&self, handle: Option<HWND>) -> Result<()>;

    fn on_mouse_move(&self, handle: HWND, pt: Point<i32>) -> Result<()>;

    fn on_click(&self, handle: HWND, pt: Point<i32>) -> Result<()>;

    fn render(&self, handle: HWND) -> Result<()>;
}
