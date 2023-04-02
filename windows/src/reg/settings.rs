use std::ffi::OsString;
use std::ffi::c_void;
use std::os::windows::prelude::OsStringExt;

use windows::core::Result;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::System::Registry::RegGetValueA;
use windows::Win32::System::Registry::RegGetValueW;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::HKEY_CURRENT_USER;
use windows::Win32::System::Registry::RRF_RT_REG_SZ;

use crate::check_win32error;
use crate::pcwstr;
use crate::reg::hkey::Hkey;
use crate::winerr;

static SETTINGS_REG_PATH: &str = "Software\\Khiin PJH\\Settings";

fn settings_root() -> Result<HKEY> {
    HKEY_CURRENT_USER.create_subkey(SETTINGS_REG_PATH)
}

fn get_string_value(key: HKEY, name: &str) -> Result<OsString> {
    unsafe {
        let subkey = pcwstr!(name);
        let mut data_size: u32 = 0;
        if ERROR_SUCCESS
            != RegGetValueW(
                key,
                None,
                subkey,
                RRF_RT_REG_SZ,
                None,
                None,
                Some(&mut data_size),
            )
        {
            return winerr!(E_FAIL);
        }
        let u16_size = data_size as usize / std::mem::size_of::<u16>();
        let mut data: Vec<u16> = vec![0; u16_size];
        if ERROR_SUCCESS
            != RegGetValueW(
                key,
                None,
                subkey,
                RRF_RT_REG_SZ,
                None,
                Some(data.as_ptr() as *mut c_void),
                Some(&mut data_size),
            )
        {
            return winerr!(E_FAIL);
        }
        let data_size = data_size as usize;
        let u16_size = data_size / std::mem::size_of::<u16>();
        data.truncate(data_size / u16_size - 1);

        Ok(OsString::from_wide(&data))
    }
}
