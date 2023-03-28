use std::borrow::Borrow;

use windows::core::implement;
use windows::core::ComInterface;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::TRUE;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfKeyEventSink;
use windows::Win32::UI::TextServices::ITfKeyEventSink_Impl;
use windows::Win32::UI::TextServices::ITfKeystrokeMgr;
use windows::Win32::UI::TextServices::ITfThreadMgr;

use super::text_service::TextService;

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink<'a> {
    service: &'a TextService,
    shift_pressed: bool,
}

impl<'a> KeyEventSink<'a> {
    fn new(service: &'a TextService) -> Self {
        KeyEventSink {
            shift_pressed: false,
            service,
        }
    }

    pub fn advise(
        service: &TextService,
        threadmgr: &ITfThreadMgr,
    ) -> Result<()> {
        let sink: ITfKeyEventSink = KeyEventSink::new(service).into();
        let keystroke_mgr: ITfKeystrokeMgr = threadmgr.cast()?;

        unsafe {
            keystroke_mgr.AdviseKeyEventSink(
                service.clientid(),
                &sink,
                TRUE,
            )?;
        }

        Ok(())
    }

    pub fn unadvise(
        service: &TextService,
        threadmgr: &ITfThreadMgr,
    ) -> Result<()> {
        let keystroke_mgr: ITfKeystrokeMgr = threadmgr.cast()?;

        unsafe {
            keystroke_mgr.UnadviseKeyEventSink(service.clientid())?;
        }

        Ok(())
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink<'_> {
    fn OnSetFocus(&self, _fforeground: BOOL) -> Result<()> {
        Ok(())
    }

    fn OnTestKeyDown(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn OnTestKeyUp(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn OnKeyUp(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }

    fn OnPreservedKey(
        &self,
        pic: Option<&ITfContext>,
        rguid: *const GUID,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }
}
