#![cfg(windows)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::ffi::c_void;

use khiin_windows::resource::*;
use khiin_windows::utils::pcwstr::ToPcwstr;

use windows::Win32::UI::Controls::CreatePropertySheetPageW;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::Graphics::Gdi::HPALETTE;
use windows::Win32::System::LibraryLoader::GetModuleHandleExW;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::SystemServices::userHPALETTE_0;
use windows::Win32::UI::Controls::InitCommonControls;
use windows::Win32::UI::Controls::PropertySheetW;
use windows::Win32::UI::Controls::HPROPSHEETPAGE;
use windows::Win32::UI::Controls::LPFNPSPCALLBACKW;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_0;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_1;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_2;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_3;
use windows::Win32::UI::Controls::PROPSHEETHEADERW_V2_4;
use windows::Win32::UI::Controls::PROPSHEETPAGEW;
use windows::Win32::UI::Controls::PROPSHEETPAGEW_0;
use windows::Win32::UI::Controls::PROPSHEETPAGEW_1;
use windows::Win32::UI::Controls::PROPSHEETPAGEW_2;
use windows::Win32::UI::Controls::PSH_NOCONTEXTHELP;
use windows::Win32::UI::Controls::PSH_USECALLBACK;
use windows::Win32::UI::Controls::PSH_USEICONID;
use windows::Win32::UI::Controls::PSP_USETITLE;
use windows::Win32::UI::WindowsAndMessaging::LoadIconW;
use windows::Win32::UI::WindowsAndMessaging::DLGPROC;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::HWND_DESKTOP;

pub struct PropSheetPage {
    handle: RefCell<HWND>,
}

impl PropSheetPage {}

impl Dlgproc for PropSheetPage {}

pub trait Dlgproc {
    extern "system" fn dlgproc(
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        0
    }

    fn create(module: HMODULE, template_id: u16) -> HPROPSHEETPAGE {
        let title = "Styles";
        let p_title = title.to_pcwstr();

        let mut page = PROPSHEETPAGEW {
            dwSize: std::mem::size_of::<PROPSHEETPAGEW>() as u32,
            dwFlags: PSP_USETITLE,
            hInstance: module,
            Anonymous1: PROPSHEETPAGEW_0 {
                pszTemplate: make_int_resource(template_id),
            },
            Anonymous2: PROPSHEETPAGEW_1 { hIcon: HICON(0) },
            pszTitle: *p_title,
            pfnDlgProc: Some(Self::dlgproc),
            lParam: LPARAM(0),
            pfnCallback: None,
            pcRefParent: std::ptr::null_mut(),
            pszHeaderTitle: PCWSTR::null(),
            pszHeaderSubTitle: PCWSTR::null(),
            hActCtx: HANDLE(0),
            Anonymous3: PROPSHEETPAGEW_2 {
                hbmHeader: HBITMAP(0),
            },
        };

        let hpsp = unsafe { CreatePropertySheetPageW(&mut page) };

        hpsp
    }
}

extern "system" fn propsheet_cb(
    handle: HWND,
    message: u32,
    lparam: LPARAM,
) -> i32 {
    0
}

fn run() -> Result<isize> {
    let module = unsafe { GetModuleHandleW(PCWSTR::null())? };
    let template = IDD_APPEARANCETAB;
    let psp = <PropSheetPage as Dlgproc>::create(module, template);
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
        Anonymous4: PROPSHEETHEADERW_V2_3::default(),
        hplWatermark: HPALETTE(0),
        Anonymous5: PROPSHEETHEADERW_V2_4::default(),
    };

    let pid = unsafe { PropertySheetW(&mut psh) };

    Ok(pid)
}

pub fn main() {
    unsafe {
        InitCommonControls();
    }
    run().unwrap();
}
