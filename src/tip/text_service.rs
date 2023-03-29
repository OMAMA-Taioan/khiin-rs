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
use crate::tip::engine_mgr::EngineMgr;
use crate::tip::key_event_sink::KeyEventSink;
use crate::utils::rwlock::RwLock;

const TF_CLIENTID_NULL: u32 = 0;

#[implement(ITfTextInputProcessorEx, ITfTextInputProcessor)]
pub struct TextService {
    dll_ref_count: Arc<AtomicUsize>,
    disp_attrs: DisplayAttributes,
    clientid: RwLock<u32>,
    dwflags: RwLock<u32>,
    threadmgr: RwLock<Option<ITfThreadMgr>>,
    enabled: RwLock<bool>,
    engine: EngineMgr,
}

impl TextService {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        TextService {
            dll_ref_count,
            disp_attrs: DisplayAttributes::new(),
            clientid: RwLock::new(TF_CLIENTID_NULL),
            dwflags: RwLock::new(0),
            threadmgr: RwLock::new(None),
            enabled: RwLock::new(false),
            engine: EngineMgr::new(),
        }
    }

    pub fn disp_attrs(&self) -> &DisplayAttributes {
        &self.disp_attrs
    }

    pub fn clientid(&self) -> u32 {
        self.clientid.get().unwrap()
    }

    pub fn enabled(&self) -> bool {
        self.enabled.get().unwrap()
    }

    pub fn engine(&self) -> &EngineMgr {
        &self.engine
    }

    fn threadmgr(&self) -> ITfThreadMgr {
        self.threadmgr.get().unwrap().unwrap()
    }

    fn activate(&self) -> Result<()> {
        KeyEventSink::advise(self, self.threadmgr().clone())?;

        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        let threadmgr = self.threadmgr.get()?;
        if let Some(threadmgr) = threadmgr {
            KeyEventSink::unadvise(self, threadmgr)?;
        }
        Ok(())
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
        self.clientid.set(tid)?;
        self.dwflags.set(dwflags)?;



        match ptim {
            Some(threadmgr) => {
                let threadmgr = threadmgr.clone();
                self.threadmgr.set(Some(threadmgr))?;
                self.activate()
            }
            None => Ok(()),
        }
    }
}
