use std::mem::transmute;
use std::rc::Rc;

use khiin_windows::resource::make_int_resource;
use khiin_windows::resource::IDI_MAINICON;
use khiin_windows::utils::pcwstr::ToPcwstr;

use windows::core::PCWSTR;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::Graphics::Gdi::HPALETTE;
use windows::Win32::UI::Controls::CreatePropertySheetPageW;
use windows::Win32::UI::Controls::InitCommonControls;
use windows::Win32::UI::Controls::PropertySheetW;
use windows::Win32::UI::Controls::HPROPSHEETPAGE;
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
use windows::Win32::UI::WindowsAndMessaging::GetSystemMenu;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongW;
use windows::Win32::UI::WindowsAndMessaging::InsertMenuW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongW;
use windows::Win32::UI::WindowsAndMessaging::GWL_STYLE;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::HWND_DESKTOP;
use windows::Win32::UI::WindowsAndMessaging::MF_BYPOSITION;
use windows::Win32::UI::WindowsAndMessaging::MF_STRING;
use windows::Win32::UI::WindowsAndMessaging::SC_MINIMIZE;
use windows::Win32::UI::WindowsAndMessaging::WM_INITDIALOG;
use windows::Win32::UI::WindowsAndMessaging::WS_MINIMIZEBOX;

use crate::locales::t;
use crate::pages::PageHandler;
use crate::pages::PropSheetPage;
use crate::winuser::*;

static mut INITIALIZED: bool = false;

pub struct PropSheet {
    module: HMODULE,
    pages: Vec<PropSheetPage>,
    psp_handles: Vec<HPROPSHEETPAGE>,
}

impl PropSheet {
    pub fn new(module: HMODULE) -> Self {
        Self {
            module,
            pages: Vec::new(),
            psp_handles: Vec::new(),
        }
    }

    pub fn run(&mut self) -> isize {
        let mut psh = PROPSHEETHEADERW_V2 {
            dwSize: std::mem::size_of::<PROPSHEETHEADERW_V2>() as u32,
            dwFlags: PSH_NOCONTEXTHELP | PSH_USECALLBACK | PSH_USEICONID,
            hwndParent: HWND_DESKTOP,
            hInstance: self.module,
            Anonymous1: PROPSHEETHEADERW_V2_0 {
                pszIcon: make_int_resource(IDI_MAINICON),
            },
            pszCaption: PCWSTR::null(),
            nPages: self.psp_handles.len() as u32,
            Anonymous2: PROPSHEETHEADERW_V2_1 { nStartPage: 0 },
            Anonymous3: PROPSHEETHEADERW_V2_2 {
                phpage: self.psp_handles.as_mut_ptr(),
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

        unsafe { PropertySheetW(&mut psh) }
    }

    pub fn add_page(
        &mut self,
        template_id: u16,
        handler: Rc<dyn PageHandler>,
    ) -> HPROPSHEETPAGE {
        let p_title = t(template_id).to_pcwstr();

        let page = PROPSHEETPAGEW {
            dwSize: std::mem::size_of::<PropSheetPage>() as u32,
            dwFlags: PSP_USETITLE,
            hInstance: self.module,
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

        let mut page = PropSheetPage {
            winapi: page,
            handler,
        };

        let hpsp = unsafe { CreatePropertySheetPageW(page.as_winapi()) };

        self.pages.push(page);
        self.psp_handles.push(hpsp);

        hpsp
    }

    extern "system" fn dlgproc(
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        match message {
            WM_INITDIALOG => {
                let psp: &PropSheetPage = unsafe { transmute(lparam) };
                psp.handler.set_handle(handle);
                unsafe {
                    SetWindowLongPtrW(handle, DWLP_USER, transmute(lparam));
                    INITIALIZED = true;
                };
                psp.handler.on_message(message, wparam, lparam)
            }
            _ => {
                if unsafe { !INITIALIZED } {
                    0
                } else {
                    let userdata =
                        unsafe { GetWindowLongPtrW(handle, DWLP_USER) };
                    let this =
                        std::ptr::NonNull::<PropSheetPage>::new(userdata as _);
                    this.map_or(0, |mut t| unsafe {
                        t.as_mut().handler.on_message(message, wparam, lparam)
                    })
                }
            }
        }
    }
}

pub unsafe extern "system" fn propsheet_cb(
    handle: HWND,
    message: u32,
    _lparam: LPARAM,
) -> i32 {
    match message {
        PCSB_INITIALIZED => {
            // Add the minimize button which is not usually on property sheets
            let newitem = "Minimize".to_pcwstr();
            SetWindowLongW(
                handle,
                GWL_STYLE,
                GetWindowLongW(handle, GWL_STYLE) | WS_MINIMIZEBOX.0 as i32,
            );
            let hmenu = GetSystemMenu(handle, FALSE);
            InsertMenuW(
                hmenu,
                u32::MAX,
                MF_BYPOSITION | MF_STRING,
                SC_MINIMIZE as usize,
                *newitem,
            );
            0
        }
        PCSB_PRECREATE => {
            InitCommonControls();
            0
        }
        _ => 0,
    }
}
