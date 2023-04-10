#![cfg(windows)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod locales;
mod macros;
mod propsheet;
mod propsheetpage;
mod resource;
mod style_page;
mod windowsx;
mod winuser;

use std::rc::Rc;


use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use crate::resource::*;
use crate::propsheet::PropSheet;
use crate::style_page::StylePage;

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
