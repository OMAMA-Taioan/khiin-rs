use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use khiin_protos::config::AppConfig;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::Interface;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::BSTR;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::E_INVALIDARG;
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::Foundation::POINT;
use windows::Win32::Foundation::RECT;
use windows::Win32::System::Ole::CONNECT_E_CANNOTCONNECT;
use windows::Win32::System::Ole::CONNECT_E_NOCONNECTION;
use windows::Win32::UI::TextServices::ITfLangBarItem;
use windows::Win32::UI::TextServices::ITfLangBarItemButton;
use windows::Win32::UI::TextServices::ITfLangBarItemButton_Impl;
use windows::Win32::UI::TextServices::ITfLangBarItemMgr;
use windows::Win32::UI::TextServices::ITfLangBarItemSink;
use windows::Win32::UI::TextServices::ITfLangBarItem_Impl;
use windows::Win32::UI::TextServices::ITfMenu;
use windows::Win32::UI::TextServices::ITfSource;
use windows::Win32::UI::TextServices::ITfSource_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::TextServices::TfLBIClick;
use windows::Win32::UI::TextServices::GUID_LBI_INPUTMODE;
use windows::Win32::UI::TextServices::TF_LANGBARITEMINFO;
use windows::Win32::UI::TextServices::TF_LBI_CLK_LEFT;
use windows::Win32::UI::TextServices::TF_LBI_CLK_RIGHT;
use windows::Win32::UI::TextServices::TF_LBI_ICON;
use windows::Win32::UI::TextServices::TF_LBI_STYLE_BTN_BUTTON;
use windows::core::w;
use windows::Win32::Foundation::COLORREF;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Graphics::Gdi::CreateBitmap;
use windows::Win32::Graphics::Gdi::CreateCompatibleDC;
use windows::Win32::Graphics::Gdi::CreateDIBSection;
use windows::Win32::Graphics::Gdi::CreateFontW;
use windows::Win32::Graphics::Gdi::DeleteDC;
use windows::Win32::Graphics::Gdi::DeleteObject;
use windows::Win32::Graphics::Gdi::DrawTextW;
use windows::Win32::Graphics::Gdi::GdiFlush;
use windows::Win32::Graphics::Gdi::SelectObject;
use windows::Win32::Graphics::Gdi::SetBkMode;
use windows::Win32::Graphics::Gdi::SetTextColor;
use windows::Win32::Graphics::Gdi::ANTIALIASED_QUALITY;
use windows::Win32::Graphics::Gdi::BITMAPINFO;
use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;
use windows::Win32::Graphics::Gdi::BI_RGB;
use windows::Win32::Graphics::Gdi::DEFAULT_CHARSET;
use windows::Win32::Graphics::Gdi::DIB_RGB_COLORS;
use windows::Win32::Graphics::Gdi::DT_CENTER;
use windows::Win32::Graphics::Gdi::DT_SINGLELINE;
use windows::Win32::Graphics::Gdi::DT_VCENTER;
use windows::Win32::Graphics::Gdi::FW_BOLD;
use windows::Win32::Graphics::Gdi::HDC;
use windows::Win32::Graphics::Gdi::TRANSPARENT;
use windows::Win32::UI::WindowsAndMessaging::CreateIconIndirect;
use windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::ICONINFO;
use windows::Win32::UI::WindowsAndMessaging::SM_CXSMICON;

use crate::reg::guids::IID_KhiinTextService;
use crate::ui::systray::SystrayMenu;
use crate::ui::wndproc::Wndproc;
use crate::winerr;

static INFO: TF_LANGBARITEMINFO = TF_LANGBARITEMINFO {
    clsidService: IID_KhiinTextService,
    guidItem: GUID_LBI_INPUTMODE,
    dwStyle: TF_LBI_STYLE_BTN_BUTTON,
    ulSort: 0,
    szDescription: [0; 32],
};

#[implement(ITfSource, ITfLangBarItem, ITfLangBarItemButton)]
pub struct LangBarIndicator {
    tip: ITfTextInputProcessor,
    threadmgr: ITfThreadMgr,
    sink_map: Arc<Mutex<HashMap<u32, ITfLangBarItemSink>>>,
    status: u32,
    added: Cell<bool>,
    popup: Arc<SystrayMenu>,
}

