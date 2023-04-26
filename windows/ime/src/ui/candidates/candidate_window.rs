use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use log::debug as d;
use once_cell::sync::Lazy;
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
use windows::Win32::Graphics::Gdi::RedrawWindow;
use windows::Win32::Graphics::Gdi::PAINTSTRUCT;
use windows::Win32::Graphics::Gdi::RDW_INVALIDATE;
use windows::Win32::Graphics::Gdi::RDW_UPDATENOW;
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::Input::KeyboardAndMouse::SetCapture;
use windows::Win32::UI::Input::KeyboardAndMouse::TrackMouseEvent;
use windows::Win32::UI::Input::KeyboardAndMouse::TME_LEAVE;
use windows::Win32::UI::Input::KeyboardAndMouse::TRACKMOUSEEVENT;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER;
use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WS_BORDER;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;
use windows::Win32::UI::WindowsAndMessaging::WS_POPUP;

use crate::dll::DllModule;
use crate::fail;
use crate::geometry::Point;
use crate::geometry::Rect;
use crate::geometry::Size;
use crate::ui::candidates::CandidateLayout;
use crate::ui::candidates::CandidatePage;
use crate::ui::candidates::CandidateRenderer;
use crate::ui::candidates::Metrics;
use crate::ui::colors::ColorScheme_F;
use crate::ui::colors::COLOR_SCHEME_LIGHT;
use crate::ui::dpi::dpi_aware;
use crate::ui::dpi::Density;
use crate::ui::window::WindowData;
use crate::ui::window::WindowHandler;
use crate::ui::wndproc::Wndproc;
use crate::utils::CloneInner;
use crate::utils::Hwnd;

static FONT_NAME: &str = "Arial";
const DW_STYLE: Lazy<WINDOW_STYLE> = Lazy::new(|| WS_BORDER | WS_POPUP);
const DW_EX_STYLE: Lazy<WINDOW_EX_STYLE> =
    Lazy::new(|| WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE);

#[derive(Default, Clone)]
pub enum DisplayMode {
    #[default]
    ShortColumn,
    LongColumn,
    Grid,
}

