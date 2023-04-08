use windows::core::Result;
use windows::core::HSTRING;
use windows::core::PCWSTR;
use windows::Win32::System::Registry::RegCloseKey;
use windows::Win32::System::Registry::RegCreateKeyExW;
use windows::Win32::System::Registry::RegDeleteTreeW;
use windows::Win32::System::Registry::RegDeleteValueW;
use windows::Win32::System::Registry::RegSetValueExW;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::KEY_READ;
use windows::Win32::System::Registry::KEY_WRITE;
use windows::Win32::System::Registry::REG_OPTION_NON_VOLATILE;
use windows::Win32::System::Registry::REG_SZ;

use crate::check_win32error;
use crate::utils::ToPcwstr;
use crate::utils::WinString;

pub trait Hkey {
    fn create_subkey(&self, subkey: &str) -> Result<HKEY>;
    fn delete_all(&self) -> Result<()>;
    fn delete_tree(&self, subkey: &str) -> Result<()>;
    fn close(&self) -> Result<()>;
    fn set_string_value(&self, name: &str, value: &str) -> Result<()>;
}

impl Hkey for HKEY {
    fn create_subkey(&self, subkey: &str) -> Result<HKEY> {
        let mut ret = HKEY::default();

        let result = unsafe {
            RegCreateKeyExW(
                *self,
                PCWSTR(HSTRING::from(subkey).as_ptr()),
                0,
                PCWSTR::null(),
                REG_OPTION_NON_VOLATILE,
                KEY_READ | KEY_WRITE,
                None,
                &mut ret,
                None,
            )
        };

        check_win32error!(result, ret)
    }

    fn delete_all(&self) -> Result<()> {
        let result = unsafe { RegDeleteValueW(self.clone(), PCWSTR::null()) };
        check_win32error!(result)
    }

    fn delete_tree(&self, subkey: &str) -> Result<()> {
        let subkey = subkey.to_pcwstr();
        let result = unsafe { RegDeleteTreeW(*self, *subkey) };
        check_win32error!(result)
    }

    fn close(&self) -> Result<()> {
        let result = unsafe { RegCloseKey(*self) };
        check_win32error!(result)
    }

    fn set_string_value(&self, name: &str, value: &str) -> Result<()> {
        let result = unsafe {
            RegSetValueExW(
                *self,
                PCWSTR(HSTRING::from(name).as_ptr()),
                0,
                REG_SZ,
                Some(value.to_wide_bytes_nul().as_slice()),
            )
        };

        check_win32error!(result)
    }
}
