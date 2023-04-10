#![cfg(windows)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod locales;
mod macros;
mod pages;
mod propsheet;
mod resource;
mod windowsx;
mod winuser;

use std::rc::Rc;

use pages::InputPage;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::resource::*;
use crate::propsheet::PropSheet;
use crate::pages::StylePage;

fn run() -> Result<isize> {
    let module = unsafe { GetModuleHandleW(PCWSTR::null())? };
    let mut propsheet = PropSheet::new(module);

    let style_page = Rc::new(StylePage::default());
    propsheet.add_page(IDD_APPEARANCETAB, style_page);

    let input_page = Rc::new(InputPage::default());
    propsheet.add_page(IDD_INPUTTAB, input_page);

    let pid = propsheet.run();
    Ok(pid)
}

pub fn main() {
    run().unwrap();
}
