#![cfg(windows)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod propsheetpage;
mod propsheet;

use std::cell::RefCell;
use std::rc::Rc;

use khiin_windows::resource::*;

use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HPALETTE;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::PropertySheetW;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_0;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_1;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_2;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_3;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_4;
use windows::Win32::UI::Controls::PSH_NOCONTEXTHELP;
use windows::Win32::UI::Controls::PSH_USECALLBACK;
use windows::Win32::UI::Controls::PSH_USEICONID;
use windows::Win32::UI::WindowsAndMessaging::HWND_DESKTOP;

use crate::propsheet::Propsheet;
use crate::propsheet::propsheet_cb;
use crate::propsheetpage::Handler;


#[derive(Default)]
struct StylePage {
    handle: RefCell<HWND>,
}

impl Handler for StylePage {
    fn set_handle(&self, handle: HWND) {
        self.handle.replace(handle);
    }
}

impl Propsheet<StylePage> for StylePage {}

fn run() -> Result<isize> {
    let module = unsafe { GetModuleHandleW(PCWSTR::null())? };

    let style_page = Rc::new(StylePage::default());
    let psp =
        StylePage::create_page(module, IDD_APPEARANCETAB, style_page.clone());
    let mut psp_vec = vec![psp];

    let mut psh = PROPSHEETHEADERW_V2 {
        dwSize: std::mem::size_of::<PROPSHEETHEADERW_V2>() as u32,
        dwFlags: PSH_NOCONTEXTHELP | PSH_USECALLBACK | PSH_USEICONID,
        hwndParent: HWND_DESKTOP,
        hInstance: module,
        Anonymous1: PROPSHEETHEADERW_V2_0 {
            pszIcon: make_int_resource(IDI_MAINICON),
        },
        pszCaption: PCWSTR::null(),
        nPages: 1,
        Anonymous2: PROPSHEETHEADERW_V2_1 { nStartPage: 0 },
        Anonymous3: PROPSHEETHEADERW_V2_2 {
            phpage: psp_vec.as_mut_ptr(),
        },
        pfnCallback: Some(propsheet_cb),
        Anonymous4: PROPSHEETHEADERW_V2_3 {
            pszbmWatermark: PCWSTR::null(),
        },
        hplWatermark: HPALETTE(0),
        Anonymous5: PROPSHEETHEADERW_V2_4 {
            pszbmHeader: PCWSTR::null(),
        },
    };

    let pid = unsafe { PropertySheetW(&mut psh) };

    Ok(pid)
}

pub fn main() {
    run().unwrap();
}
