use core::option::Option;
use std::cell::Cell;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
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
    threadmgr: Cell<*const ITfThreadMgr>,
}

impl TextService {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        TextService {
            dll_ref_count,
            disp_attrs: DisplayAttributes::new(),
            clientid: Cell::new(TF_CLIENTID_NULL),
            dwflags: Cell::new(0),
            threadmgr: Cell::new(std::ptr::null()),
        }
    }

    pub fn disp_attrs(&self) -> &DisplayAttributes {
        &self.disp_attrs
    }

    pub fn clientid(&self) -> u32 {
        self.clientid.get()
    }

    fn activate(&self, threadmgr: &ITfThreadMgr) -> Result<()> {
        KeyEventSink::advise(self, threadmgr)?;
        Ok(())
    }

    fn deactivate(&self, threadmgr: &ITfThreadMgr) -> Result<()> {
        KeyEventSink::unadvise(self, threadmgr)?;
        Ok(())
    }

    fn threadmgr(&self) -> &ITfThreadMgr {
        unsafe { &*self.threadmgr.get() }
    }
}

impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        self.ActivateEx(ptim, tid, 0)
    }

    fn Deactivate(&self) -> Result<()> {
        self.deactivate(self.threadmgr())?;
        self.threadmgr.set(std::ptr::null());
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
            Some(threadmgr) => {
                self.threadmgr.set(&*threadmgr);
                self.activate(threadmgr)
            },
            None => Ok(()),
        }
    }
}
