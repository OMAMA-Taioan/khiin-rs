use anyhow::{Result, Error};
use khiin_protos::command::*;
use khiin_protos::command::preedit::*;
use protobuf::Message;

pub struct Engine;

fn get_mock_command(cmd: &mut Command) {
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

impl Engine {
    pub fn new() -> Option<Engine> {
        Some(Engine)
    }

    pub fn send_command_bytes(&self, bytes: &[u8]) -> Result<Vec<u8>> {
        let mut cmd = Command::parse_from_bytes(bytes)?;
        get_mock_command(&mut cmd);
        cmd.write_to_bytes().map_err(|_| Error::msg("Failed to write protobuf bytes"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let engine = Engine::new();
        assert!(engine.is_some());
    }
}
