use std::cell::RefCell;
use std::rc::Rc;

use windows::core::AsImpl;
use windows::core::Result;
use windows::Win32::Foundation::D2DERR_RECREATE_TARGET;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::Gdi::BeginPaint;
use windows::Win32::Graphics::Gdi::EndPaint;
use windows::Win32::Graphics::Gdi::PAINTSTRUCT;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use crate::ui::candidates::metrics::Metrics;
use crate::ui::window::WindowData;
use crate::ui::window::WindowHandler;
use crate::ui::wndproc::Wndproc;
use crate::utils::CloneInner;

static FONT_NAME: &str = "Arial";

#[derive(Default)]
pub enum DisplayMode {
    #[default]
    ShortColumn,
    LongColumn,
    Grid,
}

pub struct CandidateWindow {
    tip: ITfTextInputProcessor,
    window: Rc<RefCell<WindowData>>,
    brush: RefCell<ID2D1SolidColorBrush>,
    textformat: RefCell<IDWriteTextFormat>,
    textformat_sm: RefCell<IDWriteTextFormat>,
    metrics: RefCell<Metrics>,
    display_mode: RefCell<DisplayMode>,
    focused_id: RefCell<usize>,
    quickselect_col: RefCell<usize>,
    quickselect_active: RefCell<bool>,
    mouse_focused_id: RefCell<usize>,
    tracking_mouse: RefCell<bool>,
}

impl CandidateWindow {
    pub(crate) fn new(tip: ITfTextInputProcessor) -> Result<Self> {
        let service = tip.as_impl();
        let factory = service.render_factory.clone();
        let window = WindowData::new(factory)?;
        let color = D2D1_COLOR_F::default();
        let brush =
            unsafe { window.target.CreateSolidColorBrush(&color, None)? };
        let metrics = Metrics::new(0.0);
        let textformat = window
            .factory
            .create_text_format(FONT_NAME, metrics.font_size)?;
        let textformat_sm = window
            .factory
            .create_text_format(FONT_NAME, metrics.font_size_sm)?;

        Ok(Self {
            tip,
            window: Rc::new(RefCell::new(window)),
            brush: RefCell::new(brush),
            textformat: RefCell::new(textformat),
            textformat_sm: RefCell::new(textformat_sm),
            metrics: RefCell::new(metrics),
            display_mode: RefCell::new(DisplayMode::ShortColumn),
            focused_id: RefCell::new(usize::MAX),
            quickselect_col: RefCell::new(0),
            quickselect_active: RefCell::new(false),
            mouse_focused_id: RefCell::new(usize::MAX),
            tracking_mouse: RefCell::new(false),
        })
    }
}

impl Wndproc<CandidateWindow> for CandidateWindow {}
impl WindowHandler for CandidateWindow {
    const WINDOW_CLASS_NAME: &'static str = "CandidateWindow";

    fn window_data(&self) -> Rc<RefCell<WindowData>> {
        self.window.clone()
    }

    fn set_handle(&self, handle: Option<HWND>) -> Result<()> {
        if let Ok(mut window) = self.window.try_borrow_mut() {
            window.handle = handle;
        }
        Ok(())
    }

    fn render(&self, handle: HWND) -> Result<()> {
        let window = self.window.try_clone_inner()?;
        let factory = window.factory;
        let target = window.target;
        let mut ps = PAINTSTRUCT::default();
        let mut rc = RECT::default();

        unsafe {
            GetClientRect(handle, &mut rc);
            BeginPaint(handle, &mut ps);
            target.BindDC(ps.hdc, &rc)?;
            target.BeginDraw();

            // draw(
            // factory,
            // target.clone(),
            // (*self.brush.borrow()).clone(),
            // (*self.colors.borrow()).clone(),
            // try_clone_inner(self.items)?;
            // (*self.highlighted_index.borrow()).clone(),
            // );

            match target.EndDraw(None, None) {
                Ok(_) => {}
                Err(e) => {
                    if e.code() == D2DERR_RECREATE_TARGET {
                        self.reset_render_target()?;
                    }
                }
            }

            EndPaint(handle, &ps);
            Ok(())
        }
    }
}
