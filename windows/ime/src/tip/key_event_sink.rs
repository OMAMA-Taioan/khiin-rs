use std::cell::Cell;

use khiin_protos::command::SpecialKey;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::Interface;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::TRUE;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_BACK;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_DOWN;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_H;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_L;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_LEFT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_OEM_3;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RIGHT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SHIFT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_TAB;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_UP;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfKeyEventSink;
use windows::Win32::UI::TextServices::ITfKeyEventSink_Impl;
use windows::Win32::UI::TextServices::ITfKeystrokeMgr;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::WindowsAndMessaging::WM_KEYDOWN;
use windows::Win32::UI::WindowsAndMessaging::WM_KEYUP;

use khiin_protos::command::Command;
use khiin_protos::command::CommandType;
use khiin_protos::command::Request;

use crate::reg::guids::GUID_PRESERVED_KEY_FULL_WIDTH_SPACE;
use crate::reg::guids::GUID_PRESERVED_KEY_ON_OFF;
use crate::reg::guids::GUID_PRESERVED_KEY_SWITCH_MODE;
use crate::tip::KeyEvent;
use crate::tip::TextService;

const HANDLED_KEYS: &[VIRTUAL_KEY] = &[
    VK_SPACE, VK_BACK, VK_TAB, VK_RETURN, VK_DOWN, VK_UP, VK_RIGHT, VK_LEFT,
    VK_OEM_3,
];

fn is_handled_key(key: &KeyEvent) -> bool {
    let vk = key.as_virtual_key();

    key.ascii > 0 || HANDLED_KEYS.contains(&vk)
}

pub fn handle_key(
    tip: ITfTextInputProcessor,
    context: ITfContext,
    key_event: KeyEvent,
) -> Result<()> {
    let khi = key_event.to_khiin();
    let mut req = Request::new();
    req.id = rand::random::<u32>();
    req.type_ = CommandType::CMD_SEND_KEY.into();
    req.key_event = Some(khi).into();
    let mut cmd = Command::new();
    cmd.request = Some(req).into();

    unsafe { tip.as_impl().send_command(context, cmd) }
}

#[implement(ITfKeyEventSink)]
pub struct KeyEventSink {
    tip: ITfTextInputProcessor,
    threadmgr: ITfThreadMgr,
    shift_pressed: Cell<bool>,
    ctrl_pressed: Cell<bool>,
}

impl KeyEventSink {
    pub fn new(tip: ITfTextInputProcessor, threadmgr: ITfThreadMgr) -> Self {
        KeyEventSink {
            tip,
            threadmgr,
            shift_pressed: Cell::new(false),
            ctrl_pressed: Cell::new(false),
        }
    }

    pub fn advise(&self) -> Result<()> {
        let sink: ITfKeyEventSink =
            KeyEventSink::new(self.tip.clone(), self.threadmgr.clone()).into();
        let keystroke_mgr: ITfKeystrokeMgr = self.threadmgr.cast()?;
        let service: &TextService = unsafe { self.tip.as_impl() };

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
        let service: &TextService = unsafe { self.tip.as_impl() };

        unsafe {
            keystroke_mgr.UnadviseKeyEventSink(service.clientid()?)?;
        }

        Ok(())
    }

    fn test_key_down(
        &self,
        _context: ITfContext,
        key_event: &KeyEvent,
    ) -> Result<BOOL> {
        let service = unsafe { self.tip.as_impl() };

        if !service.enabled()? {
            return Ok(FALSE);
        }

        let composing = service.composing();
        let special_key =
            key_event.to_khiin().special_key.enum_value_or_default();

        log::debug!("Composing: {}", composing);
        log::debug!("Special key: {:?}", special_key);

        if !service.composing()
            && key_event.to_khiin().special_key.enum_value_or_default()
                != SpecialKey::SK_NONE
        {
            return Ok(FALSE);
        }

        // if key_event.keycode == VK_SHIFT.0 as u32
        // /* TODO: check config */
        // {
        //     self.shift_pressed.set(true);
        //     return Ok(TRUE);
        // }
        if key_event.keycode == VK_CONTROL.0 as u32 {
            self.ctrl_pressed.set(true);
            return Ok(TRUE);
        }

        // TODO: check for candidate UI priority keys

        // TODO: check other keys, etc.
        if is_handled_key(key_event) {
            Ok(TRUE)
        } else {
            Ok(FALSE)
        }
    }

    fn key_down(
        &self,
        context: ITfContext,
        key_event: KeyEvent,
    ) -> Result<BOOL> {
        let service = unsafe { self.tip.as_impl() };

        if !service.enabled()? {
            return Ok(FALSE);
        }

        let test = self.test_key_down(context.clone(), &key_event);

        if self.shift_pressed.get() {
            return Ok(FALSE);
        }

        if self.ctrl_pressed.get() {
            if key_event.keycode == VK_OEM_3.0 as u32 {
                log::debug!("toggle input mode");
                self.ctrl_pressed.set(false);
                service.toggle_input_mode(context);
                return Ok(TRUE);
            } else if key_event.keycode == VK_H.0 as u32 {
                log::debug!("change hanji first");
                self.ctrl_pressed.set(false);
                service.change_output_mode(context, true);
                return Ok(TRUE);
            } else if key_event.keycode == VK_L.0 as u32 {
                log::debug!("change lomaji first");
                self.ctrl_pressed.set(false);
                service.change_output_mode(context, false);
                return Ok(TRUE);
            }
        }

        match test {
            Ok(TRUE) => {
                log::debug!("Key event: {:?}", key_event);
                self.shift_pressed.set(false);
                match handle_key(self.tip.clone(), context, key_event) {
                    Ok(_) => Ok(TRUE),
                    Err(_) => Ok(FALSE),
                }
            },
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
        } else if self.ctrl_pressed.get()
            && key_event.keycode == VK_CONTROL.0 as u32
        {
            self.ctrl_pressed.set(false);
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
        } else if self.ctrl_pressed.get()
            && key_event.keycode == VK_CONTROL.0 as u32
        {
            self.ctrl_pressed.set(false);
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
                self.test_key_down(context.clone(), &key_event)
            },
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
            },
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
                self.key_down(context.clone(), key_event)
            },
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
            },
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
