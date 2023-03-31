use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink_Impl;
use windows::Win32::UI::TextServices::ITfKeyEventSink;
use windows::Win32::UI::TextServices::ITfLangBarItemButton;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::TextServices::GUID_COMPARTMENT_KEYBOARD_OPENCLOSE;

use crate::tip::display_attributes::DisplayAttributes;
use crate::tip::engine_mgr::EngineMgr;
use crate::tip::key_event_sink::KeyEventSink;
use crate::utils::arc_lock::ArcLock;

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

    // After the TextService is pinned in COM (by going `.into()`
    // the ITfTextInputProcessor), set `this` as a COM smart pointer
    // to self. All other impls that need TextService should recieve
    // a clone of `this`, and cast it to &TextService using `.as_impl()`
    // We must destroy this clone in `Deactivate` by setting `this` back
    // to `None`.
    this: RefCell<Option<ITfTextInputProcessor>>,
    clientid: ArcLock<u32>,
    dwflags: ArcLock<u32>,
    enabled: ArcLock<bool>,
    disp_attrs: DisplayAttributes,
    engine: EngineMgr,
    threadmgr: RefCell<Option<ITfThreadMgr>>,
    open_close_compartment: RefCell<Option<Compartment>>,
    key_event_sink: RefCell<Option<ITfKeyEventSink>>,
    lang_bar_indicator: RefCell<Option<ITfLangBarItemButton>>,
}

impl TextService {
    pub fn new(dll_ref_count: Arc<AtomicUsize>) -> Self {
        TextService {
            dll_ref_count,
            this: RefCell::new(None),
            disp_attrs: DisplayAttributes::new(),
            clientid: ArcLock::new(TF_CLIENTID_NULL),
            dwflags: ArcLock::new(0),
            enabled: ArcLock::new(false),
            engine: EngineMgr::new(),
            threadmgr: RefCell::new(None),
            open_close_compartment: RefCell::new(None),
            key_event_sink: RefCell::new(None),
            lang_bar_indicator: RefCell::new(None),
        }
    }

    pub fn disp_attrs(&self) -> &DisplayAttributes {
        &self.disp_attrs
    }

    pub fn clientid(&self) -> Result<u32> {
        self.clientid.get()
    }

    pub fn enabled(&self) -> Result<bool> {
        self.enabled.get()
    }

    pub fn toggle_enabled(&self) -> Result<()> {
        Ok(())
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
        self.init_key_event_sink()?;
        self.init_lang_bar_indicator()?;
        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        self.deinit_lang_bar_indicator()?;
        self.deinit_key_event_sink()?;
        self.deinit_open_close_compartment()?;
        Ok(())
    }

    fn init_open_close_compartment(&self) -> Result<()> {
        let unknown: IUnknown = self.threadmgr().cast()?;
        let compartment = Compartment::new(
            unknown,
            self.clientid.get()?,
            GUID_COMPARTMENT_KEYBOARD_OPENCLOSE,
            false,
        )?;
        self.open_close_compartment.replace(Some(compartment));
        self.set_open_close_compartment(true)?;
        Ok(())
    }

    fn deinit_open_close_compartment(&self) -> Result<()> {
        let _ = self.set_open_close_compartment(false);
        self.open_close_compartment.replace(None);
        Ok(())
    }

    fn init_key_event_sink(&self) -> Result<()> {
        let sink = KeyEventSink::new(self.this(), self.threadmgr());
        let sink: ITfKeyEventSink = sink.into();
        self.key_event_sink.replace(Some(sink));
        self.key_event_sink().as_impl().advise()
    }

    fn deinit_key_event_sink(&self) -> Result<()> {
        let _ = self.key_event_sink().as_impl().unadvise();
        self.key_event_sink.replace(None);
        Ok(())
    }

    fn init_lang_bar_indicator(&self) -> Result<()> {
        let indicator = LangBarIndicator::new(self.this(), self.threadmgr())?;
        self.lang_bar_indicator.replace(Some(indicator));
        Ok(())
    }

    fn deinit_lang_bar_indicator(&self) -> Result<()> {
        let button = self.lang_bar_indicator().clone();
        let indicator = button.clone();
        let indicator = indicator.as_impl();
        let _ = indicator.shutdown(button);
        // logging?
        self.lang_bar_indicator.replace(None);
        Ok(())
    }

    fn lang_bar_indicator(&self) -> ITfLangBarItemButton {
        self.lang_bar_indicator.borrow().clone().unwrap()
    }

    fn key_event_sink(&self) -> ITfKeyEventSink {
        self.key_event_sink.borrow().clone().unwrap()
    }

    fn set_open_close_compartment(&self, value: bool) -> Result<()> {
        let x = self.open_close_compartment.borrow_mut();
        let x = x.as_ref().unwrap();
        x.set_bool(value)
    }

    fn get_open_close_compartment(&self) -> Result<bool> {
        let x = self.open_close_compartment.borrow_mut();
        let x = x.as_ref().unwrap();
        x.get_bool()
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
        if self.ActivateEx(ptim, tid, 0).is_err() {
            self.deactivate()
        } else {
            Ok(())
        }
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
                self.threadmgr.replace(Some(threadmgr));
                self.activate()
            }
            None => Ok(()),
        }
    }
}
