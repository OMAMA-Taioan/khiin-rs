use std::cell::RefCell;

use khiin_windows::utils::pcwstr::ToPcwstr;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetParent;
use windows::Win32::UI::WindowsAndMessaging::SetWindowTextW;

use crate::locales::t;
use crate::pages::PageHandler;
use crate::resource::*;

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

        self.set_labels(vec![
            IDL_COLOR,
            IDL_CANDIDATE_SIZE,
            IDL_CANDIDATE_SIZE_S,
            IDL_CANDIDATE_SIZE_L,
            IDL_DISPLAY_LANGUAGE,
            IDL_EDIT_TRY,
        ]);

        self.init_combobox(
            IDC_COMBOBOX_THEME_COLOR,
            vec![IDS_LIGHT_THEME, IDS_DARK_THEME],
            0,
        );

        self.init_combobox(
            IDC_DISPLAY_LANGUAGE,
            vec![IDS_DISPLAY_LANGUAGE_EN, IDS_DISPLAY_LANGUAGE_HANLO],
            0,
        );

        0
    }
}
