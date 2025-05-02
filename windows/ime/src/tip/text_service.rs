use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

use khiin_protos::command::EditState;
use khiin_protos::command::SegmentStatus;
use log::debug as d;
use protobuf::MessageField;
use windows::core::implement;
use windows::core::AsImpl;
use windows::core::IUnknown;
use windows::core::Interface;
use windows::core::Result;
use windows::core::GUID;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::TextServices::CLSID_TF_CategoryMgr;
use windows::Win32::UI::TextServices::IEnumTfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::ITfCategoryMgr;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink;
use windows::Win32::UI::TextServices::ITfCompartmentEventSink_Impl;
use windows::Win32::UI::TextServices::ITfComposition;
use windows::Win32::UI::TextServices::ITfCompositionSink;
use windows::Win32::UI::TextServices::ITfCompositionSink_Impl;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfDisplayAttributeInfo;
use windows::Win32::UI::TextServices::ITfDisplayAttributeProvider;
use windows::Win32::UI::TextServices::ITfDisplayAttributeProvider_Impl;
use windows::Win32::UI::TextServices::ITfKeyEventSink;
use windows::Win32::UI::TextServices::ITfLangBarItemButton;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx;
use windows::Win32::UI::TextServices::ITfTextInputProcessorEx_Impl;
use windows::Win32::UI::TextServices::ITfTextInputProcessor_Impl;
use windows::Win32::UI::TextServices::ITfThreadMgr;
use windows::Win32::UI::TextServices::ITfThreadMgrEventSink;
use windows::Win32::UI::TextServices::ITfUIElement;
use windows::Win32::UI::TextServices::GUID_COMPARTMENT_KEYBOARD_DISABLED;
use windows::Win32::UI::TextServices::GUID_COMPARTMENT_KEYBOARD_OPENCLOSE;
use windows::Win32::UI::WindowsAndMessaging::DestroyWindow;

use khiin_protos::command::Command;
use khiin_protos::config::AppConfig;
use khiin_protos::config::BoolValue;

use crate::dll::DllModule;
use crate::engine::EngineCoordinator;
use crate::engine::MessageHandler;
use crate::fail;
use crate::locales::set_locale;
use crate::reg::guids::GUID_CONFIG_CHANGED_COMPARTMENT;
use crate::reg::guids::GUID_DISPLAY_ATTRIBUTE_CONVERTED;
use crate::reg::guids::GUID_DISPLAY_ATTRIBUTE_FOCUSED;
use crate::reg::guids::GUID_DISPLAY_ATTRIBUTE_INPUT;
use crate::reg::guids::GUID_RESET_USERDATA_COMPARTMENT;
use crate::tip::composition_utils::text_position;
use crate::tip::open_edit_session;
use crate::tip::CandidateListUI;
use crate::tip::Compartment;
use crate::tip::CompositionMgr;
use crate::tip::DisplayAttributes;
use crate::tip::KeyEventSink;
use crate::tip::LangBarIndicator;
use crate::tip::PreservedKeyMgr;
use crate::tip::SinkMgr;
use crate::tip::TfEditCookie;
use crate::tip::ThreadMgrEventSink;
use crate::ui::candidates::CandidateWindow;
use crate::ui::systray::SystrayMenu;
use crate::ui::wndproc::Wndproc;
use crate::ui::RenderFactory;
use crate::utils::co_create_inproc;
use crate::utils::ArcLock;
use crate::utils::GetPath;

pub const TF_CLIENTID_NULL: u32 = 0;
pub const TF_INVALID_GUIDATOM: u32 = 0;

