use std::cell::RefCell;

use khiin_windows::resource::*;
use khiin_windows::utils::pcwstr::ToPcwstr;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetParent, SetWindowTextW};

use crate::windowsx::*;
use crate::propsheetpage::PageHandler;

static TITLE: &str = "起引拍字法 Khíín Settings";
static S1: &str = "Hello";
static S2: &str = "World";

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
        let title_p = TITLE.to_pcwstr();
        unsafe { SetWindowTextW(hwnd, *title_p); }

        let hwndCtl = self.item(IDC_COMBOBOX_THEME_COLOR);
        ComboBox_ResetContent(hwndCtl);
        ComboBox_AddString(hwndCtl, S1);
        ComboBox_AddString(hwndCtl, S2);
        ComboBox_SetCurSel(hwndCtl, 0);

        let hwndCtl = self.item(IDC_DISPLAY_LANGUAGE);
        ComboBox_ResetContent(hwndCtl);
        ComboBox_AddString(hwndCtl, S1);
        ComboBox_AddString(hwndCtl, S2);
        ComboBox_SetCurSel(hwndCtl, 0);
        0
    }
}
