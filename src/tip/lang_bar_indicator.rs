use windows::core::implement;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::BSTR;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::POINT;
use windows::Win32::Foundation::RECT;
use windows::Win32::UI::TextServices::ITfLangBarItem;
use windows::Win32::UI::TextServices::ITfLangBarItemButton;
use windows::Win32::UI::TextServices::ITfLangBarItemButton_Impl;
use windows::Win32::UI::TextServices::ITfLangBarItem_Impl;
use windows::Win32::UI::TextServices::ITfMenu;
use windows::Win32::UI::TextServices::ITfSource;
use windows::Win32::UI::TextServices::ITfSource_Impl;
use windows::Win32::UI::TextServices::TfLBIClick;
use windows::Win32::UI::TextServices::TF_LANGBARITEMINFO;
use windows::Win32::UI::WindowsAndMessaging::HICON;

#[implement(ITfSource, ITfLangBarItem, ITfLangBarItemButton)]
struct LangBarIndicator;

impl ITfSource_Impl for LangBarIndicator {
    fn AdviseSink(
        &self,
        riid: *const GUID,
        punk: Option<&IUnknown>,
    ) -> Result<u32> {
        todo!()
    }

    fn UnadviseSink(&self, dwcookie: u32) -> Result<()> {
        todo!()
    }
}

impl ITfLangBarItem_Impl for LangBarIndicator {
    fn GetInfo(&self, pinfo: *mut TF_LANGBARITEMINFO) -> Result<()> {
        todo!()
    }

    fn GetStatus(&self) -> Result<u32> {
        todo!()
    }

    fn Show(&self, fshow: BOOL) -> Result<()> {
        todo!()
    }

    fn GetTooltipString(&self) -> Result<BSTR> {
        todo!()
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