impl LangBarIndicator {
    pub fn new(
        tip: ITfTextInputProcessor,
        threadmgr: ITfThreadMgr,
    ) -> Result<ITfLangBarItemButton> {
        let this = LangBarIndicator {
            tip: tip.clone(),
            threadmgr: threadmgr.clone(),
            sink_map: Arc::new(Mutex::new(HashMap::new())),
            status: 0, // always 0
            added: Cell::new(false),
            popup: SystrayMenu::new(tip)?,
        };

        let button: ITfLangBarItemButton = this.into();
        LangBarIndicator::add_item(threadmgr, button.clone())?;
        Ok(button)
    }

    pub fn shutdown(&self, button: ITfLangBarItemButton) -> Result<()> {
        self.popup.destroy()?;
        self.remove_item(button)
    }

    pub fn add_item(
        threadmgr: ITfThreadMgr,
        button: ITfLangBarItemButton,
    ) -> Result<()> {
        let indicator: &LangBarIndicator = unsafe { button.as_impl() };
        if indicator.added.get() {
            return Ok(());
        }

        let langbarmgr: ITfLangBarItemMgr = threadmgr.cast()?;
        unsafe { langbarmgr.AddItem(&button)? };
        indicator.added.set(true);
        Ok(())
    }

    pub fn remove_item(&self, button: ITfLangBarItemButton) -> Result<()> {
        let indicator: &LangBarIndicator = unsafe { button.as_impl() };

        if !indicator.added.get() {
            return Ok(());
        }

        let langbarmgr: ITfLangBarItemMgr = self.threadmgr.cast()?;
        unsafe { langbarmgr.RemoveItem(&button)? };
        indicator.added.set(false);
        Ok(())
    }

    pub fn on_config_change(
        &self,
        config: Arc<RwLock<AppConfig>>,
    ) -> Result<()> {
        if let Ok(map) = self.sink_map.lock() {
            for (_, sink) in map.iter() {
                unsafe {
                    sink.OnUpdate(TF_LBI_ICON)?;
                }
            }
        }

        self.popup.on_config_change(config)
    }

    /// Ask the language bar to re-query the icon (e.g. after the input mode
    /// changed) so `GetIcon` runs again and the "CT"/"CI" label updates.
    pub fn refresh_icon(&self) -> Result<()> {
        if let Ok(map) = self.sink_map.lock() {
            for (_, sink) in map.iter() {
                unsafe {
                    sink.OnUpdate(TF_LBI_ICON)?;
                }
            }
        }
        Ok(())
    }
}

impl ITfSource_Impl for LangBarIndicator {
    fn AdviseSink(
        &self,
        riid: *const GUID,
        punk: Option<&IUnknown>,
    ) -> Result<u32> {
        if unsafe { *riid } != ITfLangBarItemSink::IID {
            return winerr!(CONNECT_E_CANNOTCONNECT);
        }

        if punk.is_none() {
            return winerr!(E_INVALIDARG);
        }

        let sink: ITfLangBarItemSink = punk.unwrap().clone().cast()?;

        if let Ok(mut map) = self.sink_map.lock() {
            let cookie = map.keys().max().unwrap_or(&0) + 1;
            match map.insert(cookie, sink) {
                Some(_) => winerr!(CONNECT_E_CANNOTCONNECT),
                None => Ok(cookie),
            }
        } else {
            winerr!(CONNECT_E_CANNOTCONNECT)
        }
    }

    fn UnadviseSink(&self, dwcookie: u32) -> Result<()> {
        let mut map = self.sink_map.lock().unwrap();
        match map.remove(&dwcookie) {
            Some(_) => Ok(()),
            None => winerr!(CONNECT_E_NOCONNECTION),
        }
    }
}

impl ITfLangBarItem_Impl for LangBarIndicator {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> Result<()> {
        unsafe { *pinfo = INFO };
        Ok(())
    }

    fn GetStatus(&self) -> Result<u32> {
        Ok(self.status)
    }

    fn Show(&self, _fshow: BOOL) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn GetTooltipString(&self) -> Result<BSTR> {
        winerr!(E_NOTIMPL)
    }
}

impl ITfLangBarItemButton_Impl for LangBarIndicator {
    fn OnClick(
        &self,
        click: TfLBIClick,
        pt: &POINT,
        prcarea: *const RECT,
    ) -> Result<()> {
        match click {
            TF_LBI_CLK_LEFT => unsafe { self.tip.as_impl().toggle_enabled() },
            TF_LBI_CLK_RIGHT => unsafe { self.tip.as_impl().open_settings_app() },
            // TF_LBI_CLK_RIGHT => self.popup.show(pt.into()),
            _ => Ok(()),
        }
    }

    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn OnMenuSelect(&self, wid: u32) -> Result<()> {
        winerr!(E_NOTIMPL)
    }

