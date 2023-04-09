#![cfg(windows)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod propsheet;
mod propsheetpage;

use std::cell::RefCell;
use std::rc::Rc;

use khiin_windows::resource::*;

use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::propsheet::PropSheet;
use crate::propsheetpage::PageHandler;

#[derive(Default)]
struct StylePage {
    handle: RefCell<HWND>,
}

impl PageHandler for StylePage {
    fn set_handle(&self, handle: HWND) {
        self.handle.replace(handle);
    }
}

fn run() -> Result<isize> {
    let module = unsafe { GetModuleHandleW(PCWSTR::null())? };
    let mut propsheet = PropSheet::new(module);

    let style_page = Rc::new(StylePage::default());
    propsheet.add_page(IDD_APPEARANCETAB, style_page);

    let pid = propsheet.run();
    Ok(pid)
}

pub fn main() {
    run().unwrap();
}