#[implement(
    ITfTextInputProcessorEx,
    ITfTextInputProcessor,
    ITfCompartmentEventSink,
    ITfCompositionSink,
    ITfDisplayAttributeProvider
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

    // Config
    on_off_state_locked: ArcLock<bool>,
    config: Arc<RwLock<AppConfig>>,

    // Key handling
    key_event_sink: RefCell<Option<ITfKeyEventSink>>,

    // Thread mgr
    threadmgr_event_sink: RefCell<Option<ITfThreadMgrEventSink>>,
    threadmgr_event_sink_sinkmgr: RefCell<SinkMgr<ITfThreadMgrEventSink>>,

    // Compartments
    open_close_compartment: Arc<RwLock<Compartment>>,
    open_close_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    config_compartment: Arc<RwLock<Compartment>>,
    config_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    userdata_compartment: Arc<RwLock<Compartment>>,
    userdata_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    kbd_disabled_compartment: Arc<RwLock<Compartment>>,
    kbd_disabled_sinkmgr: RefCell<SinkMgr<ITfCompartmentEventSink>>,

    // UI elements
    enum_disp_attr_info: IEnumTfDisplayAttributeInfo,
    disp_attr_guidatoms: RefCell<HashMap<SegmentStatus, u32>>,
    lang_bar_indicator: RefCell<Option<ITfLangBarItemButton>>,
    preserved_key_mgr: RefCell<Option<PreservedKeyMgr>>,
    candidate_list_ui: RefCell<Option<ITfUIElement>>,
    composition_mgr: Arc<RwLock<CompositionMgr>>,
    pub render_factory: Arc<RenderFactory>,

    // Data
    engine_coordinator: RefCell<Option<EngineCoordinator>>,
    message_handler: RefCell<Option<HWND>>,
    context_cache: Rc<RefCell<HashMap<u32, ITfContext>>>,
    current_command: RefCell<Option<Arc<Command>>>,

    // State
    is_editing_state: RefCell<bool>,
}

// Public portion
impl TextService {
    pub const IID: GUID =
        GUID::from_u128(0x829893f6_728d_11ec_8c6e_e0d46491b35a);

    pub fn new() -> Result<Self> {
        Ok(Self {
            this: RefCell::new(None),
            threadmgr: RefCell::new(None),
            clientid: ArcLock::new(TF_CLIENTID_NULL),
            dwflags: ArcLock::new(0),

            on_off_state_locked: ArcLock::new(false),
            config: Arc::new(RwLock::new(AppConfig::new())),

            key_event_sink: RefCell::new(None),

            threadmgr_event_sink: RefCell::new(None),
            threadmgr_event_sink_sinkmgr: RefCell::new(SinkMgr::<
                ITfThreadMgrEventSink,
            >::new()),

            open_close_compartment: Arc::new(RwLock::new(Compartment::new())),
            open_close_sinkmgr: RefCell::new(
                SinkMgr::<ITfCompartmentEventSink>::new(),
            ),

            config_compartment: Arc::new(RwLock::new(Compartment::new())),
            config_sinkmgr: RefCell::new(
                SinkMgr::<ITfCompartmentEventSink>::new(),
            ),

            userdata_compartment: Arc::new(RwLock::new(Compartment::new())),
            userdata_sinkmgr: RefCell::new(
                SinkMgr::<ITfCompartmentEventSink>::new(),
            ),

            kbd_disabled_compartment: Arc::new(RwLock::new(Compartment::new())),
            kbd_disabled_sinkmgr: RefCell::new(SinkMgr::<
                ITfCompartmentEventSink,
            >::new()),

            preserved_key_mgr: RefCell::new(None),
            enum_disp_attr_info: DisplayAttributes::new().into(),
            disp_attr_guidatoms: RefCell::new(HashMap::new()),
            lang_bar_indicator: RefCell::new(None),
            candidate_list_ui: RefCell::new(None),
            composition_mgr: Arc::new(RwLock::new(CompositionMgr::new()?)),
            render_factory: Arc::new(RenderFactory::new()?),
            message_handler: RefCell::new(None),
            engine_coordinator: RefCell::new(None),
            context_cache: Rc::new(RefCell::new(HashMap::new())),
            current_command: RefCell::new(None),
            is_editing_state: RefCell::new(false),
        })
    }

    pub fn clientid(&self) -> Result<u32> {
        self.clientid.get()
    }

    pub fn enabled(&self) -> Result<bool> {
        if let Ok(config) = self.config.read() {
            Ok(config.ime_enabled.value)
        } else {
            Ok(false)
        }
    }

    pub fn set_enabled(&self, on_off: bool) -> Result<()> {
        if self.on_off_state_locked.get()? {
            return Ok(());
        }

        if let Ok(mut config) = self.config.write() {
            if config.ime_enabled.value != on_off {
                let mut enabled = BoolValue::new();
                enabled.value = on_off;
                config.ime_enabled = MessageField::some(enabled);
            }
        }

        if !on_off {
            // TODO: commit outstanding buffer
        }

        Ok(())
    }

