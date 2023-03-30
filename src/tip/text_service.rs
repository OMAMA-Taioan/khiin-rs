use std::cell::Cell;
use std::cell::RefCell;
use std::ffi::c_void;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::Error;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::TRUE;
use windows::Win32::UI::TextServices::ITfCompartment;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::TextServices::GUID_COMPARTMENT_KEYBOARD_OPENCLOSE;

use crate::tip::display_attributes::DisplayAttributes;
use crate::tip::engine_mgr::EngineMgr;
use crate::tip::key_event_sink::KeyEventSink;

use super::compartment::Compartment;
use super::lang_bar_indicator::LangBarIndicator;

const TF_CLIENTID_NULL: u32 = 0;

#[implement(
    ITfTextInputProcessorEx,
    ITfTextInputProcessor,
    ITfCompartmentEventSink
)]
pub struct TextService {    
    dll_ref_count: Arc<AtomicUsize>,
    disp_attrs: DisplayAttributes,
    clientid: Cell<u32>,
    dwflags: Cell<u32>,
    threadmgr: RefCell<Option<ITfThreadMgr>>,
    enabled: Cell<bool>,
    engine: EngineMgr,
    open_close_compartment: RefCell<Option<Compartment>>,

    // After the TextService is pinned in COM (by going `.into()`
    // the ITfTextInputProcessor), set `this` as a COM smart pointer
    // to self. All other impls that need TextService should recieve
    // a clone of `this`, and cast it to &TextService using `.as_impl()`
    // We must destroy this clone in `Deactivate` by setting `this` back
    // to `None`.
    this: RefCell<Option<ITfTextInputProcessor>>,}

impl TextService {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        TextService {
            dll_ref_count,
            disp_attrs: DisplayAttributes::new(),
            clientid: Cell::new(TF_CLIENTID_NULL),
            dwflags: Cell::new(0),
            threadmgr: RefCell::new(None),
            enabled: Cell::new(false),
            engine: EngineMgr::new(),
            open_close_compartment: RefCell::new(None),
            this: RefCell::new(None),
        }
    }

    pub fn disp_attrs(&self) -> &DisplayAttributes {
        &self.disp_attrs
    }

    pub fn clientid(&self) -> u32 {
        self.clientid.get()
    }

    pub fn enabled(&self) -> bool {
        self.enabled.get()
    }

    pub fn engine(&self) -> &EngineMgr {
        &self.engine
    }

    pub fn set_this(&self, this: ITfTextInputProcessor) {
        self.this.replace(Some(this));
    }

    pub fn this(&self) -> ITfTextInputProcessor {
        self.this.borrow().clone().unwrap()
    }

    fn threadmgr(&self) -> ITfThreadMgr {
        self.threadmgr.borrow().clone().unwrap()
    }

    fn activate(&self) -> Result<()> {
        self.init_open_close_compartment()?;
        self.set_open_close_compartment(true as u32)?;
        KeyEventSink::advise(self.this(), self.threadmgr())?;
        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        let threadmgr = self.threadmgr();
        KeyEventSink::unadvise(self, threadmgr)?;
        self.this.replace(None);
        Ok(())
    }

    fn init_open_close_compartment(&self) -> Result<()> {
        let unknown: IUnknown = self.threadmgr().cast()?;
        let compartment = Compartment::new(
            unknown,
            self.clientid.get(),
            GUID_COMPARTMENT_KEYBOARD_OPENCLOSE,
            false,
        )?;
        self.open_close_compartment.replace(Some(compartment));
        Ok(())
    }

    fn set_open_close_compartment(&self, value: u32) -> Result<()> {
        let x = self.open_close_compartment.borrow_mut();
        let x = x.as_ref().unwrap();
        x.set_value(value)
    }

    fn get_open_close_compartment(&self) -> Result<u32> {
        let x = self.open_close_compartment.borrow_mut();
        let x = x.as_ref().unwrap();
        x.get_value()
    }
}

impl ITfCompartmentEventSink_Impl for TextService {
    fn OnChange(&self, rguid: *const GUID) -> Result<()> {
        let rguid = unsafe { *rguid };
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
        self.clientid.set(tid);
        self.dwflags.set(dwflags);

        match ptim {
            Some(threadmgr) => {
                let threadmgr = threadmgr.clone();
                self.threadmgr.replace(Some(threadmgr));
                self.activate()
            }
            None => Ok(()),
        }
    }
}
