use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

use windows::core::AsImpl;
use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;
use windows::Win32::UI::WindowsAndMessaging::WS_POPUP;

use khiin_protos::config::AppConfig;

use crate::dll::DllModule;
use crate::geometry::Point;
use crate::ui::colors::color;
use crate::ui::colors::AsD2D1_F;
use crate::ui::window::WindowData;
use crate::ui::wndproc::Wndproc;

use super::window::WindowHandler;

static FONT_NAME: &str = "Microsoft JhengHei UI Regular";
const DW_STYLE: WINDOW_STYLE = WS_POPUP;
fn window_ex_style() -> WINDOW_EX_STYLE {
    WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE
}

pub struct PopupMenu {
    tip: ITfTextInputProcessor,
    brush: ID2D1SolidColorBrush,
    textformat: IDWriteTextFormat,
    window: Rc<RefCell<WindowData>>,
}

impl PopupMenu {
    pub fn new(tip: ITfTextInputProcessor) -> Result<Self> {
        let service = tip.as_impl();
        let factory = service.render_factory.clone();
        let target = factory.create_dc_render_target()?;

        let window = WindowData {
            handle: None,
            factory,
            showing: false,
            tracking_mouse: false,
            max_width: 100,
            max_height: 100,
            dpi_parent: 96,
            dpi: 96,
            scale: 1.0,
            origin: Point::default(),
            target: target.clone(),
        };

        let color = Box::into_raw(Box::new(color(0).f()));
        let brush = unsafe { target.CreateSolidColorBrush(color, None)? };
        let textformat = window.factory.create_text_format(FONT_NAME, 16.0)?;

        let mut this = Self {
            window: Rc::new(RefCell::new(window)),
            tip,
            brush,
            textformat,
        };

        Wndproc::create(
            &mut this,
            DllModule::global().module,
            "",
            DW_STYLE.0,
            window_ex_style().0,
        )?;

        Ok(this)
    }

    pub fn on_config_change(
        &self,
        config: Arc<RwLock<AppConfig>>,
    ) -> Result<()> {
        Ok(())
    }
}

impl Wndproc<PopupMenu> for PopupMenu {}
impl WindowHandler for PopupMenu {
    const WINDOW_CLASS_NAME: &'static str = "PopupMenuWindow";

    fn window_data(&self) -> Rc<RefCell<WindowData>> {
        self.window.clone()
    }
}
