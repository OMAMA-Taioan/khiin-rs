use anyhow::anyhow;
use anyhow::Result;
use khiin_protos::command::EditState;
use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;
use khiin_protos::command::preedit::Segment;

use crate::config::Config;
use crate::config::InputMode;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::converter::convert_all;
use crate::input::parse_input;
use crate::Engine;

use super::Buffer;
use super::BufferElement;

pub struct BufferMgr {
    composition: Buffer,
    candidates: Vec<Buffer>,
    edit_state: EditState,
    char_caret: usize,
}

impl BufferMgr {
    pub fn new() -> Self {
        Self {
            composition: Buffer::default(),
            candidates: Vec::new(),
            edit_state: EditState::ES_EMPTY,
            char_caret: 0,
        }
    }

    pub fn build_preedit(&self) -> Preedit {
        let mut preedit = Preedit::default();

        for elem in self.composition.iter() {
            let mut segment = Segment::default();
            segment.value = elem.composed_text().into();
            segment.status = SegmentStatus::SS_COMPOSING.into();
            preedit.segments.push(segment);
        }

        preedit.caret = self.char_caret as i32;
        preedit
    }

    pub fn edit_state(&self) -> EditState {
        self.edit_state
    }

    pub fn insert(
        &mut self,
        db: &Database,
        dict: &Dictionary,
        conf: &Config,
        ch: char,
    ) -> Result<()> {
        self.edit_state = EditState::ES_COMPOSING;

        match conf.input_mode() {
            InputMode::Continuous => self.insert_continuous(db, dict, conf, ch),
            InputMode::SingleWord => self.insert_single_word(ch),
            InputMode::Manual => self.insert_manual(ch),
        }
    }

    fn insert_continuous(
        &mut self,
        db: &Database,
        dict: &Dictionary,
        conf: &Config,
        ch: char,
    ) -> Result<()> {
        let mut composition = self.composition.raw_text();
        composition.push(ch);
        self.char_caret += 1;
        assert!(composition.is_ascii());
        let (comp, cand) = convert_all(db, dict, conf, &composition)?;
        self.composition = comp;
        self.candidates = cand;
        Ok(())
    }

    fn insert_single_word(&mut self, ch: char) -> Result<()> {
        Err(anyhow!("Not implemented"))
    }

    fn insert_manual(&mut self, ch: char) -> Result<()> {
        Err(anyhow!("Not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use super::*;

    fn setup() -> (Database, Dictionary, Config, BufferMgr) {
        (get_db(), get_dict(), get_conf(), BufferMgr::new())
    }

    #[test]
    fn it_works() {
        let buf = BufferMgr::new();
        assert_eq!(buf.char_caret, 0);
    }

    #[test]
    fn it_inserts_chars_continuous_mode() -> Result<()> {
        let (db, dict, conf, mut buf) = setup();
        buf.insert_continuous(&db, &dict, &conf, 'a')?;
        assert_eq!(buf.composition.raw_text().as_str(), "a");
        assert_eq!(buf.composition.composed_text().as_str(), "a");
        Ok(())
    }
}
