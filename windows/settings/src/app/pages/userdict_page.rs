use std::cell::RefCell;

use windows::Win32::Foundation::HWND;

use crate::app::pages::PageHandler;
use crate::app::resource::*;

#[derive(Default)]
pub struct UserDictPage {
    handle: RefCell<HWND>,
}

impl PageHandler for UserDictPage {
    fn handle(&self) -> HWND {
        *self.handle.borrow()
    }

    fn set_handle(&self, handle: HWND) {
        self.handle.replace(handle);
    }

    fn initialize(&self) -> isize {
        self.set_labels(vec![
            IDL_EDIT_USERDICT,
            IDC_EDIT_USEDICT_BTN,
            IDL_RESET_USERDATA,
            IDC_RESET_USERDATA_BTN,
        ]);

        0
    }
}
