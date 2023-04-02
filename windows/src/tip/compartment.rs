use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Weak;

use windows::Win32::Foundation::E_FAIL;
use windows::core::implement;
use windows::core::ComInterface;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::ERROR_INVALID_PARAMETER;
use windows::Win32::System::Com::VARIANT;
use windows::Win32::System::Com::VT_I4;
use windows::Win32::UI::TextServices::ITfCompartmentMgr;
use windows::Win32::UI::TextServices::{ITfCompartment, ITfThreadMgr};

use crate::winerr;

#[derive(Clone)]
pub struct Compartment {
    manager: Option<ITfCompartmentMgr>,
    clientid: u32,
    guid: GUID,
}

impl Compartment {
    pub fn new() -> Self {
        Self {
            manager: None,
            clientid: 0,
            guid: GUID::zeroed(),
        }
    }

    fn init(
        &mut self,
        provider: IUnknown,
        clientid: u32,
        guid: GUID,
        global: bool,
    ) -> Result<()> {
        if global {
            let threadmgr: ITfThreadMgr = provider.cast()?;
            self.manager = Some(threadmgr.cast()?);
        } else {
            self.manager = Some(provider.cast()?);
        }

        self.clientid = clientid;
        self.guid = guid;
        
        Ok(())
    }

    pub fn init_thread(
        &mut self,
        threadmgr: ITfThreadMgr,
        clientid: u32,
        guid: GUID,
    ) -> Result<()> {
        let unknown: IUnknown = threadmgr.cast()?;
        self.init(unknown, clientid, guid, false)
    }

    pub fn deinit(&mut self) -> Result<()> {
        self.manager = None;
        self.clientid = 0;
        self.guid = GUID::zeroed();
        Ok(())
    }

    pub fn from_void(ptr: *mut c_void) -> Box<Compartment> {
        unsafe { Box::from_raw(ptr as *mut Compartment) }
    }

    pub fn set_bool(&self, value: bool) -> Result<()> {
        self.set_value(value as u32)
    }

    pub fn get_bool(&self) -> Result<bool> {
        Ok(self.get_value()? != 0)
    }

    pub fn set_value(&self, value: u32) -> Result<()> {
        let mut variant = VARIANT::default();
        unsafe {
            (*variant.Anonymous.Anonymous).vt = VT_I4;
            (*variant.Anonymous.Anonymous).Anonymous.lVal = value as i32;
            self.compartment()?.SetValue(self.clientid, &variant)
        }
    }

    pub fn get_value(&self) -> Result<u32> {
        unsafe {
            let variant = self.compartment()?.GetValue()?;
            if variant.Anonymous.Anonymous.vt == VT_I4 {
                Ok(variant.Anonymous.Anonymous.Anonymous.lVal as u32)
            } else {
                winerr!(ERROR_INVALID_PARAMETER)
            }
        }
    }

    pub fn compartment(&self) -> Result<ITfCompartment> {
        match self.manager.clone() {
            Some(manager) => unsafe { manager.GetCompartment(&self.guid) },
            None => winerr!(E_FAIL)
        }
    }
}
