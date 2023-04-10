use std::cell::RefCell;

use khiin_windows::utils::pcwstr::ToPcwstr;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetParent, SetWindowTextW};

use crate::locales::t;
use crate::propsheetpage::PageHandler;
use crate::resource::*;
use crate::windowsx::*;

#[derive(Default)]
pub struct StylePage {
    handle: RefCell<HWND>,
}

impl PageHandler for StylePage {
    fn handle(&self) -> HWND {
        *self.handle.borrow()
    }

    fn set_handle(&self, handle: HWND) {
        self.handle.replace(handle);
    }

    fn initialize(&self) -> isize {
        let hwnd = unsafe { GetParent(self.handle()) };
        let title = t(IDS_WINDOW_CAPTION).to_pcwstr();
        unsafe {
            SetWindowTextW(hwnd, *title);
        }

        let ctl = self.item(IDL_COLOR);
        Static_SetText(ctl, &t(IDL_COLOR));

        let ctl = self.item(IDC_COMBOBOX_THEME_COLOR);
        ComboBox_ResetContent(ctl);
        ComboBox_AddString(ctl, &t(IDS_LIGHT_THEME));
        ComboBox_AddString(ctl, &t(IDS_DARK_THEME));
        ComboBox_SetCurSel(ctl, 0);

        let ctl = self.item(IDC_DISPLAY_LANGUAGE);
        ComboBox_ResetContent(ctl);
        ComboBox_AddString(ctl, &t(IDS_DISPLAY_LANGUAGE_EN));
        ComboBox_AddString(ctl, &t(IDS_DISPLAY_LANGUAGE_HANLO));
        ComboBox_SetCurSel(ctl, 0);
        0
    }
}
