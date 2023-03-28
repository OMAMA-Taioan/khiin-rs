#![cfg(windows)]
#![allow(non_snake_case)]

mod protos;
mod reg;
mod tip;
mod utils;

use log::warn;
use once_cell::sync::OnceCell;
use reg::reg_dll::register_categories;
use reg::reg_dll::register_profiles;
use reg::reg_dll::unregister_categories;
use reg::reg_dll::unregister_profiles;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use windows::core::Result;
use windows::core::GUID;
use windows::core::HRESULT;
use windows::w;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::CLASS_E_CLASSNOTAVAILABLE;
use windows::Win32::Foundation::E_UNEXPECTED;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::S_OK;
use windows::Win32::System::Com::IClassFactory;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::SystemServices::DLL_PROCESS_DETACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::MB_DEFBUTTON2;

use crate::reg::guids::*;
use crate::reg::reg_dll::register_clsid;
use crate::reg::reg_dll::unregister_clsid;
use crate::tip::class_factory::KhiinClassFactory;
use crate::utils::win::GetPath;
use crate::utils::win::WinGuid;

static DLL_INSTANCE: OnceCell<DllModule> = OnceCell::new();
const IDS_TEXT_SERVICE_DISPLAY_NAME: u32 = 101;

#[derive(Debug)]
pub struct DllModule {
    ref_count: Arc<AtomicUsize>,
    hinstance: HINSTANCE,
}

impl DllModule {
    pub fn global() -> &'static DllModule {
        DLL_INSTANCE.get().expect("DllModule is not initialized")
    }

    pub fn add_ref(&self) -> usize {
        self.ref_count.fetch_add(1, Ordering::SeqCst)
    }

    pub fn release(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst)
    }

    pub fn can_unload(&self) -> bool {
        self.ref_count.load(Ordering::SeqCst) <= 0
    }

    pub fn path() -> Result<String> {
        DllModule::global().hinstance.get_path()
    }
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[no_mangle]
extern "system" fn DllMain(
    hinstance: HINSTANCE,
    call_reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            let dll_module = DllModule {
                ref_count: Arc::new(AtomicUsize::new(0)),
                hinstance,
            };
            DLL_INSTANCE.set(dll_module).unwrap();
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    BOOL::from(true)
}

#[no_mangle]
pub extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut std::ffi::c_void,
) -> HRESULT {
    let rclsid = &unsafe { *rclsid };
    let riid = &unsafe { *riid };
    let ppv = unsafe { &mut *ppv };

    *ppv = std::ptr::null_mut();

    if *rclsid != IID_KhiinTextService {
        warn!(
            "DllGetClassObject: Unexpected CLSID requested: {}",
            rclsid.to_string().unwrap_or_default()
        );

        return CLASS_E_CLASSNOTAVAILABLE;
    }

    if *riid != IID_IClassFactory {
        warn!(
            "DllGetClassObject: Unexpected IID requested: {}",
            riid.to_string().unwrap_or_default()
        );

        return E_UNEXPECTED;
    }

    let factory = KhiinClassFactory::new(DllModule::global().ref_count.clone());
    let factory: IClassFactory = factory.into();

    *ppv = unsafe { core::mem::transmute(factory) };

    S_OK
}

#[allow(unused_must_use)]
#[no_mangle]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Waiting for debugger..."),
            w!("OK"),
            MB_DEFBUTTON2,
        );
    }

    let module_path = DllModule::path();
    if module_path.is_err() {
        return HRESULT::from(E_UNEXPECTED);
    }

    let result = register_clsid(module_path.unwrap().as_ref());

    if result.is_err() {
        return DllUnregisterServer();
    }

    let module_path = DllModule::path();
    if module_path.is_err() {
        return HRESULT::from(E_UNEXPECTED);
    }

    let result = register_profiles(
        module_path.unwrap().as_ref(),
        0,
        IDS_TEXT_SERVICE_DISPLAY_NAME,
    );
    if result.is_err() {
        return DllUnregisterServer();
    }

    let result = register_categories();
    if result.is_err() {
        return DllUnregisterServer();
    }

    result.into()
}

#[no_mangle]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Waiting for debugger..."),
            w!("OK"),
            MB_DEFBUTTON2,
        );
    }

    let result = unregister_categories();
    if result.is_err() {
        return result.into();
    }

    let result = unregister_profiles();
    if result.is_err() {
        return result.into();
    }

    let result = unregister_clsid();
    if result.is_err() {
        return result.into();
    }

    result.into()
}
