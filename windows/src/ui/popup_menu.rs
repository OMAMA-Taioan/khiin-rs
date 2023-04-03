use std::cell::RefCell;
use std::cmp::max;
use std::ffi::c_void;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HRGN;
use windows::Win32::Graphics::Gdi::RDW_INVALIDATE;
use windows::Win32::Graphics::Gdi::RDW_UPDATENOW;
use windows::Win32::Graphics::Gdi::RedrawWindow;
use windows::core::AsImpl;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS;
use windows::Win32::Graphics::Gdi::BeginPaint;
use windows::Win32::Graphics::Gdi::EndPaint;
use windows::Win32::Graphics::Gdi::PAINTSTRUCT;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::Win32::UI::WindowsAndMessaging::SetWindowPos;
use windows::Win32::UI::WindowsAndMessaging::SystemParametersInfoW;
use windows::Win32::UI::WindowsAndMessaging::SPI_GETWORKAREA;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;
use windows::Win32::UI::WindowsAndMessaging::SWP_NOZORDER;
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
use crate::resource::*;
use crate::ui::colors::color;
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
use crate::winerr;

static FONT_NAME: &str = "Microsoft JhengHei UI Regular";

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
        IDS_CONTINUOUS_MODE,
        IDI_MODE_CONTINUOUS,
        true,
    ));
    ret.push(PopupMenuItem::button(IDS_BASIC_MODE, IDI_MODE_BASIC, false));
    ret.push(PopupMenuItem::button(IDS_MANUAL_MODE, IDI_MODE_PRO, false));
    ret.push(PopupMenuItem::button(
        IDS_DIRECT_MODE,
        IDI_MODE_ALPHA,
        false,
    ));
    ret.push(PopupMenuItem::sep());
    ret.push(PopupMenuItem::button(
        IDS_OPEN_SETTINGS,
        IDI_SETTINGS,
        false,
    ));

    ret
}

pub struct PopupMenu {
    tip: ITfTextInputProcessor,
    brush: ID2D1SolidColorBrush,
    textformat: IDWriteTextFormat,
    window: Rc<RefCell<WindowData>>,
    colors: RefCell<ColorScheme_F>,
    items: RefCell<Vec<PopupMenuItem>>,
    highlighted_index: RefCell<Option<usize>>,
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

        let color = color_f(&COLOR_BLACK);
        let brush = unsafe { target.CreateSolidColorBrush(&color, None)? };
        let textformat = window.factory.create_text_format(FONT_NAME, 16.0)?;

        let mut this = Self {
            window: Rc::new(RefCell::new(window)),
            tip,
            brush,
            textformat,
            colors: RefCell::new(COLOR_SCHEME_LIGHT.into()),
            items: RefCell::new(get_menu_items()),
            highlighted_index: RefCell::new(None),
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

    fn calculate_layout(&self) -> Result<()> {
        let mut max_item_width = 0i32;
        let mut max_row_height = 0i32;

        let items = self.items.try_borrow_mut();
        if items.is_err() {
            return winerr!(E_FAIL);
        }
        let mut items = items.unwrap();

        let window = self.window.try_borrow();
        if window.is_err() {
            return winerr!(E_FAIL);
        }
        let window = window.unwrap();

        if window.handle.is_none() {
            return winerr!(E_FAIL);
        }
        let handle = window.handle.unwrap();

        for item in items.iter_mut() {
            if item.separator {
                continue;
            }
            unsafe {
                let layout = self.window.borrow().factory.create_text_layout(
                    "Test",
                    self.textformat.clone(),
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
                o: Point {
                    x: 0,
                    y: total_height,
                },
                w: total_width,
                h: item_height,
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
            RedrawWindow(handle, None, HRGN::default(), RDW_INVALIDATE | RDW_UPDATENOW);
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

    fn show(&self, pt: Point<i32>) -> Result<()> {
        if let Ok(mut window) = self.window.try_borrow_mut() {
            let dpi = window.dpi;
            if !dpi_aware() {
                window.origin = Point{ x: pt.x.to_dp(dpi) as i32, y: pt.y.to_dp(dpi) as i32 };
            } else {
                window.origin = pt;
            }
        }
        self.calculate_layout()?;
        Ok(())
    }

    fn render(&self) -> Result<()> {
        if let Ok(window) = self.window_data().try_borrow() {
            if let Some(handle) = window.handle {
                unsafe {
                    let target = &window.target;
                    let mut ps = PAINTSTRUCT::default();
                    let mut rc = RECT::default();
                    GetClientRect(handle, &mut rc);
                    BeginPaint(handle, &mut ps);
                    target.BindDC(ps.hdc, &rc)?;
                    target.BeginDraw();

                    {
                        let items = self.items.borrow();
                        let colors = self.colors.borrow();
                        let highlighted =
                            self.highlighted_index.borrow().clone();
                        let brush = &self.brush;
                        target.Clear(Some(&colors.background));

                        for (i, item) in items.iter().enumerate() {
                            let rect = item.d2d_rect_f();

                            if !item.separator {
                                if let Some(hl) = highlighted {
                                    if hl == i {
                                        brush.SetColor(
                                            &colors.background_selected,
                                        );
                                        target.FillRectangle(&rect, brush);
                                    }
                                }

                                brush.SetColor(&colors.text);
                            } else {
                            }
                        }
                    }

                    window.target.EndDraw(None, None)?;
                    EndPaint(handle, &ps);
                    return Ok(());
                }
            }
        }

        winerr!(E_FAIL)
    }
}
