use std::cell::Cell;

use windows::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;
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
use windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_KEYUP;

use crate::tip::key_event::KeyEvent;
use crate::tip::text_service::TextService;

use super::engine_mgr;
use super::key_handlers::handle_key;

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink<'a> {
    service: &'a TextService,
    shift_pressed: Cell<bool>,
}

impl<'a> KeyEventSink<'a> {
    fn new(service: &'a TextService) -> Self {
        KeyEventSink {
            shift_pressed: Cell::new(false),
            service,
        }
    }

    pub fn advise(
        service: &TextService,
        threadmgr: ITfThreadMgr,
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
        threadmgr: ITfThreadMgr,
    ) -> Result<()> {
        let keystroke_mgr: ITfKeystrokeMgr = threadmgr.cast()?;

        unsafe {
            keystroke_mgr.UnadviseKeyEventSink(service.clientid())?;
        }

        Ok(())
    }

    fn test_key_down(
        &self,
        _context: &ITfContext,
        key_event: &KeyEvent,
    ) -> Result<BOOL> {
        if !self.service.enabled() {
            return Ok(FALSE);
        }

        if key_event.keycode == VK_SHIFT.0 as u32 /* TODO: check config */ {
            self.shift_pressed.set(true);
            return Ok(TRUE);
        }

        // TODO: check for candidate UI priority keys

        match self.service.engine().on_test_key(&key_event) {
            true => Ok(TRUE),
            false => Ok(FALSE),
        }
    }

    fn key_down(
        &self,
        context: &ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        if !self.service.enabled() {
            return Ok(FALSE);
        }

        let test =  self.test_key_down(context, &key_event);

        if self.shift_pressed.get() {
            return Ok(FALSE);
        }

        match test {
            Ok(TRUE) => {
                self.shift_pressed.set(false);
                match handle_key(self.service, context, key_event) {
                    Ok(_) => Ok(TRUE),
                    Err(_) => Ok(FALSE)
                }
            },
            _ => Ok(FALSE)
        }
    }

    fn test_key_up(
        &self,
        _context: &ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        if self.shift_pressed.get() && key_event.keycode == VK_SHIFT.0 as u32 /* TODO: check config */ {
            self.shift_pressed.set(false);
            Ok(TRUE)
        } else{
            Ok(FALSE)
        }
    }

    fn key_up(
        &self,
        _context: &ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        if self.shift_pressed.get() && key_event.keycode == VK_SHIFT.0 as u32 /* TODO: check config */ {
            self.shift_pressed.set(false);
            Ok(TRUE)
        } else{
            Ok(FALSE)
        }
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
        match pic {
            Some(context) => {
                let key_event = KeyEvent::new(WM_KEYDOWN, wparam, lparam);
                self.test_key_down(context, &key_event)
            }
            None => Ok(FALSE),
        }
    }

    fn OnTestKeyUp(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        match pic {
            Some(context) => {
                let key_event = KeyEvent::new(WM_KEYUP, wparam, lparam);
                self.test_key_up(context, key_event)
            }
            None => Ok(FALSE),
        }
    }

    fn OnKeyDown(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        match pic {
            Some(context) => {
                let key_event = KeyEvent::new(WM_KEYDOWN, wparam, lparam);
                self.key_down(context, key_event)
            }
            None => Ok(FALSE),
        }
    }

    fn OnKeyUp(
        &self,
        pic: Option<&ITfContext>,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<BOOL> {
        match pic {
            Some(context) => {
                let key_event = KeyEvent::new(WM_KEYUP, wparam, lparam);
                self.key_up(context, key_event)
            }
            None => Ok(FALSE),
        }
    }

    fn OnPreservedKey(
        &self,
        pic: Option<&ITfContext>,
        rguid: *const GUID,
    ) -> Result<BOOL> {
        Ok(FALSE)
    }
}
