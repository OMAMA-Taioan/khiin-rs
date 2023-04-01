use std::cell::RefCell;

use windows::Win32::UI::TextServices::ITfComposition;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::ComInterface;
use windows::core::IUnknown;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::UI::TextServices::CLSID_TF_CategoryMgr;
use windows::Win32::UI::TextServices::ITfCategoryMgr;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink_Impl;
use windows::Win32::UI::TextServices::ITfCompositionSink;
use windows::Win32::UI::TextServices::ITfCompositionSink_Impl;
use windows::Win32::UI::TextServices::ITfKeyEventSink;
use windows::Win32::UI::TextServices::ITfLangBarItemButton;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::TextServices::ITfThreadMgrEventSink;
use windows::Win32::UI::TextServices::GUID_COMPARTMENT_KEYBOARD_DISABLED;
use windows::Win32::UI::TextServices::GUID_COMPARTMENT_KEYBOARD_OPENCLOSE;

use crate::dll::DllModule;
use crate::reg::guids::GUID_CONFIG_CHANGED_COMPARTMENT;
use crate::reg::guids::GUID_DISPLAY_ATTRIBUTE_CONVERTED;
use crate::reg::guids::GUID_DISPLAY_ATTRIBUTE_FOCUSED;
use crate::reg::guids::GUID_DISPLAY_ATTRIBUTE_INPUT;
use crate::reg::guids::GUID_RESET_USERDATA_COMPARTMENT;
use crate::tip::compartment::Compartment;
use crate::tip::display_attributes::DisplayAttributes;
use crate::tip::engine_mgr::EngineMgr;
use crate::tip::key_event_sink::KeyEventSink;
use crate::tip::lang_bar_indicator::LangBarIndicator;
use crate::tip::sink_mgr::SinkMgr;
use crate::ui::popup_menu::PopupMenu;
use crate::ui::window::Window;
use crate::utils::arc_lock::ArcLock;
use crate::utils::win::co_create_inproc;

use super::preserved_key_mgr::PreservedKeyMgr;
use super::thread_mgr_event_sink::ThreadMgrEventSink;

const TF_CLIENTID_NULL: u32 = 0;
const TF_INVALID_GUIDATOM: u32 = 0;

#[implement(
    ITfTextInputProcessorEx,
    ITfTextInputProcessor,
    ITfCompartmentEventSink,
    ITfCompositionSink
)]
pub struct TextService {
    // After the TextService is pinned in COM (by going `.into()`
    // the ITfTextInputProcessor), set `this` as a COM smart pointer
    // to self. All other impls that need TextService should recieve
    // a clone of `this`, and cast it to &TextService using `.as_impl()`
    this: RefCell<Option<ITfTextInputProcessor>>,

    // Given by TSF
    threadmgr: RefCell<Option<ITfThreadMgr>>,
    clientid: ArcLock<u32>,
    dwflags: ArcLock<u32>,

    // State flag
    enabled: ArcLock<bool>,

    // Sinks (event handlers)
    key_event_sink: RefCell<Option<ITfKeyEventSink>>,
    threadmgr_event_sink: RefCell<Option<ITfThreadMgrEventSink>>,
    threadmgr_event_sink_sinkmgr: RefCell<SinkMgr<ITfThreadMgrEventSink>>,

    // Compartments
    open_close_compartment: RefCell<Option<Compartment>>,
    open_close_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    config_compartment: RefCell<Option<Compartment>>,
    config_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    userdata_compartment: RefCell<Option<Compartment>>,
    userdata_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    kbd_disabled_compartment: RefCell<Option<Compartment>>,
    kbd_disabled_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    // UI elements
    disp_attrs: DisplayAttributes,
    input_attr_guidatom: ArcLock<u32>,
    converted_attr_guidatom: ArcLock<u32>,
    focused_attr_guidatom: ArcLock<u32>,
    lang_bar_indicator: RefCell<Option<ITfLangBarItemButton>>,
    preserved_key_mgr: RefCell<Option<PreservedKeyMgr>>,

    // Data
    engine: EngineMgr,
}

