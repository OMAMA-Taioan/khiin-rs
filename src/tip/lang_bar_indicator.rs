use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::Error;
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
use windows::Win32::UI::TextServices::TF_LBI_STYLE_BTN_BUTTON;
use windows::Win32::UI::WindowsAndMessaging::HICON;

use crate::reg::guids::IID_KhiinTextService;
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
    service: ITfTextInputProcessor,
    threadmgr: ITfThreadMgr,
    sink_map: Arc<Mutex<HashMap<u32, ITfLangBarItemSink>>>,
    status: u32,
    added: Cell<bool>,
}

impl LangBarIndicator {
    pub fn new(
        service: ITfTextInputProcessor,
        threadmgr: ITfThreadMgr,
    ) -> Result<ITfLangBarItemButton> {
        let this = LangBarIndicator {
            service,
            threadmgr: threadmgr.clone(),
            sink_map: Arc::new(Mutex::new(HashMap::new())),
            status: 0, // always 0
            added: Cell::new(false),
        };
        let button: ITfLangBarItemButton = this.into();
        LangBarIndicator::add_item(threadmgr, button.clone())?;
        Ok(button)
    }

    fn lang_bar_item_mgr(&self) -> Result<ITfLangBarItemMgr> {
        Ok(self.threadmgr.cast()?)
    }

    pub fn add_item(
        threadmgr: ITfThreadMgr,
        button: ITfLangBarItemButton,
    ) -> Result<()> {
        let indicator: &LangBarIndicator = button.as_impl();
        if indicator.added.get() {
            return Ok(());
        }

        let langbarmgr: ITfLangBarItemMgr = threadmgr.cast()?;
        unsafe { langbarmgr.AddItem(&button)? };
        indicator.added.set(true);
        Ok(())
    }

    pub fn remove_item(
        threadmgr: ITfThreadMgr,
        button: ITfLangBarItemButton,
    ) -> Result<()> {
        let indicator: &LangBarIndicator = button.as_impl();

        if !indicator.added.get() {
            return Ok(());
        }

        let langbarmgr: ITfLangBarItemMgr = threadmgr.cast()?;
        unsafe { langbarmgr.RemoveItem(&button)? };
        indicator.added.set(false);
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
        let mut map = self.sink_map.lock().unwrap();
        let cookie = map.keys().max().unwrap_or(&0) + 1;
        match map.insert(cookie, sink) {
            Some(_) => winerr!(CONNECT_E_CANNOTCONNECT),
            None => Ok(cookie),
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
        todo!()
    }

    fn InitMenu(&self, pmenu: Option<&ITfMenu>) -> Result<()> {
        todo!()
    }

    fn OnMenuSelect(&self, wid: u32) -> Result<()> {
        todo!()
    }

    fn GetIcon(&self) -> Result<HICON> {
        todo!()
    }

    fn GetText(&self) -> Result<BSTR> {
        todo!()
    }
}
