use std::cell::RefCell;
use std::cmp::max;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

use windows::Win32::Foundation::D2DERR_RECREATE_TARGET;
use windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F;
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS;
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE;
use windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE;
use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;
use windows::Win32::Graphics::Gdi::RDW_INVALIDATE;
use windows::Win32::Graphics::Gdi::RDW_UPDATENOW;
use windows::Win32::Graphics::Gdi::RedrawWindow;
use windows::core::AsImpl;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS;
use windows::Win32::Graphics::Gdi::BeginPaint;
use windows::Win32::Graphics::Gdi::EndPaint;
use windows::Win32::Graphics::Gdi::PAINTSTRUCT;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::ShowWindow;
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
use windows::Win32::UI::WindowsAndMessaging::SPI_GETWORKAREA;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNA;
use windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW;
use windows::Win32::UI::WindowsAndMessaging::WS_EX_TOPMOST;
use windows::Win32::UI::WindowsAndMessaging::WS_POPUP;

use khiin_protos::config::AppConfig;

use crate::dll::DllModule;
use crate::geometry::Point;
use crate::geometry::Rect;
use crate::locales::t;
use crate::resource::*;
use crate::ui::colors::color_f;
use crate::ui::colors::ColorScheme_F;
use crate::ui::colors::COLOR_BLACK;
use crate::ui::colors::COLOR_SCHEME_LIGHT;
use crate::ui::dpi::dpi_aware;
use crate::ui::dpi::Density;
use crate::ui::popup_menu_item::PopupMenuItem;
use crate::ui::window::WindowData;
use crate::ui::window::WindowHandler;
use crate::ui::wndproc::Wndproc;

static FONT_NAME: &str = "Arial";

const DW_STYLE: WINDOW_STYLE = WS_POPUP;

static BULLET_COL_WIDTH: i32 = 24;
static ICON_COL_WIDTH: i32 = 32;
static ROW_HEIGHT: i32 = 34;
static VPAD: i32 = 8;
static HPAD: i32 = 16;

fn window_ex_style() -> WINDOW_EX_STYLE {
    WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE
}

fn work_area_bottom() -> i32 {
    unsafe {
        let mut rect = RECT::default();
        let ptr = &mut rect as *mut _ as *mut c_void;
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(ptr),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );
        rect.bottom
    }
}

fn get_menu_items() -> Vec<PopupMenuItem> {
    let mut ret = Vec::new();

    ret.push(PopupMenuItem::button(
        "continuous",
        IDI_MODE_CONTINUOUS,
        true,
    ));
    ret.push(PopupMenuItem::button("basic", IDI_MODE_BASIC, false));
    ret.push(PopupMenuItem::button("manual", IDI_MODE_PRO, false));
    ret.push(PopupMenuItem::button("direct", IDI_MODE_ALPHA, false));
    ret.push(PopupMenuItem::sep());
    ret.push(PopupMenuItem::button("settings", IDI_SETTINGS, false));

    ret
}

fn to_owned<T>(rc: Rc<RefCell<T>>) -> Result<T>
where
    T: Clone,
{
    Ok(rc.try_borrow().map_err(|_| Error::from(E_FAIL))?.clone())
}

pub struct PopupMenu {
    tip: ITfTextInputProcessor,
    brush: RefCell<Option<ID2D1SolidColorBrush>>,
    textformat: RefCell<Option<IDWriteTextFormat>>,
    window: Rc<RefCell<WindowData>>,
    colors: RefCell<ColorScheme_F>,
    items: Rc<RefCell<Vec<PopupMenuItem>>>,
    highlighted_index: RefCell<usize>,
}

