use std::rc::Rc;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Controls::PROPSHEETPAGEW;

// cf Appending additional payload to a PROPSHEETPAGE structure
// https://devblogs.microsoft.com/oldnewthing/20211124-00/?p=105961
#[repr(C)]
pub struct PropSheetPage {
    pub winapi: PROPSHEETPAGEW,
    pub handler: Rc<dyn PageHandler>,
}

pub trait PageHandler {
    fn set_handle(&self, handle: HWND);

    fn on_message(
        &self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        0
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
