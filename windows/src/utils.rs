use std::ffi::c_ushort;

use windows::core::Error;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::Com::StringFromGUID2;
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;

pub trait WinGuid {
    fn to_string(&self) -> Result<String>;
}

impl WinGuid for GUID {
    fn to_string(&self) -> Result<String> {
        let mut buffer = [0u16; 39];
        let len = unsafe { StringFromGUID2(self, &mut buffer) };
        if len == 0 {
            return Err(Error::from(E_FAIL));
        }
        let result = String::from_utf16(&buffer[..len as usize]);
        match result {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::from(E_FAIL)),
        }
    }
}

pub trait GetPath {
    fn get_path(&self) -> Result<String>;
}

impl GetPath for HINSTANCE {
    fn get_path(&self) -> Result<String> {
        let mut buffer = [0u16; MAX_PATH as usize];
        let len = unsafe { GetModuleFileNameW(*self, &mut buffer) };
        if len == 0 {
            return Err(Error::from(E_FAIL));
        }
        let result = String::from_utf16(&buffer[..len as usize]);
        match result {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::from(E_FAIL)),
        }
    }
}

pub trait WinString {
    // Use str.to_wide_bytes().as_slice()
    // for COM methods that take `const BYTE *` of a UTF-16 string
    fn to_wide_bytes(self) -> Vec<u8>;
}

impl WinString for &str {
    fn to_wide_bytes(self) -> Vec<u8> {
        let mut s = String::from(self);
        s.push_str("\0");
        s.encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>()
    }
}

pub fn lo_word(value: u32) -> u16 {
    (value & 0xffff) as u16
}

pub fn hi_word(value: u32) -> u16 {
    ((value >> 16) & 0xffff) as u16
}

pub fn lo_byte(value: u16) -> u8 {
    (value & 0xff) as u8
}

pub fn hi_byte(value: u16) -> u8 {
    ((value >> 8) & 0xff) as u8
}

#[macro_export]
macro_rules! pcwstr {
    ($s:expr) => {{
        let s: &str = $s;
        windows::core::PCWSTR(windows::core::HSTRING::from(s).as_ptr())
    }};
}

#[macro_export]
macro_rules! check_win32error {
    ($result:ident) => {
        if $result.is_ok() {
            Ok(())
        } else {
            Err(Error::from($result.to_hresult()))
        }
    };
    ($result:ident,$return:ident) => {
        if $result.is_ok() {
            Ok($return)
        } else {
            Err(Error::from($result.to_hresult()))
        }
    };
}
