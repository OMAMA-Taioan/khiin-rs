use std::ffi::c_void;
use std::mem::size_of;
use std::mem::transmute;
use std::sync::Arc;

use windows::core::AsImpl;
use windows::core::Result;
use windows::core::PCWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::LRESULT;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::WindowsAndMessaging::CreateWindowExW;
use windows::Win32::UI::WindowsAndMessaging::DefWindowProcW;
use windows::Win32::UI::WindowsAndMessaging::GetClassInfoExW;
use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::IsWindow;
use windows::Win32::UI::WindowsAndMessaging::RegisterClassExW;
use windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;
use windows::Win32::UI::WindowsAndMessaging::UnregisterClassW;
use windows::Win32::UI::WindowsAndMessaging::CREATESTRUCTW;
use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;
use windows::Win32::UI::WindowsAndMessaging::GWLP_USERDATA;
use windows::Win32::UI::WindowsAndMessaging::HCURSOR;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use windows::Win32::UI::WindowsAndMessaging::HMENU;
use windows::Win32::UI::WindowsAndMessaging::HWND_MESSAGE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_EX_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_STYLE;
use windows::Win32::UI::WindowsAndMessaging::WM_NCCREATE;
use windows::Win32::UI::WindowsAndMessaging::WM_USER;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW;
use windows::Win32::UI::WindowsAndMessaging::WNDCLASS_STYLES;

use khiin_protos::command::*;

use crate::fail;
use crate::utils::ToPcwstr;

pub const WM_KHIIN_COMMAND: u32 = WM_USER;

pub struct MessageHandler {
    tip: ITfTextInputProcessor,
}

impl MessageHandler {
    const WINDOW_CLASS_NAME: &'static str = "MessageHandler";

    pub fn new(tip: ITfTextInputProcessor) -> Self {
        Self { tip }
    }

    fn on_message(
        &mut self,
        message: u32,
        _wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<()> {
        match message {
            WM_KHIIN_COMMAND => {
                unsafe {
                    let cmd: Arc<Command> = Arc::from_raw(transmute(lparam));
                    self.tip.as_impl().handle_command(cmd)?;
                }

                Ok(())
            },
            _ => Err(fail!()),
        }
    }

    // Should be called when the app starts, or via Self::create
    fn register_class(module: HMODULE) -> bool {
        unsafe {
            let class_name = Self::WINDOW_CLASS_NAME.to_pcwstr();
            let mut wc = WNDCLASSEXW::default();

            if GetClassInfoExW(module, *class_name, &mut wc).is_ok() {
                // already registered
                return true;
            }

            let wc = WNDCLASSEXW {
                cbSize: size_of::<WNDCLASSEXW>() as u32,
                style: WNDCLASS_STYLES::default(),
                lpfnWndProc: Some(Self::wndproc),
                cbClsExtra: 0,
                hInstance: module.into(),
                lpszClassName: *class_name,
                hIcon: HICON::default(),
                hIconSm: HICON::default(),
                hCursor: HCURSOR::default(),
                lpszMenuName: PCWSTR::null(),
                hbrBackground: HBRUSH::default(),
                cbWndExtra: 0,
            };

            RegisterClassExW(&wc) != 0
        }
    }

    // Should be called when the app is deactivated
    pub fn unregister_class(module: HMODULE) -> bool {
        unsafe {
            let class_name = Self::WINDOW_CLASS_NAME.to_pcwstr();
            let mut wc = WNDCLASSEXW::default();

            if GetClassInfoExW(module, *class_name, &mut wc).is_ok() {
                // already unregistered
                return true;
            }

            UnregisterClassW(*class_name, module).is_ok()
        }
    }

    pub fn create(this: Arc<Self>, module: HMODULE) -> Result<HWND> {
        unsafe {
            let instance = GetModuleHandleW(None)?;
            let class_name = Self::WINDOW_CLASS_NAME.to_pcwstr();
            if !Self::register_class(module) {
                return Err(fail!());
            }

            let this_ptr = Arc::into_raw(this.clone());

            let handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                *class_name,
                PCWSTR::null(),
                WINDOW_STYLE::default(),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                HWND_MESSAGE,
                HMENU::default(),
                module,
                Some(this_ptr as *mut c_void),
            );

            if IsWindow(handle) != FALSE {
                Ok(handle)
            } else {
                let err = GetLastError();
                Err(fail!())
            }
        }
    }

    extern "system" fn wndproc(
        handle: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let lpcs: &CREATESTRUCTW = transmute(lparam);
                SetWindowLongPtrW(
                    handle,
                    GWLP_USERDATA,
                    lpcs.lpCreateParams as _,
                );
                return DefWindowProcW(handle, message, wparam, lparam);
            }

            let userdata = GetWindowLongPtrW(handle, GWLP_USERDATA);
            let this = std::ptr::NonNull::<Self>::new(userdata as _);
            let handled = this.map_or(false, |mut s| {
                s.as_mut().on_message(message, wparam, lparam).is_ok()
            });
            if handled {
                LRESULT::default()
            } else {
                DefWindowProcW(handle, message, wparam, lparam)
            }
        }
    }
}
