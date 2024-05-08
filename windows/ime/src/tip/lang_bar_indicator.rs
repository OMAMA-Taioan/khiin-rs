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
use windows::Win32::UI::WindowsAndMessaging::LoadImageW;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::IMAGE_ICON;
use windows::Win32::UI::WindowsAndMessaging::LR_DEFAULTCOLOR;

use crate::dll::DllModule;
use crate::reg::guids::IID_KhiinTextService;
use crate::resource::make_int_resource;
use crate::resource::IDI_MODE_CONTINUOUS;
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
            TF_LBI_CLK_RIGHT => self.popup.show(pt.into()),
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
        unsafe {
            let res = make_int_resource(IDI_MODE_CONTINUOUS);

            let handle = LoadImageW(
                DllModule::global().module,
                res,
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTCOLOR,
            )?;

            Ok(HICON(handle.0))
        }
    }

    fn GetText(&self) -> Result<BSTR> {
        winerr!(E_NOTIMPL)
    }
}