    fn GetIcon(&self) -> Result<HICON> {
        // Draw the label for the current input mode: "由" while in manual
        // mode (自由/自動), otherwise "動" for classic mode (自動, the default).
        let text = {
            let service = unsafe { self.tip.as_impl() };
            if service.is_manual_mode() {
                "由"
            } else {
                "動"
            }
        };
        unsafe { render_mode_icon(text) }
    }

    fn GetText(&self) -> Result<BSTR> {
        winerr!(E_NOTIMPL)
    }
}

/// Render a short label (e.g. "CT" / "CI") into a small `HICON` for the
/// language bar. The icon is drawn at runtime with GDI, so no `.ico` assets
/// are needed and the label can follow the current input mode.
unsafe fn render_mode_icon(text: &str) -> Result<HICON> {
    // Draw a light, opaque badge with dark text (like other IME tray icons)
    // so the label stays legible regardless of the taskbar theme.
    const BG_R: u32 = 0xF5;
    const BG_G: u32 = 0xF5;
    const BG_B: u32 = 0xF5;
    const FG_R: u32 = 0x20;
    const FG_G: u32 = 0x20;
    const FG_B: u32 = 0x20;

    let size = {
        let cx = GetSystemMetrics(SM_CXSMICON);
        if cx <= 0 {
            16
        } else {
            cx
        }
    };

    let dc = CreateCompatibleDC(HDC::default());

    // Top-down 32bpp DIB section so we can post-process the pixels directly.
    let bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: size,
            biHeight: -size,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
    let dib = CreateDIBSection(
        dc,
        &bmi,
        DIB_RGB_COLORS,
        &mut bits,
        HANDLE::default(),
        0,
    )?;
    let old_bmp = SelectObject(dc, dib);

    let font = CreateFontW(
        -(size * 3 / 4),
        0,
        0,
        0,
        FW_BOLD.0 as i32,
        0,
        0,
        0,
        DEFAULT_CHARSET.0 as u32,
        0,
        0,
        ANTIALIASED_QUALITY.0 as u32,
        0,
        w!("Microsoft JhengHei"),
    );
    let old_font = SelectObject(dc, font);

    // White, antialiased text on the transparent (black) background. GDI can't
    // write the alpha channel, so the grayscale coverage is recovered below.
    SetBkMode(dc, TRANSPARENT);
    SetTextColor(dc, COLORREF(0x00FF_FFFF));

    let mut wtext: Vec<u16> = text.encode_utf16().collect();
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: size,
        bottom: size,
    };
    DrawTextW(
        dc,
        &mut wtext,
        &mut rect,
        DT_CENTER | DT_VCENTER | DT_SINGLELINE,
    );

    // Flush batched GDI ops before reading the DIB memory.
    let _ = GdiFlush();

    // GDI can't write the alpha channel, so rebuild each pixel: use the white
    // text's grayscale coverage to blend the dark text over the light badge,
    // and force full opacity for the whole tile.
    let pixels = std::slice::from_raw_parts_mut(
        bits as *mut u32,
        (size * size) as usize,
    );
    for px in pixels.iter_mut() {
        let v = *px;
        let b = v & 0xFF;
        let g = (v >> 8) & 0xFF;
        let r = (v >> 16) & 0xFF;
        let cov = r.max(g).max(b);
        let inv = 255 - cov;
        let out_r = (BG_R * inv + FG_R * cov) / 255;
        let out_g = (BG_G * inv + FG_G * cov) / 255;
        let out_b = (BG_B * inv + FG_B * cov) / 255;
        *px = (0xFF << 24) | (out_r << 16) | (out_g << 8) | out_b;
    }

    // The badge is fully opaque; supply an all-zero AND mask.
    let mask = CreateBitmap(size, size, 1, 1, None);

    let icon_info = ICONINFO {
        fIcon: BOOL::from(true),
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: mask,
        hbmColor: dib,
    };
    let hicon = CreateIconIndirect(&icon_info);

    // CreateIconIndirect copied the bitmaps; release our GDI objects.
    SelectObject(dc, old_font);
    SelectObject(dc, old_bmp);
    let _ = DeleteObject(font);
    let _ = DeleteObject(dib);
    let _ = DeleteObject(mask);
    let _ = DeleteDC(dc);

    hicon
}
