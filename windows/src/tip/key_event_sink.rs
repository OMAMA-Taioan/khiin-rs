use std::cell::Cell;

use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::TRUE;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfKeyEventSink;
use windows::Win32::UI::TextServices::ITfKeyEventSink_Impl;
use windows::Win32::UI::TextServices::ITfKeystrokeMgr;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_KEYUP;

use crate::reg::guids::GUID_PRESERVED_KEY_FULL_WIDTH_SPACE;
use crate::reg::guids::GUID_PRESERVED_KEY_ON_OFF;
use crate::reg::guids::GUID_PRESERVED_KEY_SWITCH_MODE;
use crate::tip::key_event::KeyEvent;
use crate::tip::text_service::TextService;
use crate::winerr;

// use super::engine_mgr;
use super::key_handlers::handle_key;

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    service: ITfTextInputProcessor,
    threadmgr: ITfThreadMgr,
    shift_pressed: Cell<bool>,
}

impl KeyEventSink {
    pub fn new(
        service: ITfTextInputProcessor,
        threadmgr: ITfThreadMgr,
    ) -> Self {
        KeyEventSink {
            service,
            threadmgr,
            shift_pressed: Cell::new(false),
        }
    }

    pub fn advise(&self) -> Result<()> {
        let sink: ITfKeyEventSink =
            KeyEventSink::new(self.service.clone(), self.threadmgr.clone())
                .into();
        let keystroke_mgr: ITfKeystrokeMgr = self.threadmgr.cast()?;
        let service: &TextService = self.service.as_impl();

        unsafe {
            keystroke_mgr.AdviseKeyEventSink(
                service.clientid()?,
                &sink,
                TRUE,
            )?;
        }

        Ok(())
    }

    pub fn unadvise(&self) -> Result<()> {
        let keystroke_mgr: ITfKeystrokeMgr = self.threadmgr.cast()?;
        let service: &TextService = self.service.as_impl();

        unsafe {
            keystroke_mgr.UnadviseKeyEventSink(service.clientid()?)?;
        }

        Ok(())
    }

    fn test_key_down(
        &self,
        _context: &ITfContext,
        key_event: &KeyEvent,
    ) -> Result<BOOL> {
        let service = self.service.as_impl();

        if !service.enabled()? {
            return Ok(FALSE);
        }

        if key_event.keycode == VK_SHIFT.0 as u32
        /* TODO: check config */
        {
            self.shift_pressed.set(true);
            return Ok(TRUE);
        }

        // TODO: check for candidate UI priority keys

        if let Ok(engine) = service.engine().read() {
            match engine.on_test_key(&key_event) {
                true => Ok(TRUE),
                false => Ok(FALSE),
            }
        } else {
            Ok(FALSE)
        }
    }

    fn key_down(
        &self,
        context: &ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        let service = self.service.as_impl();

        if !service.enabled()? {
            return Ok(FALSE);
        }

        let test = self.test_key_down(context, &key_event);

        if self.shift_pressed.get() {
            return Ok(FALSE);
        }

        match test {
            Ok(TRUE) => {
                self.shift_pressed.set(false);
                match handle_key(service, context, key_event) {
                    Ok(_) => Ok(TRUE),
                    Err(_) => Ok(FALSE),
                }
            }
            _ => Ok(FALSE),
        }
    }

    fn test_key_up(
        &self,
        _context: &ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        if self.shift_pressed.get() && key_event.keycode == VK_SHIFT.0 as u32
        /* TODO: check config */
        {
            self.shift_pressed.set(false);
            Ok(TRUE)
        } else {
            Ok(FALSE)
        }
    }

    fn key_up(
        &self,
        _context: &ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        if self.shift_pressed.get() && key_event.keycode == VK_SHIFT.0 as u32
        /* TODO: check config */
        {
            self.shift_pressed.set(false);
            Ok(TRUE)
        } else {
            Ok(FALSE)
        }
    }
}

impl ITfKeyEventSink_Impl for KeyEventSink {
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
        let guid = unsafe { *rguid };

        match guid {
            GUID_PRESERVED_KEY_ON_OFF => Ok(TRUE),
            GUID_PRESERVED_KEY_SWITCH_MODE => Ok(TRUE),
            GUID_PRESERVED_KEY_FULL_WIDTH_SPACE => Ok(TRUE),
            _ => Ok(FALSE),
        }
    }
}
