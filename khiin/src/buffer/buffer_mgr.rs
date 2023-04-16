use anyhow::anyhow;
use anyhow::Result;

use khiin_protos::command::preedit::Segment;
use khiin_protos::command::Candidate;
use khiin_protos::command::CandidateList;
use khiin_protos::command::EditState;
use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
use crate::config::Config;
use crate::config::InputMode;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::converter::convert_all;
use crate::input::converter::get_candidates;

pub struct BufferMgr {
    composition: Buffer,
    candidates: Vec<Buffer>,
    edit_state: EditState,
    char_caret: usize,
    focused_elem_idx: usize,
}

impl BufferMgr {
    pub fn new() -> Self {
        Self {
            composition: Buffer::default(),
            candidates: Vec::new(),
            edit_state: EditState::ES_EMPTY,
            char_caret: 0,
            focused_elem_idx: 0,
        }
    }

    pub fn build_preedit(&self) -> Preedit {
        let mut preedit = Preedit::default();

        let mut composing_segment = String::new();

        for (i, elem) in self.composition.iter().enumerate() {
            if !elem.is_converted() {
                composing_segment.push_str(&elem.composed_text());
            } else {
                if !composing_segment.is_empty() {
                    let mut segment = Segment::default();
                    segment.value = composing_segment.clone();
                    segment.status = SegmentStatus::SS_COMPOSING.into();
                    preedit.segments.push(segment);
                }

                let mut segment = Segment::default();
                segment.value = elem.composed_text();
                segment.status = (if self.focused_elem_idx == i {
                    SegmentStatus::SS_FOCUSED
                } else {
                    SegmentStatus::SS_CONVERTED
                })
                .into();
                preedit.segments.push(segment);
            }
        }

        if !composing_segment.is_empty() {
            let mut segment = Segment::default();
            segment.value = composing_segment.clone();
            segment.status = SegmentStatus::SS_COMPOSING.into();
            preedit.segments.push(segment);
        }

        preedit.caret = self.char_caret as i32;
        preedit
    }

    pub fn get_candidates(&self) -> CandidateList {
        let mut list = CandidateList::default();

        if self.edit_state == EditState::ES_CONVERTED {
            return list;
        }

        for (i, c) in self.candidates.iter().enumerate() {
            let mut cand = Candidate::default();
            cand.value = c.display_text();
            cand.id = (i + 1) as i32;
            list.candidates.push(cand);
        }

        list
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
        self.composition = convert_all(db, dict, conf, &composition)?;
        self.candidates = get_candidates(db, dict, conf, &composition)?;
        let mut first = self.composition.clone();
        first.set_converted(true);
        self.candidates.insert(0, first);
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
    use super::*;
    use crate::input::unicode::*;
    use crate::tests::*;

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
        assert_eq!(buf.composition.display_text().as_str(), "a");
        assert!(buf.candidates.len() > 0);
        assert!(contains_hanji(&buf.candidates[0].display_text()));
        Ok(())
    }
}
