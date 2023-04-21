use std::rc::Rc;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Controls::PROPSHEETPAGEW;
use windows::Win32::UI::WindowsAndMessaging::GetDlgItem;
use windows::Win32::UI::WindowsAndMessaging::WM_INITDIALOG;

use crate::app::locales::t;
use crate::app::windowsx::ComboBox_AddString;
use crate::app::windowsx::ComboBox_ResetContent;
use crate::app::windowsx::ComboBox_SetCurSel;
use crate::app::windowsx::Static_SetText;

// cf Appending additional payload to a PROPSHEETPAGE structure
// https://devblogs.microsoft.com/oldnewthing/20211124-00/?p=105961
#[repr(C)]
pub struct PropSheetPage {
    pub winapi: PROPSHEETPAGEW,
    pub handler: Rc<dyn PageHandler>,
}

pub trait PageHandler {
    fn handle(&self) -> HWND;

    fn set_handle(&self, handle: HWND);

    fn item(&self, rid: u16) -> HWND {
        unsafe { GetDlgItem(self.handle(), rid as i32) }
    }

    fn set_labels(&self, label_rids: Vec<u16>) {
        for rid in label_rids {
            let ctl = self.item(rid);
            Static_SetText(ctl, &t(rid));
        }
    }

    fn init_combobox(
        &self,
        ctl_rid: u16,
        option_rids: Vec<u16>,
        selected_idx: usize,
    ) {
        let ctl = self.item(ctl_rid);
        ComboBox_ResetContent(ctl);
        for rid in option_rids {
            ComboBox_AddString(ctl, &t(rid));
        }
        ComboBox_SetCurSel(ctl, selected_idx);
    }

    fn initialize(&self) -> isize;

    fn on_message(
        &self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        match message {
            WM_INITDIALOG => self.initialize(),
            _ => 0,
        }
    }
}

impl core::ops::Deref for PropSheetPage {
    type Target = PROPSHEETPAGEW;
    fn deref(&self) -> &Self::Target {
        &self.winapi
    }
}

impl core::ops::DerefMut for PropSheetPage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.winapi
    }
}

impl PropSheetPage {
    pub fn as_winapi(&mut self) -> *mut PROPSHEETPAGEW {
        let p: *mut Self = self;
        p.cast()
    }
}
