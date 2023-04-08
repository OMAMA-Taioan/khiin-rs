use std::ffi::c_void;
use std::ffi::OsString;
use std::os::windows::prelude::OsStringExt;

use log::debug;
use windows::core::Result;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::ERROR_FILE_NOT_FOUND;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::System::Registry::RegDeleteValueW;
use windows::Win32::System::Registry::RegGetValueW;
use windows::Win32::System::Registry::RegSetValueExW;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::HKEY_CURRENT_USER;
use windows::Win32::System::Registry::REG_DWORD;
use windows::Win32::System::Registry::REG_SZ;
use windows::Win32::System::Registry::RRF_RT_REG_DWORD;
use windows::Win32::System::Registry::RRF_RT_REG_SZ;

use crate::check_win32error;
use crate::reg::hkey::Hkey;
use crate::ui::colors::SystemTheme;
use crate::utils::ToPcwstr;
use crate::utils::WinString;
use crate::winerr;

static SETTINGS_REG_PATH: &str = "Software\\Khiin PJH\\Settings";
static SYSTEM_THEME_SUBKEY: &str =
    "Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize";
static SYSTEM_THEME_SUBKEY_NAME: &str = "SystemUsesLightTheme";

pub enum SettingsKey {
    SettingsApp,
    DatabaseFile,
    ConfigFile,
    UserDbFile,
    FontDir,
    UiColors,
    UiSize,
    UiLanguage,
    OnOffHotkey,
    InputModeHotkey,
    UserDictionaryFile,
    DebugLock,
}

impl SettingsKey {
    pub fn reg_key(&self) -> &'static str {
        match *self {
            SettingsKey::SettingsApp => "settings_exe",
            SettingsKey::DatabaseFile => "installed_db",
            SettingsKey::ConfigFile => "config_toml",
            SettingsKey::UserDbFile => "user_db",
            SettingsKey::FontDir => "font_dir",
            SettingsKey::UiColors => "ui_colors",
            SettingsKey::UiSize => "ui_size",
            SettingsKey::UiLanguage => "ui_language",
            SettingsKey::OnOffHotkey => "on_off_hotkey",
            SettingsKey::InputModeHotkey => "input_mode_hotkey",
            SettingsKey::UserDictionaryFile => "user_dictionary_file",
            SettingsKey::DebugLock => "DebugLock",
        }
    }
}

pub fn get_system_theme() -> Result<SystemTheme> {
    unsafe {
        let subkey = SYSTEM_THEME_SUBKEY.to_pcwstr();
        let name = SYSTEM_THEME_SUBKEY_NAME.to_pcwstr();
        let data = Box::into_raw(Box::from(0u32));
        let mut data_size = std::mem::size_of::<u32>() as u32;

        let err = RegGetValueW(
            HKEY_CURRENT_USER,
            *subkey,
            *name,
            RRF_RT_REG_DWORD,
            None,
            Some(data as *mut c_void),
            Some(&mut data_size),
        );

        if err != ERROR_SUCCESS {
            let err = GetLastError().0;
            debug!("error: {}", err);
            return winerr!(E_FAIL);
        }

        let data = Box::from_raw(data);

        match *data {
            1 => Ok(SystemTheme::Light),
            _ => Ok(SystemTheme::Dark),
        }
    }
}

pub fn get_settings_string(key: SettingsKey) -> Result<OsString> {
    get_string_value(settings_root()?, key.reg_key())
}

pub fn set_settings_string(key: SettingsKey, value: &str) -> Result<()> {
    set_string_value(settings_root()?, key.reg_key(), value)
}

pub fn get_settings_u32(key: SettingsKey) -> Result<u32> {
    get_u32_value(settings_root()?, key.reg_key())
}

pub fn set_settings_u32(key: SettingsKey, value: u32) -> Result<()> {
    set_u32_value(settings_root()?, key.reg_key(), value)
}

fn settings_root() -> Result<HKEY> {
    HKEY_CURRENT_USER.create_subkey(SETTINGS_REG_PATH)
}