    pub fn toggle_enabled(&self) -> Result<()> {
        Ok(())
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

    pub fn reset(&self) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn composing(&self) -> bool {
        self.composition_mgr
            .try_read()
            .map_or(false, |c| c.composing())
    }

    pub fn handle_composition(
        &self,
        ec: TfEditCookie,
        context: ITfContext,
        command: Arc<Command>,
    ) -> Result<()> {
        let mut comp_mgr = self.composition_mgr.write().map_err(|_| fail!())?;

        let sink: ITfCompositionSink = self.this().cast()?;
        let attr_atoms = &*self.disp_attr_guidatoms.borrow();
        comp_mgr.notify_command(ec, context, sink, command, attr_atoms)
    }

    pub fn commit_all(&self, context: ITfContext) -> Result<()> {
        let preedit = self
            .current_command
            .borrow()
            .clone()
            .ok_or(fail!())?
            .response
            .preedit
            .clone();
        self.current_command.replace(None);
        let composing = self.composing();
        open_edit_session(self.clientid.get()?, context.clone(), |ec| {
            let mut comp_mgr = self.composition_mgr.write().map_err(|_| fail!())?;
            comp_mgr.commit_all(ec, context.clone(), &preedit)?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn commit_composition(&self) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn handle_candidates(
        &self,
        ec: TfEditCookie,
        context: ITfContext,
        command: Arc<Command>,
    ) -> Result<()> {
        let focused_caret = command.response.preedit.focused_caret;
        let rect = text_position(ec, context.clone(), focused_caret)?;

        log::debug!("Text position: {:?}", rect);

        let cand_ui = self
            .candidate_list_ui
            .try_borrow()
            .map_err(|_| fail!())?
            .clone()
            .ok_or(fail!())?;

        let cand_ui = unsafe { cand_ui.as_impl() };
        cand_ui.notify_command(context, command, rect)
    }

    pub fn on_candidate_selected(&self, id: u32) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn send_command(
        &self,
        context: ITfContext,
        command: Command,
    ) -> Result<()> {
        let id = command.request.id;
        self.context_cache.borrow_mut().insert(id, context.clone());

        if let Some(x) = self.engine_coordinator.borrow().as_ref() {
            x.send_command(command).map_err(|_| fail!())
            // let tx = x.sender();
            // tx.send(command).map_err(|_| fail!())
        } else {
            Ok(())
        }
    }

    pub fn handle_command(&self, command: Arc<Command>) -> Result<()> {
        let context = self
            .context_cache
            .borrow_mut()
            .remove(&command.request.id)
            .ok_or(fail!())?
            .clone();

        self.current_command.replace(Some(command.clone()));
        open_edit_session(self.clientid.get()?, context.clone(), |ec| {
            self.handle_composition(ec, context.clone(), command.clone())
        })?;

        let editing = command.response.edit_state.enum_value_or_default()
            != EditState::ES_EMPTY;
        self.is_editing_state.replace(editing);

        if command.response.edit_state.enum_value_or_default()
            == EditState::ES_EMPTY
        {
            let cand_ui = self
                .candidate_list_ui
                .try_borrow()
                .map_err(|_| fail!())?
                .clone()
                .ok_or(fail!())?;

            let cand_ui = unsafe { cand_ui.as_impl() };
            cand_ui.notify_command(context, command, Default::default());
        } else if self.is_classic_mode() {
            open_edit_session(self.clientid.get()?, context.clone(), |ec| {
                self.handle_candidates(ec, context.clone(), command.clone())
            })?;
        } else {
            log::debug!("Not in classic mode, ignoring handle candidate");
        }

        Ok(())
    }

    pub fn is_classic_mode(&self) -> bool {
        if let Ok(config) = self.config.read() {
            return config.input_mode.enum_value_or_default()
                == khiin_protos::config::AppInputMode::CLASSIC;
        } else {
            return false;
        }
    }

    pub fn is_manual_mode(&self) -> bool {
        if let Ok(config) = self.config.read() {
            return config.input_mode.enum_value_or_default()
                == khiin_protos::config::AppInputMode::MANUAL;
        } else {
            return false;
        }
    }

    pub fn is_editing(&self) -> bool {
        *self.is_editing_state.borrow()
    }
}

// Private portion
impl TextService {
    fn activate(&self) -> Result<()> {
        DllModule::global().add_ref();
        set_locale("en");
        self.init_engine()?;
        CandidateWindow::register_class(DllModule::global().module);
        SystrayMenu::register_class(DllModule::global().module);
        self.init_lang_bar_indicator()?;
        self.init_threadmgr_event_sink()?;
        self.init_candidate_ui()?;
        self.init_open_close_compartment()?;
        self.init_config_compartment()?;
        self.init_userdata_compartment()?;
        self.init_kbd_disabled_compartment()?;
        self.init_preserved_key_mgr()?;
        self.init_key_event_sink()?;
        self.init_display_attributes()?;
        self.set_enabled(true)?;
        log::debug!("TextService fully activated");
        Ok(())
    }

    fn deactivate(&self) -> Result<()> {
        log::debug!("TextService begin deactivating");
        self.set_enabled(false).ok();
        self.deinit_composition_mgr().ok();
        self.deinit_display_attributes().ok();
        self.deinit_key_event_sink().ok();
        self.deinit_preserved_key_mgr().ok();
        self.deinit_kbd_disabled_compartment().ok();
        self.deinit_userdata_compartment().ok();
        self.deinit_config_compartment().ok();
        self.deinit_open_close_compartment().ok();
        self.deinit_candidate_ui().ok();
        self.deinit_threadmgr_event_sink().ok();
        self.deinit_lang_bar_indicator().ok();
        self.deinit_engine().ok();
        SystrayMenu::unregister_class(DllModule::global().module);
        CandidateWindow::unregister_class(DllModule::global().module);
        DllModule::global().release();
        Ok(())
    }

    fn init_engine(&self) -> Result<()> {
        let filename = PathBuf::from(DllModule::global().module.get_path()?);
        let mut dbfile = filename.parent().ok_or(fail!())?.to_path_buf();
        dbfile.push("khiin.db");
        let dbfile = dbfile.to_string_lossy().to_string();

        let handler = Arc::new(MessageHandler::new(self.this()));
        let handle =
            MessageHandler::create(handler, DllModule::global().module)?;
        let engine = EngineCoordinator::new(handle)?;
        self.message_handler.replace(Some(handle));
        self.engine_coordinator.replace(Some(engine));
        Ok(())
    }

    fn deinit_engine(&self) -> Result<()> {
        let handle = self.message_handler.borrow().clone().unwrap();
        unsafe {
            DestroyWindow(handle);
            MessageHandler::unregister_class(DllModule::global().module);
        }
        let engine = self.engine_coordinator.replace(None);
        engine.unwrap().shutdown()?;
        Ok(())
    }

    // compartments & sinkmgrs
    fn init_compartment(
        &self,
        guid: GUID,
        compartment: &Arc<RwLock<Compartment>>,
        sinkmgr: &RefCell<SinkMgr<ITfCompartmentEventSink>>,
    ) -> Result<()> {
        if let Ok(mut comp) = compartment.write() {
            comp.init_thread(self.threadmgr(), self.clientid()?, guid)?;
            let mut sinkmgr = sinkmgr.borrow_mut();
            let punk: IUnknown = comp.compartment()?.cast()?;
            let this: ITfCompartmentEventSink = self.this().cast()?;
            sinkmgr.advise(punk, this)
        } else {
            Err(fail!())
        }
    }

    fn deinit_compartment(
        &self,
        compartment: &Arc<RwLock<Compartment>>,
        sinkmgr: &RefCell<SinkMgr<ITfCompartmentEventSink>>,
    ) -> Result<()> {
        sinkmgr.borrow_mut().unadvise()?;
        match compartment.write() {
            Ok(mut comp) => comp.deinit(),
            Err(_) => Err(fail!()),
        }
    }

    fn get_compartment_bool(
        &self,
        compartment: &Arc<RwLock<Compartment>>,
    ) -> Result<bool> {
        match compartment.read() {
            Ok(comp) => comp.get_bool(),
            Err(_) => Err(fail!()),
        }
    }

    fn get_compartment_u32(
        &self,
        compartment: &Arc<RwLock<Compartment>>,
    ) -> Result<u32> {
        match compartment.read() {
            Ok(comp) => comp.get_value(),
            Err(_) => Err(fail!()),
        }
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
        match self.open_close_compartment.read() {
            Ok(comp) => comp.set_bool(value),
            Err(_) => Err(fail!()),
        }
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
        unsafe { self.key_event_sink().as_impl().advise() }
    }

    fn deinit_key_event_sink(&self) -> Result<()> {
        let _ = unsafe { self.key_event_sink().as_impl() }.unadvise();
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
        let indicator = unsafe { indicator.as_impl() };
        let _ = indicator.shutdown(button);
        // logging?
        self.lang_bar_indicator.replace(None);
        Ok(())
    }

    fn lang_bar_indicator(&self) -> ITfLangBarItemButton {
        self.lang_bar_indicator.borrow().clone().unwrap()
    }

    // candidate ui
    fn init_candidate_ui(&self) -> Result<()> {
        self.candidate_list_ui
            .replace(Some(CandidateListUI::new(self.this())?));
        Ok(())
    }

    fn deinit_candidate_ui(&self) -> Result<()> {
        {
            let cand_ui = (&*self.candidate_list_ui.borrow()).clone().unwrap();
            let cand_ui = unsafe { cand_ui.as_impl() };
            cand_ui.shutdown()?;
        }
        self.candidate_list_ui.replace(None);
        Ok(())
    }

    // display attributes (underlines)
    fn init_display_attributes(&self) -> Result<()> {
        unsafe {
            let categorymgr = self.categorymgr()?;

            let input_attr =
                categorymgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_INPUT)?;
            let converted_attr =
                categorymgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_CONVERTED)?;
            let focused_attr =
                categorymgr.RegisterGUID(&GUID_DISPLAY_ATTRIBUTE_FOCUSED)?;

            let mut map = HashMap::new();
            map.insert(SegmentStatus::SS_COMPOSING, input_attr);
            map.insert(SegmentStatus::SS_FOCUSED, focused_attr);
            map.insert(SegmentStatus::SS_CONVERTED, converted_attr);
            map.insert(SegmentStatus::SS_UNMARKED, TF_INVALID_GUIDATOM);
            self.disp_attr_guidatoms.replace(map);
        }

        Ok(())
    }

    fn deinit_display_attributes(&self) -> Result<()> {
        self.disp_attr_guidatoms.replace(HashMap::new());
        Ok(())
    }

    fn deinit_composition_mgr(&self) -> Result<()> {
        self.composition_mgr
            .try_read()
            .map_err(|_| fail!())?
            .reset()
    }
}

//+---------------------------------------------------------------------------
//
// ITfDisplayAttributeProvider
//
//----------------------------------------------------------------------------

impl ITfDisplayAttributeProvider_Impl for TextService {
    fn EnumDisplayAttributeInfo(&self) -> Result<IEnumTfDisplayAttributeInfo> {
        Ok(self.enum_disp_attr_info.clone())
    }

    fn GetDisplayAttributeInfo(
        &self,
        guid: *const GUID,
    ) -> Result<ITfDisplayAttributeInfo> {
        let guid = unsafe { *guid };
        let disp_attrs = unsafe { self.enum_disp_attr_info.as_impl() };

        match disp_attrs.by_guid(guid) {
            Some(attr) => Ok(attr.into()),
            None => Err(fail!()),
        }
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
        composition: Option<&ITfComposition>,
    ) -> Result<()> {
        // TODO: send command
        // self.engine()
        //     .try_read()
        //     .map_err(|_| Error::from(E_FAIL))?
        //     .reset()?;
        if let Some(comp) = composition {
            unsafe {
                comp.EndComposition(ecwrite)?;
            }
            self.composition_mgr
                .try_read()
                .map_err(|_| fail!())?
                .cancel_composition(ecwrite)?;
        }
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
        self.ActivateEx(ptim, tid, 0)
    }

    fn Deactivate(&self) -> Result<()> {
        self.deactivate().ok();
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
        crate::trace!();

        self.clientid.set(tid)?;
        self.dwflags.set(dwflags)?;

        match ptim {
            Some(threadmgr) => {
                self.threadmgr.replace(Some(threadmgr.clone()));
                if self.activate().is_err() {
                    d!("TextService activation failed, deactivating...");
                    self.deactivate()?;
                }
                Ok(())
            },
            None => Ok(()),
        }
    }
}
