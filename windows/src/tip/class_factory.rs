#![allow(non_upper_case_globals)]

use core::ffi::c_void;
use core::option::Option;
use log::debug;
use log::warn;
use windows::Win32::Foundation::E_UNEXPECTED;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use windows::core::implement;
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

use crate::reg::guids::*;
use crate::tip::text_service;
use crate::tip::text_service::TextService;
use crate::utils::WinGuid;
use crate::DllModule;

#[implement(IClassFactory)]
pub struct KhiinClassFactory {
    dll_ref_count: Arc<AtomicUsize>,
}

impl KhiinClassFactory {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        let old_dll_refs = DllModule::global().add_ref();

        debug!("Created ClassFactory - {} refs", old_dll_refs + 1);

        KhiinClassFactory { dll_ref_count }
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

        if *riid != IID_ITfTextInputProcessor {
            warn!(
                "KhiinClassFactory: Unexpected IID Requested: {}",
                riid.to_string().unwrap_or_default()
            );

            return Err(Error::from(E_NOINTERFACE));
        }

        let text_service = TextService::new(self.dll_ref_count.clone());

        if text_service.is_err() {
            warn!(
                "KhiinClassFactory: Unable to create TextService: {}",
                riid.to_string().unwrap_or_default()
            );

            return Err(Error::from(E_UNEXPECTED));
        }

        let text_service = text_service.unwrap();

        *ppvobject = unsafe { core::mem::transmute(text_service) };

        Ok(())
    }

    fn LockServer(&self, flock: BOOL) -> Result<()> {
        debug!("Lock server: {}", flock.as_bool());

        if flock.as_bool() {
            self.dll_ref_count.fetch_add(1, Ordering::SeqCst);
        } else {
            self.dll_ref_count.fetch_sub(1, Ordering::SeqCst);
        }

        Ok(())
    }
}
