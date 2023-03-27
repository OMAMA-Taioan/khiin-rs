use core::option::Option;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;

#[implement(ITfTextInputProcessorEx, ITfTextInputProcessor)]
#[allow(dead_code)]
pub struct TextService {
    pub dll_ref_count: Arc<AtomicUsize>,
}

impl ITfTextInputProcessor_Impl for TextService {
    fn Activate(&self, ptim: Option<&ITfThreadMgr>, tid: u32) -> Result<()> {
        self.ActivateEx(ptim, tid, 0)
    }

    fn Deactivate(&self) -> Result<()> {
        Ok(())
    }
}

impl ITfTextInputProcessorEx_Impl for TextService {
    fn ActivateEx(
        &self,
        ptim: Option<&ITfThreadMgr>,
        _tid: u32,
        _dwflags: u32,
    ) -> Result<()> {
        match ptim {
            Some(thread_mgr) => self.activate(thread_mgr),
            None => Ok(())
        }
    }
}

impl TextService {
    fn activate(&self, thread_mgr: &ITfThreadMgr) -> Result<()> {
        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        Ok(())
    }
}
