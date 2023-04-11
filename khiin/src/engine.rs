use anyhow::Error;
use anyhow::Result;
use khiin_protos::command::preedit::*;
use khiin_protos::command::*;
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
        let req = cmd.request.clone().unwrap();

        let res = match req.type_.enum_value_or_default() {
            CommandType::CMD_UNSPECIFIED => todo!(),
            CommandType::CMD_SEND_KEY => self.on_send_key(req),
            CommandType::CMD_REVERT => self.on_revert(req),
            CommandType::CMD_RESET => self.on_reset(req),
            CommandType::CMD_COMMIT => self.on_commit(req),
            CommandType::CMD_SELECT_CANDIDATE => self.on_select_candidate(req),
            CommandType::CMD_FOCUS_CANDIDATE => self.on_focus_candidate(req),
            CommandType::CMD_SWITCH_INPUT_MODE => {
                self.on_switch_input_mode(req)
            }
            CommandType::CMD_PLACE_CURSOR => self.on_place_cursor(req),
            CommandType::CMD_DISABLE => self.on_disable(req),
            CommandType::CMD_ENABLE => self.on_enable(req),
            CommandType::CMD_SET_CONFIG => self.on_set_config(req),
            CommandType::CMD_TEST_SEND_KEY => self.on_test_send_key(req),
            CommandType::CMD_LIST_EMOJIS => self.on_list_emojis(req),
            CommandType::CMD_RESET_USER_DATA => self.on_reset_user_data(req),
            CommandType::CMD_SHUTDOWN => self.on_shutdown(req),
        };
        cmd.response = Some(res).into();
        cmd.write_to_bytes()
            .map_err(|_| Error::msg("Failed to write protobuf bytes"))
    }

    fn on_send_key(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_revert(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_reset(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_commit(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_select_candidate(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_focus_candidate(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_switch_input_mode(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_place_cursor(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_disable(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_enable(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_set_config(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_test_send_key(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_list_emojis(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_reset_user_data(&self, req: Request) -> Response {
        Response::default()
    }

    fn on_shutdown(&self, req: Request) -> Response {
        Response::default()
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
