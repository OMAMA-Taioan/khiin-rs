use windows::core::ComInterface;
use windows::core::Error;
use windows::core::PCWSTR;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Foundation::MAX_PATH;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::StringFromGUID2;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;

pub fn co_create_inproc<T: ComInterface>(clsid: &GUID) -> Result<T> {
    let iface: T =
        unsafe { CoCreateInstance(clsid, None, CLSCTX_INPROC_SERVER)? };
    Ok(iface)
}

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

impl GetPath for HMODULE {
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
    fn to_wide_bytes_nul(&self) -> Vec<u8>;

    fn to_utf16(&self) -> Vec<u16>;
}

impl WinString for &str {
    fn to_wide_bytes_nul(&self) -> Vec<u8> {
        let mut s = String::from(*self);
        s.push_str("\0");
        s.encode_utf16()
            .flat_map(|c| c.to_le_bytes())
            .collect::<Vec<u8>>()
    }

    fn to_utf16(&self) -> Vec<u16> {
        let mut v: Vec<u16> = self.encode_utf16().collect();
        v.push(0);
        v
    }
}

impl WinString for String {
    fn to_wide_bytes_nul(&self) -> Vec<u8> {
        let s = &self[..];
        s.to_wide_bytes_nul()
    }

    fn to_utf16(&self) -> Vec<u16> {
        let s = &self[..];
        s.to_utf16()
    }
}

pub fn _lo_word(value: u32) -> u16 {
    (value & 0xffff) as u16
}

pub fn hi_word(value: u32) -> u16 {
    ((value >> 16) & 0xffff) as u16
}

pub fn lo_byte(value: u16) -> u8 {
    (value & 0xff) as u8
}

pub fn _hi_byte(value: u16) -> u8 {
    ((value >> 8) & 0xff) as u8
}