pub struct WindowPosInfo {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

pub struct CandidateWindow {
    tip: ITfTextInputProcessor,
    window: Rc<RefCell<WindowData>>,
    brush: RefCell<ID2D1SolidColorBrush>,
    textformat: RefCell<IDWriteTextFormat>,
    textformat_sm: RefCell<IDWriteTextFormat>,
    metrics: RefCell<Metrics>,
    mouse_focused_id: RefCell<Option<u32>>,
    tracking_mouse: RefCell<bool>,
    page_data: Rc<RefCell<CandidatePage>>,
    text_rect: RefCell<Rect<i32>>,
    layout: RefCell<CandidateLayout>,
    colors: RefCell<ColorScheme_F>,
}

impl CandidateWindow {
    pub fn new(tip: ITfTextInputProcessor) -> Result<Self> {
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
            mouse_focused_id: RefCell::new(None),
            tracking_mouse: RefCell::new(false),
            page_data: Rc::new(RefCell::new(CandidatePage::default())),
            text_rect: RefCell::new(Rect::default()),
            layout: RefCell::new(CandidateLayout::default()),
            colors: RefCell::new(COLOR_SCHEME_LIGHT.into()),
        })
    }

    pub fn create(this: Arc<Self>, parent: HWND) -> Result<()> {
        Wndproc::create(
            this,
            DllModule::global().module,
            parent,
            "",
            DW_STYLE.0,
            DW_EX_STYLE.0,
        )?;
        Ok(())
    }

    pub fn is_showing(&self) -> Result<bool> {
        Ok(self.window.try_borrow().map_err(|_| fail!())?.showing)
    }

    pub fn show(
        &self,
        page: CandidatePage,
        text_rect: Rect<i32>,
    ) -> Result<()> {
        let handle = self.handle()?;
        let pos = self.calculate_layout(page, text_rect, handle)?;

        unsafe {
            SetWindowPos(
                handle,
                HWND::default(),
                pos.left,
                pos.top,
                pos.width,
                pos.height,
                SWP_NOACTIVATE | SWP_NOZORDER,
            );
        }

        if !self.is_showing()? {
            unsafe {
                ShowWindow(handle, SW_SHOWNA);
                SetCapture(handle);
            }
            self.set_showing(true)?;
        } else {
            unsafe {
                RedrawWindow(
                    handle,
                    None,
                    None,
                    RDW_INVALIDATE | RDW_UPDATENOW,
                );
            }
        }

        Ok(())
    }

    pub fn hide(&self) -> Result<()> {
        d!("Hiding window");
        self.set_showing(false)?;
        let handle = self.handle()?;
        d!("Got handle");
        unsafe {
            ShowWindow(handle, SW_HIDE);
            d!("ShowWindow SW_HIDE");
            ReleaseCapture();
            d!("ReleaseCapture");
        }
        Ok(())
    }

    fn set_showing(&self, showing: bool) -> Result<()> {
        let mut window = self.window.try_borrow_mut().map_err(|_| fail!())?;
        window.showing = showing;
        Ok(())
    }

    pub fn calculate_layout(
        &self,
        page: CandidatePage,
        text_rect: Rect<i32>,
        handle: HWND,
    ) -> Result<WindowPosInfo> {
        self.page_data.replace(page);
        self.text_rect.replace(text_rect);

        let max_size = handle.max_size();
        d!("Max size: {:?}", max_size);

        let padding = self.metrics.borrow().padding as i32;

        let layout = CandidateLayout::new(
            self.window.borrow().factory.clone(),
            self.textformat.borrow().clone(),
            &(*self.page_data.borrow()).candidates,
            self.min_col_width(),
            padding,
            self.metrics.borrow().qs_col_w,
            max_size,
        )?;

        self.set_row_height(layout.grid.row_height() as f32)?;
        let Size { mut w, mut h } = layout.grid.grid_size();
        let mut qs_col_w = self.metrics.borrow().qs_col_w;
        let mut left = text_rect.left();
        let mut top = text_rect.bottom();

        if dpi_aware() {
            let dpi = handle.dpi();
            log::debug!("Window dpi in calculatelayout: {}", dpi);
            w = w.to_px(dpi);
            h = h.to_px(dpi);
            qs_col_w = qs_col_w.to_px(dpi);
        }

        left = left - qs_col_w;

        if left + w > max_size.w {
            d!("Window too far to the right");
            left = max_size.w - w;
        }
        if top + h > max_size.h {
            d!("Window off screen to the bottom");
            top = text_rect.top() - h;
        }
        if left < 0 {
            d!("Window off screen to the left");
            left = padding;
        }
        if top < 0 {
            d!("Window off screen to the top");
            top = padding;
        }

        d!("Window pos to (x: {}, y: {}, w: {}, h: {}", left, top, w, h);

        self.layout.replace(layout);

        Ok(WindowPosInfo {
            left,
            top,
            width: w,
            height: h,
        })
    }

    fn min_col_width(&self) -> i32 {
        match self.page_data.borrow().display_mode {
            DisplayMode::Grid => self.metrics.borrow().min_col_w_multi,
            _ => self.metrics.borrow().min_col_w_single,
        }
    }

    fn set_row_height(&self, row_height: f32) -> Result<()> {
        self.metrics
            .try_borrow_mut()
            .map_err(|_| fail!())?
            .row_height = row_height;

        Ok(())
    }

    fn mouse_focused_id(&self) -> Option<u32> {
        *self.mouse_focused_id.borrow()
    }

    fn set_mouse_focused_id(&self, id: Option<u32>) {
        self.mouse_focused_id.replace(id);
    }

    fn redraw(&self, handle: HWND) {
        let flags = RDW_INVALIDATE | RDW_UPDATENOW;
        unsafe {
            RedrawWindow(handle, None, None, flags);
        }
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

    fn on_click(&self, handle: HWND, pt: Point<i32>) -> Result<()> {
        d!("Clicked at: {:?}", pt);

        if !handle.contains_pt(pt) {
            self.hide()?;
            self.set_showing(false)?;
            return Err(fail!());
        }

        Ok(())
    }

    fn on_mouse_move(&self, handle: HWND, pt: Point<i32>) -> Result<()> {
        if !*self.tracking_mouse.borrow() {
            self.tracking_mouse.replace(true);
            let mut tme = TRACKMOUSEEVENT {
                cbSize: std::mem::size_of::<TRACKMOUSEEVENT>() as u32,
                dwFlags: TME_LEAVE,
                hwndTrack: handle,
                dwHoverTime: 0,
            };
            unsafe {
                TrackMouseEvent(&mut tme);
            }
        }

        if !handle.contains_pt(pt) {
            self.mouse_focused_id.replace(None);
            return Ok(());
        }

        // let dpi = self.window.borrow().dpi;
        let dpi = handle.dpi();
        let x = pt.x.to_dip(dpi);
        let y = pt.y.to_dip(dpi);

        if let Some(cand) = self.layout.borrow().hit_test(Point { x, y }) {
            let mouse_focused = self.mouse_focused_id();

            if let Some(curr_id) = mouse_focused {
                if curr_id as i32 != cand.id {
                    self.set_mouse_focused_id(Some(cand.id as u32));
                }
            } else {
                self.set_mouse_focused_id(Some(cand.id as u32));
            }
        }
        self.redraw(handle);
        Ok(())
    }

    fn on_mouse_leave(&self, handle: HWND) -> Result<()> {
        crate::trace!();
        self.tracking_mouse.replace(false);
        self.mouse_focused_id.replace(None);
        self.redraw(handle);
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

            CandidateRenderer {
                factory: &factory,
                target: &target,
                textformat_sm: &*self.textformat_sm.borrow(),
                brush: &*self.brush.borrow(),
                colors: &*self.colors.borrow(),
                page_data: &*self.page_data.borrow(),
                metrics: &*self.metrics.borrow(),
                cand_layout: &*self.layout.borrow(),
                mouse_focused_id: self
                    .mouse_focused_id
                    .borrow()
                    .unwrap_or(u32::MAX),
            }
            .draw();

            match target.EndDraw(None, None) {
                Ok(_) => {},
                Err(e) => {
                    if e.code() == D2DERR_RECREATE_TARGET {
                        self.reset_render_target()?;
                    }
                },
            }

            EndPaint(handle, &ps);
            Ok(())
        }
    }
}
