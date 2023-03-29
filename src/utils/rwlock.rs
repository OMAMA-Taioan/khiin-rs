use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_UNEXPECTED;

pub struct RwLock<T: Clone> {
    value: std::sync::RwLock<T>
}

impl<T: Clone> RwLock<T> {
    pub fn new(value: T) -> Self {
        RwLock { value: std::sync::RwLock::new(value) }
    }

    pub fn get(&self) -> Result<T> {
        if let Ok(reader) = self.value.try_read() {
            Ok((*reader).clone())
        } else {
            Err(Error::from(E_UNEXPECTED))
        }
    }

    pub fn set(&self, value: T) -> Result<()> {
        if let Ok(mut writer) = self.value.try_write() {
            *writer = value;
            Ok(())
        } else {
            Err(Error::from(E_UNEXPECTED))
        }
    }
}
