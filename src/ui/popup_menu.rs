use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;
use windows::Win32::UI::WindowsAndMessaging::WS_POPUP;

use crate::geometry::point::Point;
use crate::utils::arc_lock::ArcLock;

use super::dpi::dpi_aware;
use super::dpi::Density;
use super::render_factory::RenderFactory;
use super::window::BaseWindow;
use super::window::GuiWindow;

static WINDOW_CLASS_NAME: &str = "LanguageIndicatorPopupMenu";
static FONT_NAME: &str = "Microsoft JhengHei UI Regular";
const DW_STYLE: WINDOW_STYLE = WS_POPUP;
fn window_ex_style() -> WINDOW_EX_STYLE {
    WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE
}

pub fn color_f(r: u32, g: u32, b: u32) -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: r as f32,
        g: g as f32,
        b: b as f32,
        a: 0.0,
    }
}

// These were previously in GuiWindow class
// in c++ version
pub struct TheGuiWindow {
    hwnd: ArcLock<HWND>,
    showing: bool,
    tracking_mouse: bool,
    max_width: u32,
    max_height: u32,
    dpi_parent: u32,
    dpi: u32,
    scale: f32,
    factory: RenderFactory,
    target: ID2D1DCRenderTarget,
}

// these were in PopupMenu class extending GuiWindow
pub struct PopupMenu {
    service: ITfTextInputProcessor,
    brush: ID2D1SolidColorBrush,
    textformat: IDWriteTextFormat,
    origin: Point<i32>,
    gui: TheGuiWindow,
    class_name: &'static str,
}

impl PopupMenu {
    pub fn new(service: ITfTextInputProcessor) -> Result<Self> {
        let factory = RenderFactory::new()?;
        let target = factory.create_dc_render_target()?;

        let gui = TheGuiWindow {
            hwnd: ArcLock::new(HWND::default()),
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

        let color = Box::into_raw(Box::new(color_f(0, 0, 0)));
        let brush = unsafe { target.CreateSolidColorBrush(color, None)? };

        let mut this = Self {
            service,
            brush,
            textformat: gui.factory.create_text_format(FONT_NAME, 16.0)?,
            origin: Point::default(),
            gui,
            class_name: WINDOW_CLASS_NAME,
        };

        BaseWindow::<PopupMenu>::create(
            &mut this,
            "",
            DW_STYLE.0,
            window_ex_style().0,
        );

        Ok(this)
    }
}

impl BaseWindow<PopupMenu> for PopupMenu {
    fn class_name(&self) -> &str {
        self.class_name
    }
}

impl GuiWindow for PopupMenu {
    fn set_hwnd(&self, hwnd: HWND) -> Result<()> {
        self.gui.hwnd.set(hwnd)?;
        Ok(())
    }

    fn hwnd(&self) -> HWND {
        self.gui.hwnd.get().unwrap()
    }

    fn showing(&self) -> bool {
        self.gui.showing
    }

    fn show(&mut self, pt: Point<i32>) {
        self.origin = if dpi_aware() {
            pt
        } else {
            Point {
                x: pt.x.to_dp(self.gui.dpi),
                y: pt.y.to_dp(self.gui.dpi),
            }
        };

        self.gui.showing = true;
        unsafe {
            ShowWindow(self.hwnd(), SW_SHOWNA);
        }
        self.gui.tracking_mouse = true;
    }

    fn hide(&mut self) {
        if !self.gui.showing {
            return;
        }
        unsafe {
            ShowWindow(self.hwnd(), SW_HIDE);
        }
        if self.gui.tracking_mouse {
            unsafe {
                ReleaseCapture();
            }
            self.gui.tracking_mouse = false;
        }
    }

    fn on_create(&self) -> Result<()> {
        Ok(())
    }

    fn on_display_change(&self) {
        // TODO
    }

    fn on_dpi_changed(&self) {
        // TODO
    }

    // fn on_mouse_activate(&self) {
    //     todo!()
    // }

    fn on_mouse_move(&self) {
        // TODO
    }

    fn on_mouse_leave(&self) {
        // TODO
    }

    fn on_click(&self) -> bool {
        true // TODO
    }

    fn render(&self) {
        // TODO
    }

    fn on_resize(&self) {
        // TODO
    }

    fn on_window_pos_changing(&self) {
        // TODO
    }
}
