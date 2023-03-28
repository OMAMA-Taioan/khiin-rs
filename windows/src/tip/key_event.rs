use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardState;
use windows::Win32::UI::Input::KeyboardAndMouse::ToAscii;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_CONTROL;

use crate::utils::hi_word;
use crate::utils::lo_byte;

pub struct KeyEvent {
    message: u32,
    keystate: [u8; 256],
    ascii: u8,
    keycode: u32,
}

impl KeyEvent {
    pub fn new(message: u32, wparam: WPARAM, lparam: LPARAM) -> Self {
        let mut event = KeyEvent {
            message,
            keystate: [0u8; 256],
            ascii: 0,
            keycode: 0,
        };

        unsafe {
            GetKeyboardState(&mut event.keystate);
        }

        let scancode = lo_byte(hi_word(lparam.0 as u32));

        let vk_ctrl_idx = VK_CONTROL.0 as usize;
        let vk_ctrl_tmp = event.keystate[vk_ctrl_idx];
        let mut char = [0u16; 2];
        event.keystate[vk_ctrl_idx] = 0;

        let result = unsafe {
            ToAscii(
                wparam.0 as u32,
                scancode as u32,
                Some(&event.keystate),
                char.as_mut_ptr(),
                0,
            )
        };

        event.keystate[vk_ctrl_idx] = vk_ctrl_tmp;

        if result == 1 {
            event.ascii = char[0] as u8;
        }

        event
    }

    pub fn key_down(&self, vk_code: u32) -> bool {
        if let Some(val) = self.keystate.get(vk_code as usize) {
            return (val & 0x80) != 0;
        }

        return false;
    }
}
