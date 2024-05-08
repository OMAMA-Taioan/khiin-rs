#![allow(non_upper_case_globals)]

use core::ffi::c_void;
use core::option::Option;
use log::debug;
use log::warn;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::Interface;
use windows::core::Error;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::CLASS_E_NOAGGREGATION;
use windows::Win32::Foundation::E_NOINTERFACE;
use windows::Win32::System::Com::IClassFactory;
use windows::Win32::System::Com::IClassFactory_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;

use crate::dll::DllModule;
use crate::tip::TextService;
use crate::utils::WinGuid;

#[implement(IClassFactory)]
pub struct KhiinClassFactory;

impl KhiinClassFactory {
    pub fn new() -> Self {
        KhiinClassFactory
    }
}

impl IClassFactory_Impl for KhiinClassFactory {
    fn CreateInstance(
        &self,
        punkouter: Option<&IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> Result<()> {
        let riid = &unsafe { *riid };
        let ppvobject = unsafe { &mut *ppvobject };

        debug!(
            "Trying to create instance of: {}",
            riid.to_string().unwrap_or_default()
        );

        *ppvobject = std::ptr::null_mut();

        if punkouter.is_some() {
            return Err(Error::from(CLASS_E_NOAGGREGATION));
        }

        if *riid != ITfTextInputProcessor::IID {
            warn!(
                "KhiinClassFactory: Unexpected IID Requested: {}",
                riid.to_string().unwrap_or_default()
            );

            return Err(Error::from(E_NOINTERFACE));
        }

        let text_service: ITfTextInputProcessor = TextService::new()?.into();

        let it: &TextService = unsafe { text_service.as_impl() };
        it.set_this(text_service.clone());

        *ppvobject = unsafe { core::mem::transmute(text_service) };

        Ok(())
    }

    fn LockServer(&self, flock: BOOL) -> Result<()> {
        debug!("Lock server: {}", flock.as_bool());

        match flock.as_bool() {
            true => DllModule::global().add_ref(),
            false => DllModule::global().release(),
        };

        Ok(())
    }
}
