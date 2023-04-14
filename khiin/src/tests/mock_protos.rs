use khiin_protos::command::{*, preedit::*};

pub fn mock_send_key_request(ch: char) -> Request {
    let mut req = Request::default();
    req.type_ = CommandType::CMD_SEND_KEY.into();

    let mut ke = KeyEvent::default();
    ke.key_code = ch as i32;
    req.key_event = Some(ke).into();
    req
}

fn mock_command(cmd: &mut Command) {
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

    cmd.response = Some(res).into();
}