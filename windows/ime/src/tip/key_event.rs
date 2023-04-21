use khiin_protos::command::SpecialKey;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardState;
use windows::Win32::UI::Input::KeyboardAndMouse::ToAscii;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_BACK;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;

use khiin_protos::command::KeyEvent as KhiinKeyEvent;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_DELETE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_DOWN;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_END;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_ESCAPE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_HOME;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_LEFT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_NEXT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_PRIOR;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RETURN;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RIGHT;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_TAB;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_UP;

use crate::utils::hi_word;
use crate::utils::lo_byte;

const VK_CTRL: usize = VK_CONTROL.0 as usize;

#[derive(Debug)]
pub struct KeyEvent {
    pub message: u32,
    pub ascii: u8,
    pub keycode: u32,
    keystate: [u8; 256],
}

impl KeyEvent {
    pub fn new(message: u32, wparam: WPARAM, lparam: LPARAM) -> Self {
        let mut event = KeyEvent {
            message,
            ascii: 0,
            keycode: wparam.0 as u32,
            keystate: [0u8; 256],
        };

        unsafe {
            GetKeyboardState(&mut event.keystate);
        }

        let scancode = lo_byte(hi_word(lparam.0 as u32));

        let vk_ctrl_tmp = event.keystate[VK_CTRL];
        let mut char = [0u16; 2];
        event.keystate[VK_CTRL] = 0;

        let result = unsafe {
            ToAscii(
                event.keycode,
                scancode as u32,
                Some(&event.keystate),
                char.as_mut_ptr(),
                0,
            )
        };

        if result == 1 {
            event.ascii = char[0] as u8;
        }

        event.keystate[VK_CTRL] = vk_ctrl_tmp;

        event
    }

    pub fn key_down(&self, vk_code: u32) -> bool {
        if let Some(val) = self.keystate.get(vk_code as usize) {
            return (val & 0x80) != 0;
        }

        return false;
    }

    pub fn to_khiin(&self) -> KhiinKeyEvent {
        let mut e = KhiinKeyEvent::new();

        if self.ascii > 0 {
            e.key_code = self.ascii as i32;
        }

        e.special_key = windows_to_khiin_special_key_code(self).into();

        log::debug!("Khiin key event: {:?}", e);

        e
    }

    pub fn as_virtual_key(&self) -> VIRTUAL_KEY {
        VIRTUAL_KEY(self.keycode as u16)
    }
}

fn windows_to_khiin_special_key_code(e: &KeyEvent) -> SpecialKey {
    let vk = e.as_virtual_key();

    match vk {
        _ if vk == VK_SPACE => SpecialKey::SK_SPACE,
        _ if vk == VK_RETURN => SpecialKey::SK_ENTER,
        _ if vk == VK_ESCAPE => SpecialKey::SK_ESC,
        _ if vk == VK_BACK => SpecialKey::SK_BACKSPACE,
        _ if vk == VK_TAB => SpecialKey::SK_TAB,
        _ if vk == VK_LEFT => SpecialKey::SK_LEFT,
        _ if vk == VK_UP => SpecialKey::SK_UP,
        _ if vk == VK_RIGHT => SpecialKey::SK_RIGHT,
        _ if vk == VK_DOWN => SpecialKey::SK_DOWN,
        _ if vk == VK_PRIOR => SpecialKey::SK_PGUP,
        _ if vk == VK_NEXT => SpecialKey::SK_PGDN,
        _ if vk == VK_HOME => SpecialKey::SK_HOME,
        _ if vk == VK_END => SpecialKey::SK_END,
        _ if vk == VK_DELETE => SpecialKey::SK_DEL,
        _ => SpecialKey::SK_NONE,
    }
}