impl TextService {
    pub fn new() -> Self {
        TextService {
            this: RefCell::new(None),
            threadmgr: RefCell::new(None),
            clientid: ArcLock::new(TF_CLIENTID_NULL),
            dwflags: ArcLock::new(0),
            enabled: ArcLock::new(false),
            key_event_sink: RefCell::new(None),
            threadmgr_event_sink: RefCell::new(None),
            threadmgr_event_sink_sinkmgr: RefCell::new(SinkMgr::<
                ITfThreadMgrEventSink,
            >::new()),
            open_close_compartment: RefCell::new(None),
            open_close_sinkmgr: RefCell::new(
                SinkMgr::<ITfCompartmentEventSink>::new(),
            ),
            config_compartment: RefCell::new(None),
            config_sinkmgr: RefCell::new(
                SinkMgr::<ITfCompartmentEventSink>::new(),
            ),
            userdata_compartment: RefCell::new(None),
            userdata_sinkmgr: RefCell::new(
                SinkMgr::<ITfCompartmentEventSink>::new(),
            ),
            kbd_disabled_compartment: RefCell::new(None),
            kbd_disabled_sinkmgr: RefCell::new(SinkMgr::<
                ITfCompartmentEventSink,
            >::new()),
            preserved_key_mgr: RefCell::new(None),
            disp_attrs: DisplayAttributes::new(),
            input_attr_guidatom: ArcLock::new(TF_INVALID_GUIDATOM),
            converted_attr_guidatom: ArcLock::new(TF_INVALID_GUIDATOM),
            focused_attr_guidatom: ArcLock::new(TF_INVALID_GUIDATOM),
            lang_bar_indicator: RefCell::new(None),
            engine: EngineMgr::new(),
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

    pub fn threadmgr(&self) -> ITfThreadMgr {
        self.threadmgr.borrow().clone().unwrap()
    }

    pub fn categorymgr(&self) -> Result<ITfCategoryMgr> {
        co_create_inproc(&CLSID_TF_CategoryMgr)
    }

    fn activate(&self) -> Result<()> {
        DllModule::global().add_ref();
        PopupMenu::register_class(DllModule::global().module);
        self.init_lang_bar_indicator()?;
        self.init_threadmgr_event_sink()?;
        self.init_open_close_compartment()?;
        self.init_config_compartment()?;
        self.init_userdata_compartment()?;
        self.init_kbd_disabled_compartment()?;
        self.init_preserved_key_mgr()?;
        self.init_key_event_sink()?;
        self.init_display_attributes()?;
        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        let _ = self.deinit_display_attributes();
        let _ = self.deinit_key_event_sink();
        let _ = self.deinit_preserved_key_mgr();
        let _ = self.deinit_kbd_disabled_compartment();
        let _ = self.deinit_userdata_compartment();
        let _ = self.deinit_config_compartment();
        let _ = self.deinit_open_close_compartment();
        let _ = self.deinit_threadmgr_event_sink();
        let _ = self.deinit_lang_bar_indicator();
        PopupMenu::unregister_class(DllModule::global().module);
        DllModule::global().release();
        Ok(())
    }

    // compartments & sinkmgrs
    fn init_compartment(
        &self,
        guid: GUID,
        compartment: &RefCell<Option<Compartment>>,
        sinkmgr: &RefCell<SinkMgr<ITfCompartmentEventSink>>,
    ) -> Result<()> {
        let comp = Compartment::from(self.threadmgr(), self.clientid()?, guid)?;
        compartment.replace(Some(comp.clone()));

        let mut sinkmgr = sinkmgr.borrow_mut();
        let punk: IUnknown = comp.compartment()?.cast()?;
        let this: ITfCompartmentEventSink = self.this().cast()?;
        sinkmgr.advise(punk, this)
    }

    fn deinit_compartment(
        &self,
        compartment: &RefCell<Option<Compartment>>,
        sinkmgr: &RefCell<SinkMgr<ITfCompartmentEventSink>>,
    ) -> Result<()> {
        sinkmgr.borrow_mut().unadvise()?;
        compartment.replace(None);
        Ok(())
    }

    // open-close compartment
    fn init_open_close_compartment(&self) -> Result<()> {
        self.init_compartment(
            GUID_COMPARTMENT_KEYBOARD_OPENCLOSE,
            &self.open_close_compartment,
            &self.open_close_sinkmgr,
        )?;
        self.set_open_close_compartment(true)
    }

    fn deinit_open_close_compartment(&self) -> Result<()> {
        let _ = self.set_open_close_compartment(false);
        self.deinit_compartment(
            &self.open_close_compartment,
            &self.open_close_sinkmgr,
        )
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

    // config compartment
    fn init_config_compartment(&self) -> Result<()> {
        self.init_compartment(
            GUID_CONFIG_CHANGED_COMPARTMENT,
            &self.config_compartment,
            &self.config_sinkmgr,
        )
    }

    fn deinit_config_compartment(&self) -> Result<()> {
        self.deinit_compartment(&self.config_compartment, &self.config_sinkmgr)
    }

    // userdata compartment
    fn init_userdata_compartment(&self) -> Result<()> {
        self.init_compartment(
            GUID_RESET_USERDATA_COMPARTMENT,
            &self.userdata_compartment,
            &self.userdata_sinkmgr,
        )
    }

    fn deinit_userdata_compartment(&self) -> Result<()> {
        self.deinit_compartment(
            &self.userdata_compartment,
            &self.userdata_sinkmgr,
        )
    }

    // keyboard disabled compartment
    fn init_kbd_disabled_compartment(&self) -> Result<()> {
        self.init_compartment(
            GUID_COMPARTMENT_KEYBOARD_DISABLED,
            &self.kbd_disabled_compartment,
            &self.kbd_disabled_sinkmgr,
        )
    }

    fn deinit_kbd_disabled_compartment(&self) -> Result<()> {
        self.deinit_compartment(
            &self.kbd_disabled_compartment,
            &self.kbd_disabled_sinkmgr,
        )
    }

    // key event sink
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

    fn key_event_sink(&self) -> ITfKeyEventSink {
        self.key_event_sink.borrow().clone().unwrap()
    }

    // threadmgr event sink
    fn init_threadmgr_event_sink(&self) -> Result<()> {
        let tip: ITfTextInputProcessor = self.this();
        self.threadmgr_event_sink
            .replace(Some(ThreadMgrEventSink::new(tip).into()));
        let sink = self.threadmgr_event_sink.borrow().clone().unwrap();
        let punk: IUnknown = self.threadmgr().cast()?;
        self.threadmgr_event_sink_sinkmgr
            .borrow_mut()
            .advise(punk, sink)
    }

    fn deinit_threadmgr_event_sink(&self) -> Result<()> {
        self.threadmgr_event_sink_sinkmgr.borrow_mut().unadvise()?;
        self.threadmgr_event_sink.replace(None);
        Ok(())
    }

    // preseved key manager
    fn init_preserved_key_mgr(&self) -> Result<()> {
        self.preserved_key_mgr
            .replace(Some(PreservedKeyMgr::new(self.this())));

        Ok(())
    }

    fn deinit_preserved_key_mgr(&self) -> Result<()> {
        Ok(())
    }

    // language bar indicator
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

    // display attributes (underlines)
    fn init_display_attributes(&self) -> Result<()> {
        let categorymgr = self.categorymgr()?;
        unsafe {
            self.input_attr_guidatom.set(
                categorymgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_INPUT)?,
            )?;
            self.converted_attr_guidatom.set(
                categorymgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_CONVERTED)?,
            )?;
            self.focused_attr_guidatom.set(
                categorymgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_FOCUSED)?,
            )
        }
    }

