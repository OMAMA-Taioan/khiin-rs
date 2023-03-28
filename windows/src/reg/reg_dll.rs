use windows::core::{ComInterface, Result, GUID};
use windows::w;
use windows::Win32::Globalization::LocaleNameToLCID;
use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_INPROC_SERVER};
use windows::Win32::System::Registry::HKEY_CLASSES_ROOT;
use windows::Win32::UI::TextServices::{
    CLSID_TF_CategoryMgr, CLSID_TF_InputProcessorProfiles, ITfCategoryMgr,
    ITfInputProcessorProfiles, ITfInputProcessorProfilesEx,
    GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER, GUID_TFCAT_TIPCAP_COMLESS,
    GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT, GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
    GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT, GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
    GUID_TFCAT_TIP_KEYBOARD,
};

use crate::reg::guids::*;
use crate::reg::hkey::Hkey;
use crate::utils::win::WinGuid;

const SUPPORTED_CATEGORIES: &'static [GUID] = &[
    GUID_TFCAT_DISPLAYATTRIBUTEPROVIDER,
    GUID_TFCAT_TIPCAP_COMLESS,
    GUID_TFCAT_TIPCAP_INPUTMODECOMPARTMENT,
    GUID_TFCAT_TIPCAP_UIELEMENTENABLED,
    GUID_TFCAT_TIP_KEYBOARD,
    GUID_TFCAT_TIPCAP_IMMERSIVESUPPORT,
    GUID_TFCAT_TIPCAP_SYSTRAYSUPPORT,
];

const CLSID: &str = "CLSID\\";
const CLSID_DESCRIPTION: &str = "Khiin Taiwanese IME";
const INPROC_SERVER_32: &str = "InprocServer32";
const THREADING_MODEL: &str = "ThreadingModel";
const APARTMENT: &str = "Apartment";

// KhiinTextService registration

pub fn register_clsid(module_path: &str) -> Result<()> {
    // Create the CLSID\{KhiinClassFactoryGuid} registry key
    let guid = IID_KhiinTextService.to_string()?;
    let subkey = CLSID.to_owned() + &guid;
    let clsid_hkey = HKEY_CLASSES_ROOT.create_subkey(&subkey)?;

    // Set the name of the IME
    clsid_hkey.set_string_value("", CLSID_DESCRIPTION)?;

    // Set the DLL to InprocServer32 type
    let inproc_hkey = clsid_hkey.create_subkey(INPROC_SERVER_32)?;

    // Set the path of the DLL module
    inproc_hkey.set_string_value("", module_path)?;

    // Set the threading model
    inproc_hkey.set_string_value(THREADING_MODEL, APARTMENT)?;

    inproc_hkey.close()?;
    clsid_hkey.close()?;

    Ok(())
}

pub fn unregister_clsid() -> Result<()> {
    // Delete the CLSID\{KhiinClassFactoryGuid} registry key
    let guid = IID_KhiinTextService.to_string()?;
    let subkey = CLSID.to_owned() + &guid;
    HKEY_CLASSES_ROOT.delete_tree(&subkey)?;

    Ok(())
}

// ITfInputProcessorProfiles registration
// https://learn.microsoft.com/en-us/windows/win32/api/msctf/nn-msctf-itfinputprocessorprofiles

pub fn register_profiles(
    module_path: &str,
    icon_index: u32,
    display_name_index: u32,
) -> Result<()> {
    unsafe {
        let profiles: ITfInputProcessorProfiles = CoCreateInstance(
            &CLSID_TF_InputProcessorProfiles,
            None,
            CLSCTX_INPROC_SERVER,
        )?;

        let lang_id: u16 = LocaleNameToLCID(w!("zh-TW"), 0).try_into().unwrap();
        let mut pch_desc: Vec<u16> = CLSID_DESCRIPTION.encode_utf16().collect();
        pch_desc.push(0);
        let mut module_path: Vec<u16> = module_path.encode_utf16().collect();
        module_path.push(0);

        profiles.Register(&IID_KhiinTextService)?;
        profiles.AddLanguageProfile(
            &IID_KhiinTextService,
            lang_id,
            &LanguageProfile,
            &pch_desc,
            &module_path,
            icon_index,
        )?;

        let profiles_ex: ITfInputProcessorProfilesEx = profiles.cast()?;
        profiles_ex.SetLanguageProfileDisplayName(
            &IID_KhiinTextService,
            lang_id,
            &LanguageProfile,
            &module_path,
            display_name_index,
        )?;
    }

    Ok(())
}

pub fn unregister_profiles() -> Result<()> {
    unsafe {
        let profiles: ITfInputProcessorProfiles = CoCreateInstance(
            &CLSID_TF_InputProcessorProfiles,
            None,
            CLSCTX_INPROC_SERVER,
        )?;

        profiles.Unregister(&IID_KhiinTextService)
    }
}

// ITfCategoryMgr registration
// https://learn.microsoft.com/en-us/windows/win32/api/msctf/nn-msctf-itfcategorymgr

pub fn register_categories() -> Result<()> {
    unsafe {
        let category_mgr: ITfCategoryMgr = CoCreateInstance(
            &CLSID_TF_CategoryMgr,
            None,
            CLSCTX_INPROC_SERVER,
        )?;

        for category in SUPPORTED_CATEGORIES {
            category_mgr.RegisterCategory(
                &IID_KhiinTextService,
                category,
                &IID_KhiinTextService,
            )?;
        }
    }

    Ok(())
}

pub fn unregister_categories() -> Result<()> {
    unsafe {
        let category_mgr: ITfCategoryMgr = CoCreateInstance(
            &CLSID_TF_CategoryMgr,
            None,
            CLSCTX_INPROC_SERVER,
        )?;

        for category in SUPPORTED_CATEGORIES {
            category_mgr.UnregisterCategory(
                &IID_KhiinTextService,
                category,
                &IID_KhiinTextService,
            )?;
        }
    }

    Ok(())
}