impl PopupMenu {
    pub fn new(tip: ITfTextInputProcessor) -> Result<Arc<Self>> {
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

        let this = Arc::new(Self {
            window: Rc::new(RefCell::new(window)),
            tip,
            brush: RefCell::new(None),
            textformat: RefCell::new(None),
            colors: RefCell::new(COLOR_SCHEME_LIGHT.into()),
            items: Rc::new(RefCell::new(get_menu_items())),
            highlighted_index: RefCell::new(usize::MAX),
        });

        Wndproc::create(
            this.clone(),
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

    fn calculate_layout(&self) -> Result<(i32, i32, i32, i32)> {
        let mut max_item_width = 0i32;
        let mut max_row_height = 0i32;

        let mut items = self
            .items
            .try_borrow_mut()
            .map_err(|_| Error::from(E_FAIL))?;

        let window =
            self.window.try_borrow().map_err(|_| Error::from(E_FAIL))?;

        for item in items.iter_mut() {
            if item.separator {
                continue;
            }
            let text = t(&item.string_key);
            unsafe {
                let layout = window.factory.create_text_layout(
                    text,
                    self.textformat.borrow().clone().unwrap(),
                    window.max_width as f32,
                    window.max_height as f32,
                )?;

                let mut metrics = DWRITE_TEXT_METRICS::default();
                layout.GetMetrics(&mut metrics)?;
                max_item_width = max(max_item_width, metrics.width as i32);
                max_row_height = max(max_row_height, metrics.height as i32);
                item.layout = Some(layout);
            }
        }

        let width = max_item_width + BULLET_COL_WIDTH + ICON_COL_WIDTH;
        let row_height = max(ROW_HEIGHT, max_row_height + VPAD);
        let total_width = width + HPAD * 2;
        let mut total_height = VPAD;

        for item in items.iter_mut() {
            let item_height = if item.separator { VPAD } else { row_height };
            item.rect = Rect {
                origin: Point {
                    x: 0,
                    y: total_height,
                },
                width: total_width,
                height: item_height,
            };
            total_height += item_height;
        }

        total_height += VPAD;

        let mut w = total_width;
        let mut h = total_height;
        let mut x = window.origin.x;
        let mut y = work_area_bottom() - VPAD;

        if dpi_aware() {
            w = w.to_px(window.dpi);
            h = h.to_px(window.dpi);
            y = work_area_bottom() - VPAD.to_px(window.dpi);
        }

        x = x - w / 2;
        y = y - h;

        let max_width = window.max_width;
        if x + w > max_width {
            x -= x + w - max_width;
        }

        Ok((x, y, w, h))
    }

    fn reset_graphics_resources(&self) -> Result<()> {
        if let Ok(mut window) = self.window.try_borrow_mut() {
            let target = window.factory.create_dc_render_target()?;
            window.target = target.clone();
            let color = color_f(&COLOR_BLACK);
            let brush = unsafe { target.CreateSolidColorBrush(&color, None)? };
            self.brush.replace(Some(brush));
            let textformat = window.factory.create_text_format(FONT_NAME, 16.0)?;
            self.textformat.replace(Some(textformat));
        }

        Ok(())
    }
}

impl Wndproc<PopupMenu> for PopupMenu {}
impl WindowHandler for PopupMenu {
    const WINDOW_CLASS_NAME: &'static str = "PopupMenuWindow";

    fn window_data(&self) -> Rc<RefCell<WindowData>> {
        self.window.clone()
    }

    fn set_handle(&self, handle: Option<HWND>) -> Result<()> {
        if let Ok(mut window) = self.window.try_borrow_mut() {
            window.handle = handle;
        }
        Ok(())
    }

    fn show(&self, pt: Point<i32>) -> Result<()> {
        self.reset_graphics_resources()?;
        self.set_origin(pt)?;
        let (x, y, w, h) = self.calculate_layout()?;
        let handle = self.handle()?;
        unsafe {
            SetWindowPos(
                handle,
                HWND::default(),
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                SWP_NOACTIVATE | SWP_NOZORDER,
            );
            ShowWindow(handle, SW_SHOWNA);
            // RedrawWindow(handle, None, None, RDW_INVALIDATE | RDW_UPDATENOW);
        }

        Ok(())
    }

    fn on_hide_window(&self) -> Result<()> {
        let mut window = self
            .window
            .try_borrow_mut()
            .map_err(|_| Error::from(E_FAIL))?;
        window.showing = false;
        window.tracking_mouse = false;
        Ok(())
    }

    fn on_resize(&self, width: u16, height: u16) -> Result<()> {
        self.reset_graphics_resources()
    }

    fn render(&self, handle: HWND) -> Result<()> {
        self.reset_graphics_resources()?;
        let window = to_owned(self.window.clone())?;
        let target = window.target;
        let mut ps = PAINTSTRUCT::default();
        let mut rc = RECT::default();

        unsafe {
            GetClientRect(handle, &mut rc);
            BeginPaint(handle, &mut ps);
            target.BindDC(ps.hdc, &rc)?;
            target.BeginDraw();

            draw(
                target.clone(),
                self.brush.borrow().clone().unwrap(),
                self.colors.borrow().clone(),
                to_owned(self.items.clone())?,
                self.highlighted_index.borrow().clone(),
            );

            match target.EndDraw(None, None) {
                Ok(_) => {},
                Err(e) => {
                    if e.code() == D2DERR_RECREATE_TARGET {
                        self.reset_graphics_resources()?;
                    }
                }
            }

            EndPaint(handle, &ps);
            Ok(())
        }
    }
}

unsafe fn draw_text_item(
    target: &ID2D1DCRenderTarget,
    brush: &ID2D1SolidColorBrush,
    item: &PopupMenuItem,
) {
    let layout = item.layout.clone().unwrap();
    let mut origin = item.rect.origin;
    origin.x += HPAD;
    let height = item.rect.height;

    let mut o = pt(origin.x as f32, origin.y as f32);
    o.x += BULLET_COL_WIDTH as f32 + ICON_COL_WIDTH as f32;
    o.y += vertical_center_text_layout(&layout, height);
    target.DrawTextLayout(o, &layout, brush, D2D1_DRAW_TEXT_OPTIONS_NONE);
}

unsafe fn draw(
    target: ID2D1DCRenderTarget,
    brush: ID2D1SolidColorBrush,
    colors: ColorScheme_F,
    items: Vec<PopupMenuItem>,
    highlight_idx: usize,
) {
    target.Clear(Some(&colors.background));

    for (i, item) in items.iter().enumerate() {
        let rect = item.d2d_rect_f();

        if !item.separator {
            if highlight_idx == i {
                brush.SetColor(&colors.background_selected);
                target.FillRectangle(&rect, &brush);
            }

            brush.SetColor(&colors.text);
            draw_text_item(&target, &brush, item);
        } else {
            let top = item.rect.top() as f32 + item.rect.height as f32 / 2.0;
            let width = item.rect.width as f32;
            target.DrawLine(pt(0.0, top), pt(width, top), &brush, 1.0, None);
        }
    }
}

fn pt(x: f32, y: f32) -> D2D_POINT_2F {
    D2D_POINT_2F { x, y }
}

unsafe fn vertical_center_text_layout(layout: &IDWriteTextLayout, available_height: i32) -> f32 {
    let mut metrics = DWRITE_TEXT_METRICS::default();
    layout.GetMetrics(&mut metrics).ok();
    (available_height as f32 - metrics.height) / 2.0
}