    fn deinit_display_attributes(&self) -> Result<()> {
        Ok(())
    }
}
//+---------------------------------------------------------------------------
//
// ITfCompartmentEventSink
//
//----------------------------------------------------------------------------

impl ITfCompartmentEventSink_Impl for TextService {
    fn OnChange(&self, rguid: *const GUID) -> Result<()> {
        let rguid = unsafe { *rguid };

        match rguid {
            GUID_COMPARTMENT_KEYBOARD_OPENCLOSE => Ok(()),
            GUID_CONFIG_CHANGED_COMPARTMENT => Ok(()),
            GUID_RESET_USERDATA_COMPARTMENT => Ok(()),
            GUID_COMPARTMENT_KEYBOARD_DISABLED => Ok(()),
            _ => Ok(()),
        }
    }
}

//+---------------------------------------------------------------------------
//
// ITfCompositionSink
//
//----------------------------------------------------------------------------

impl ITfCompositionSink_Impl for TextService {
    fn OnCompositionTerminated(
        &self,
        ecwrite: u32,
        pcomposition: Option<&ITfComposition>,
    ) -> Result<()> {
        Ok(())
    }
}

//+---------------------------------------------------------------------------
//
// ITfTextInputProcessor
//
//----------------------------------------------------------------------------

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

//+---------------------------------------------------------------------------
//
// ITfTextInputProcessorEx
//
//----------------------------------------------------------------------------

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
