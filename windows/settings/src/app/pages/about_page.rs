use std::cell::RefCell;

use windows::Win32::Foundation::HWND;

use crate::app::pages::PageHandler;
use crate::app::resource::*;

#[derive(Default)]
pub struct AboutPage {
    pub template_id: u16,
    handle: RefCell<HWND>,
}

impl PageHandler for AboutPage {
    fn handle(&self) -> HWND {
        *self.handle.borrow()
    }

    fn set_handle(&self, handle: HWND) {
        self.handle.replace(handle);
    }

    fn initialize(&self) -> isize {
        self.set_labels(vec![IDL_KHIIN_VERSION, IDL_KHIIN_COPYRIGHT]);

        0
    }
}