fn get_string_value(key: HKEY, name: &str) -> Result<OsString> {
    unsafe {
        let name = name.to_pcwstr();
        let mut data_size: u32 = 0;
        let err = RegGetValueW(
            key,
            None,
            *name,
            RRF_RT_REG_SZ,
            None,
            None,
            Some(&mut data_size),
        );

        if err != ERROR_SUCCESS {
            let err = GetLastError().0;
            debug!("error: {}", err);
            return winerr!(E_FAIL);
        }

        let u16_size = data_size as usize / std::mem::size_of::<u16>();
        let mut data: Vec<u16> = vec![0; u16_size];
        let err = RegGetValueW(
            key,
            None,
            *name,
            RRF_RT_REG_SZ,
            None,
            Some(data.as_ptr() as *mut c_void),
            Some(&mut data_size),
        );

        if err != ERROR_SUCCESS {
            return winerr!(E_FAIL);
        }

        let data_size = data_size as usize;
        let u16_size = data_size / std::mem::size_of::<u16>();
        data.truncate(u16_size - 1);

        Ok(OsString::from_wide(&data))
    }
}

fn set_string_value(hkey: HKEY, name: &str, value: &str) -> Result<()> {
    let name = name.to_pcwstr();
    let value = value.to_wide_bytes_nul();
    let err = unsafe { RegSetValueExW(hkey, *name, 0, REG_SZ, Some(&value)) };
    check_win32error!(err)
}

fn get_u32_value(hkey: HKEY, name: &str) -> Result<u32> {
    unsafe {
        let name = name.to_pcwstr();
        let mut data = 0u32;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let err = RegGetValueW(
            hkey,
            None,
            *name,
            RRF_RT_REG_DWORD,
            None,
            Some(data as *mut c_void),
            Some(&mut data_size),
        );
        if err != ERROR_SUCCESS {
            let err = GetLastError().0;
            debug!("error: {}", err);
            return winerr!(E_FAIL);
        }
        Ok(data)
    }
}

fn set_u32_value(hkey: HKEY, name: &str, value: u32) -> Result<()> {
    let name = name.to_pcwstr();
    let value = value.to_le_bytes();
    let err =
        unsafe { RegSetValueExW(hkey, *name, 0, REG_DWORD, Some(&value)) };
    check_win32error!(err)
}

fn delete_settings_value(name: &str) -> Result<()> {
    unsafe {
        let name = name.to_pcwstr();
        let err = RegDeleteValueW(settings_root()?, *name);
        if err != ERROR_SUCCESS {
            let err = GetLastError().0;
            debug!("Error deleting key: {}", err);
            return winerr!(E_FAIL);
        }
        Ok(())
    }
}

pub fn can_attach_in_debug() -> Result<()> {
    unsafe {
        let hkey = settings_root()?;
        let name = SettingsKey::DebugLock.reg_key().to_pcwstr();
        let data = 0u32;
        let mut data_size = std::mem::size_of::<u32>() as u32;
        let err = RegGetValueW(
            hkey,
            None,
            *name,
            RRF_RT_REG_DWORD,
            None,
            Some(data as *mut c_void),
            Some(&mut data_size),
        );
        if err == ERROR_FILE_NOT_FOUND {
            let value = 0u32.to_le_bytes();
            match RegSetValueExW(hkey, *name, 0, REG_DWORD, Some(&value)) {
                ERROR_SUCCESS => Ok(()),
                _ => winerr!(E_FAIL),
            }
        } else {
            winerr!(E_FAIL)
        }
    }
}

#[cfg(debug_assertions)]
mod tests {
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn set_and_get_settings_string() {
        let key = settings_root().unwrap();
        let name = "test_string";
        let value = "Tester 123";
        assert!(set_string_value(key, name, value).is_ok());
        let retrieved = get_string_value(key, name).unwrap();
        assert_eq!(retrieved, OsString::from(value));
        assert!(delete_settings_value(name).is_ok());
    }

    #[test]
    fn set_and_get_settings_u32() {
        let key = settings_root().unwrap();
        let name = "test_u32";
        let value = 0800092000;
        assert!(set_u32_value(key, name, value).is_ok());
        let retrieved = get_u32_value(key, name).unwrap();
        assert_eq!(retrieved, value);
        assert!(delete_settings_value(name).is_ok());
    }

    #[test]
    fn can_get_system_theme() {
        let theme = get_system_theme();
        assert!(theme.is_ok());
        println!("System theme is: {:?}", theme.unwrap());
    }
}
