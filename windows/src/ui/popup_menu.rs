use std::mem::transmute;

use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
use windows::Win32::UI::WindowsAndMessaging::GWLP_USERDATA;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;
use windows::Win32::UI::WindowsAndMessaging::WS_POPUP;

use crate::dll::DllModule;
use crate::geometry::Point;
use crate::ui::colors::color;
use crate::ui::colors::AsD2D1_F;
use crate::ui::dpi::dpi_aware;
use crate::ui::dpi::Density;
use crate::ui::render_factory::RenderFactory;
use crate::ui::window::Window;
use crate::ui::window::WindowData;

static FONT_NAME: &str = "Microsoft JhengHei UI Regular";
const DW_STYLE: WINDOW_STYLE = WS_POPUP;
fn window_ex_style() -> WINDOW_EX_STYLE {
    WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE
}

pub struct PopupMenu {
    service: ITfTextInputProcessor,
    brush: ID2D1SolidColorBrush,
    textformat: IDWriteTextFormat,
    origin: Point<i32>,
    window: WindowData,
}

impl PopupMenu {
    pub fn new(service: ITfTextInputProcessor) -> Result<Self> {
        let factory = RenderFactory::new()?;
        let target = factory.create_dc_render_target()?;

        let window = WindowData {
            handle: HWND(0),
            showing: false,
            tracking_mouse: false,
            max_width: 100,
            max_height: 100,
            dpi_parent: 96,
            dpi: 96,
            scale: 1.0,
            factory,
            target: target.clone(),
        };

        let color = Box::into_raw(Box::new(color(0).f()));
        let brush = unsafe { target.CreateSolidColorBrush(color, None)? };
        let textformat = window.factory.create_text_format(FONT_NAME, 16.0)?;

        let mut this = Self {
            window,
            service,
            brush,
            textformat,
            origin: Point::default(),
        };

        Window::create(
            &mut this,
            DllModule::global().module,
            "",
            DW_STYLE.0,
            window_ex_style().0,
        )?;

        Ok(this)
    }
}

impl Window for PopupMenu {
    const WINDOW_CLASS_NAME: &'static str = "PopupMenuWindow";

    fn set_hwnd(&mut self, hwnd: HWND) {
        self.window.handle = hwnd;
    }

    fn hwnd(&self) -> HWND {
        self.window.handle
    }

    fn showing(&self) -> bool {
        self.window.showing
    }

    fn show(&mut self, pt: Point<i32>) {
        self.origin = if dpi_aware() {
            pt
        } else {
            Point {
                x: pt.x.to_dp(self.window.dpi),
                y: pt.y.to_dp(self.window.dpi),
            }
        };

        self.window.showing = true;
        unsafe {
            ShowWindow(self.hwnd(), SW_SHOWNA);
        }
        self.window.tracking_mouse = true;
    }

    fn hide(&mut self) {
        if !self.window.showing {
            return;
        }
        unsafe {
            ShowWindow(self.hwnd(), SW_HIDE);
        }
        if self.window.tracking_mouse {
            unsafe {
                ReleaseCapture();
            }
            self.window.tracking_mouse = false;
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

    extern "system" fn wndproc(
        hwnd: HWND,
        umsg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if umsg == WM_NCCREATE {
                let lpcs: *mut CREATESTRUCTW = transmute(lparam);
                let this = (*lpcs).lpCreateParams as *mut Self;
                (*this).set_hwnd(hwnd);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, transmute(this));
            } else {
                let this = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Self;
                if !this.is_null() {
                    return (*this).on_message(umsg, wparam, lparam);
                }
            };

            DefWindowProcW(hwnd, umsg, wparam, lparam)
        }
    }
}
