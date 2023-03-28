use std::cell::Cell;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use windows::core::implement;
use windows::core::ComInterface;
use windows::core::Interface;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;

use crate::tip::display_attributes::DisplayAttributes;
use crate::tip::key_event_sink::KeyEventSink;
use crate::utils::com_ptr_cell::ComPtrCell;

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
    // set on activation and borrowed as a reference for use in
    // all other functions.
    threadmgr: ComPtrCell<ITfThreadMgr>,
}

impl TextService {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        TextService {
            dll_ref_count,
            disp_attrs: DisplayAttributes::new(),
            clientid: Cell::new(TF_CLIENTID_NULL),
            dwflags: Cell::new(0),
            threadmgr: ComPtrCell::new(),
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

    fn deactivate(&self) -> Result<()> {
        if let Some(threadmgr) = self.threadmgr() {
            KeyEventSink::unadvise(self, threadmgr)?;
        }

        Ok(())
    }

    fn threadmgr(&self) -> Option<&ITfThreadMgr> {
        self.threadmgr.get()
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
            Some(threadmgr) => {
                self.threadmgr.set(threadmgr);
                self.activate(threadmgr)
            }
            None => Ok(()),
        }
    }
}
