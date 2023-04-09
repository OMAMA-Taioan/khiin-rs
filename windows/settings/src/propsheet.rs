use std::mem::transmute;
use std::rc::Rc;

use khiin_windows::resource::make_int_resource;
use khiin_windows::utils::pcwstr::ToPcwstr;
use windows::Win32::UI::WindowsAndMessaging::WM_INITDIALOG;
use windows::core::PCWSTR;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Gdi::HBITMAP;
use windows::Win32::UI::Controls::CreatePropertySheetPageW;
use windows::Win32::UI::Controls::InitCommonControls;
use windows::Win32::UI::Controls::HPROPSHEETPAGE;
use windows::Win32::UI::Controls::PROPSHEETPAGEW;
use windows::Win32::UI::Controls::PROPSHEETPAGEW_0;
use windows::Win32::UI::Controls::PROPSHEETPAGEW_1;
use windows::Win32::UI::Controls::PROPSHEETPAGEW_2;
use windows::Win32::UI::Controls::PSP_USETITLE;
use windows::Win32::UI::WindowsAndMessaging::GetSystemMenu;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongW;
use windows::Win32::UI::WindowsAndMessaging::InsertMenuW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongW;
use windows::Win32::UI::WindowsAndMessaging::DLGPROC;
use windows::Win32::UI::WindowsAndMessaging::GWL_STYLE;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::MF_BYPOSITION;
use windows::Win32::UI::WindowsAndMessaging::MF_STRING;
use windows::Win32::UI::WindowsAndMessaging::SC_MINIMIZE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_LONG_PTR_INDEX as WLPI;
use windows::Win32::UI::WindowsAndMessaging::WS_MINIMIZEBOX;

use crate::propsheetpage::Handler;
use crate::propsheetpage::PropSheetPage;

static DWLP_MSGRESULT: i32 = 0;
static DWLP_DLGPROC: i32 =
    DWLP_MSGRESULT + std::mem::size_of::<LRESULT>() as i32;
static DWLP_USER: WLPI =
    WLPI(DWLP_DLGPROC + std::mem::size_of::<DLGPROC>() as i32);

const ID_APPLY_NOW: u32 = 0x3021;
const PCSB_INITIALIZED: u32 = 1;
const PCSB_PRECREATE: u32 = 2;
const PSCB_BUTTONPRESSED: u32 = 3;

pub trait Propsheet<T: Handler> {
    extern "system" fn dlgproc(
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        match message {
            WM_INITDIALOG => {
                let psp: &PropSheetPage<T> = unsafe { transmute(lparam) };
                psp.handler.set_handle(handle);
                unsafe {
                    SetWindowLongPtrW(handle, DWLP_USER, transmute(lparam))
                };
                0
            }
            _ => {
                let userdata = unsafe { GetWindowLongPtrW(handle, DWLP_USER) };
                let this =
                    std::ptr::NonNull::<PropSheetPage<T>>::new(userdata as _);
                this.map_or(0, |mut t| unsafe {
                    t.as_mut().handler.on_message(message, wparam, lparam)
                })
            }
        }
    }

    fn create_page(
        module: HMODULE,
        template_id: u16,
        handler: Rc<T>,
    ) -> HPROPSHEETPAGE {
        let title = "Styles";
        let p_title = title.to_pcwstr();

        let page = PROPSHEETPAGEW {
            dwSize: std::mem::size_of::<PropSheetPage<T>>() as u32,
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

        let mut page = PropSheetPage {
            winapi: page,
            handler,
        };

        let hpsp = unsafe { CreatePropertySheetPageW(page.as_winapi()) };

        hpsp
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
