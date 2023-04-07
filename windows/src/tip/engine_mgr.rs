use std::cell::RefCell;
use std::sync::Arc;

use khiin_protos::command::EditState;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;

use khiin_protos::command::Candidate;
use khiin_protos::command::CandidateList;
use khiin_protos::command::Preedit;
use khiin_protos::command::Response;
use khiin_protos::command::SegmentStatus;
use khiin_protos::command::preedit::Segment;
use khiin_protos::command::Command;
use khiin_protos::command::KeyEvent;
use khiin_protos::command::Request;

use crate::tip::key_event::KeyEvent as WinKeyEvent;
use crate::winerr;

pub fn get_mock_command(key_event: WinKeyEvent) -> Command {
    let mut key_event = Some(translate_key_event(key_event)).into();
    let mut req = Request::new();
    req.key_event = key_event;

    let mut cand = Candidate::new();
    cand.id = 1;
    cand.value = "起引".to_owned();
    cand.key = "khiin".to_owned();

    let mut cl = CandidateList::new();
    cl.candidates = Vec::new();
    cl.candidates.push(cand);

    let mut seg = Segment::new();
    seg.value = "khiin".to_owned();
    seg.status = SegmentStatus::SS_COMPOSING.into();

    let mut pe = Preedit::new();
    pe.caret = 0;
    pe.focused_caret = 0;
    pe.segments = Vec::new();
    pe.segments.push(seg);

    let mut res = Response::new();
    res.edit_state = EditState::ES_COMPOSING.into();
    res.preedit = Some(pe).into();
    res.candidate_list = Some(cl).into();

    let mut cmd = Command::new();
    cmd.request = Some(req).into();
    cmd.response = Some(res).into();

    cmd
}

pub fn translate_key_event(input: WinKeyEvent) -> KeyEvent {
    let mut proto = KeyEvent::new();
    proto.key_code = input.keycode as i32;
    proto
}

pub struct EngineMgr {
    tip: RefCell<Option<ITfTextInputProcessor>>,
    engine: khiin::Engine,
}

impl EngineMgr {
    pub fn new() -> Result<Self> {
        let engine = khiin::Engine::new();

        if engine.is_none() {
            return winerr!(E_FAIL);
        }

        Ok(EngineMgr {
            engine: engine.unwrap(),
            tip: RefCell::new(None),
        })
    }

    pub fn init(&self, tip: ITfTextInputProcessor) -> Result<()> {
        self.tip.replace(Some(tip));
        Ok(())
    }

    pub fn deinit(&self) {
        // TODO
    }

    pub fn reset(&self) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn on_test_key(&self, key_event: &WinKeyEvent) -> bool {
        key_event.ascii != 0
    }

    pub fn on_key(&self, key_event: WinKeyEvent) -> Result<Arc<Command>> {
        Ok(Arc::new(get_mock_command(key_event)))
    }
}
