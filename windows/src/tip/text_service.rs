use core::option::Option;
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::os::windows::thread;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use windows::core::Interface;
use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;

use crate::tip::display_attributes::DisplayAttributes;
use crate::tip::key_event_sink::KeyEventSink;

const TF_CLIENTID_NULL: u32 = 0;

#[implement(ITfTextInputProcessorEx, ITfTextInputProcessor)]
pub struct TextService {
    pub dll_ref_count: Arc<AtomicUsize>,
    disp_attrs: DisplayAttributes,
    clientid: Cell<u32>,
    dwflags: Cell<u32>,

    // The ITfThreadMgr comes from TSF in the Activate method
    // We need this throughout the lifetime of the TextService,
    // including in methods where we don't receive it again from Windows
    // (e.g., we are expected to keep a handle to it for our own use)
    // Windows guarantees to not delete it while we are using it
    // However, we don't have a lifetime parameter to refer to,
    // so there is no way to store the reference directly. Instead,
    // we must use an UnsafeCell and the raw pointer. The pointer is
    // set on activation and retrieved as a reference for use in
    // all other functions.
    threadmgr: UnsafeCell<*mut c_void>,
}

impl TextService {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        TextService {
            dll_ref_count,
            disp_attrs: DisplayAttributes::new(),
            clientid: Cell::new(TF_CLIENTID_NULL),
            dwflags: Cell::new(0),
            threadmgr: UnsafeCell::new(std::ptr::null_mut()),
        }
    }

    pub fn disp_attrs(&self) -> &DisplayAttributes {
        &self.disp_attrs
    }

    pub fn clientid(&self) -> u32 {
        self.clientid.get()
    }

    fn activate(&self, threadmgr: &ITfThreadMgr) -> Result<()> {
        self.set_threadmgr(threadmgr);
        KeyEventSink::advise(self, threadmgr)?;
        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        if let Some(threadmgr) = self.threadmgr() {
            KeyEventSink::unadvise(self, threadmgr)?;
        }

        Ok(())
    }

    fn threadmgr(&self) -> Option<&ITfThreadMgr> {
        unsafe {
            ITfThreadMgr::from_raw_borrowed(&*self.threadmgr.get())
        }
    }

    fn set_threadmgr(&self, threadmgr: &ITfThreadMgr) {
        let unsafe_cell = self.threadmgr.get();
        unsafe { *unsafe_cell = threadmgr.clone().into_raw(); }
    }
}

impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        self.ActivateEx(ptim, tid, 0)
    }

    fn Deactivate(&self) -> Result<()> {
        self.deactivate()?;
        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for TextService {
    fn ActivateEx(
        &self,
        ptim: Option<&ITfThreadMgr>,
        tid: u32,
        dwflags: u32,
    ) -> Result<()> {
        self.clientid.set(tid);
        self.dwflags.set(dwflags);

        match ptim {
            Some(threadmgr) => self.activate(threadmgr),
            None => Ok(()),
        }
    }
}
