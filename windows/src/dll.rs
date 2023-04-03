use log::warn;
use once_cell::sync::OnceCell;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Foundation::S_FALSE;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use windows::core::ComInterface;

use windows::core::Result;
use windows::core::GUID;
use windows::core::HRESULT;
use windows::w;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::CLASS_E_CLASSNOTAVAILABLE;
use windows::Win32::Foundation::E_UNEXPECTED;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::S_OK;
use windows::Win32::System::Com::IClassFactory;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::SystemServices::DLL_PROCESS_DETACH;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::MB_DEFBUTTON2;

use crate::reg::guids::*;
use crate::reg::registrar::register_categories;
use crate::reg::registrar::register_clsid;
use crate::reg::registrar::register_profiles;
use crate::reg::registrar::unregister_categories;
use crate::reg::registrar::unregister_clsid;
use crate::reg::registrar::unregister_profiles;
use crate::reg::settings;
use crate::reg::settings::SettingsKey::DebugSingleAppMode;
use crate::tip::class_factory::KhiinClassFactory;
use crate::utils::win::GetPath;
use crate::utils::win::WinGuid;

// Normally leave this `false`, but due to the nature of some bugs
// causing a cascade of DLL loads and crashes, it is sometimes very
// convenient to set this to `true` so that the DLL will only load in
// at most one app at a time. If you set this to `true`, you must
// manually reset the registry entry each time you run the app,
// or you won't be able to run it again. The entry is at
//     HKEY_CURRENT_USER\Software\Khiin PJH\Settings
// Set `SingleAppDebugMode` to 0 to run it, and running once will
// change this key to 1 to prevent future attachments.
static DEBUG_SINGLE_APP_MODE: bool = false;

static DLL_INSTANCE: OnceCell<DllModule> = OnceCell::new();
const IDS_TEXT_SERVICE_DISPLAY_NAME: u32 = 101;

fn locked_for_debugging() -> bool {
    if !DEBUG_SINGLE_APP_MODE {
        return false;
    }

    let already_in_use = match settings::get_settings_u32(DebugSingleAppMode) {
        Ok(val) => val != 0,
        _ => false
    };

    match already_in_use {
        true => true,
        false => {
            settings::set_settings_u32(DebugSingleAppMode, 1).ok();
            false
        }
    }
}

#[derive(Debug)]
pub struct DllModule {
    pub ref_count: Arc<AtomicUsize>,
    pub module: HMODULE,
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
        DllModule::global().module.get_path()
    }
}

#[no_mangle]
pub extern "system" fn DllMain(
    module: HMODULE,
    call_reason: u32,
    _reserved: *mut std::ffi::c_void,
) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            if locked_for_debugging() {
                return FALSE;
            }

            let dll_module = DllModule {
                ref_count: Arc::new(AtomicUsize::new(0)),
                module,
            };
            DLL_INSTANCE.set(dll_module).unwrap();
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    BOOL::from(true)
}

#[no_mangle]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    match DllModule::global().can_unload() {
        true => S_OK,
        false => S_FALSE,
    }
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

    if *riid != IClassFactory::IID {
        warn!(
            "DllGetClassObject: Unexpected IID requested: {}",
            riid.to_string().unwrap_or_default()
        );

        return E_UNEXPECTED;
    }

    let factory = KhiinClassFactory::new();
    let factory: IClassFactory = factory.into();

    *ppv = unsafe { core::mem::transmute(factory) };

    S_OK
}

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
