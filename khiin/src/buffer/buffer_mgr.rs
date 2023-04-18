use anyhow::anyhow;
use anyhow::Result;

use khiin_protos::command::preedit::Segment;
use khiin_protos::command::Candidate;
use khiin_protos::command::CandidateList;
use khiin_protos::command::EditState;
use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;
use log::trace;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
use crate::config::Config;
use crate::config::InputMode;
use crate::data::Database;
use crate::data::Dictionary;
use crate::input::converter::convert_all;
use crate::input::converter::get_candidates;

use super::StringElem;

pub(crate) struct BufferMgr {
    composition: Buffer,
    candidates: Vec<Buffer>,
    edit_state: EditState,
    char_caret: usize,
    focused_elem_idx: usize,
    focused_cand_idx: Option<usize>,
}

impl BufferMgr {
    pub fn new() -> Self {
        Self {
            composition: Buffer::default(),
            candidates: Vec::new(),
            edit_state: EditState::ES_EMPTY,
            char_caret: 0,
            focused_elem_idx: 0,
            focused_cand_idx: None,
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
                segment.value = elem.converted_text();
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

        if !self
            .candidates
            .iter()
            .any(|cand| *&self.composition.eq_display(cand))
        {
            let mut first = self.composition.clone();
            first.set_converted(true);
            self.candidates.insert(0, first);
        }

        Ok(())
    }

    fn insert_single_word(&mut self, ch: char) -> Result<()> {
        Err(anyhow!("Not implemented"))
    }

    fn insert_manual(&mut self, ch: char) -> Result<()> {
        Err(anyhow!("Not implemented"))
    }

    pub fn focus_next_candidate(&mut self) -> Result<()> {
        if self.edit_state == EditState::ES_COMPOSING {
            self.edit_state = EditState::ES_SELECTING;
        }

        let to_focus = match self.focused_cand_idx {
            Some(i) => i + 1,
            None => 0,
        };
        self.focus_candidate(to_focus);

        Ok(())
    }

    // When focusing a candidate, we must construct the new composition to be
    // displayed in the preedit.
    //
    // Steps:
    // 1. Get the raw text from the candidate
    // 2. Split the current composition into [ lhs, rhs ]:
    //    [ elements up to this raw text, elements after this raw text]
    // 3. Get the remaining text after removing the prefix (1) from the raw text
    //    buffer (2-LHS)
    // 4. Make a new composition
    // 5. Add the candidate into the composition
    // 6. Add the remaining raw text (3) into the composition
    // 7. Add back the elements from 2-RHS
    fn focus_candidate(&mut self, index: usize) -> Result<()> {
        let candidate = self
            .candidates
            .get(index)
            .ok_or(anyhow!("Candidate index out of bounds"))?
            .clone();

        let cand_raw_count = candidate.raw_char_count();

        let comp_split_element_index = self
            .composition
            .elem_index_at_raw_char_count(cand_raw_count);

        let mut comp_lhs = self.composition.clone();
        let comp_rhs = comp_lhs.split_off(comp_split_element_index + 1);

        let lhs_raw = comp_lhs.raw_text();
        let lhs_remainder = substring(&lhs_raw, 0, cand_raw_count);

        let mut new_comp = candidate;

        if !lhs_remainder.is_empty() {
            new_comp.push(StringElem::from(lhs_remainder).into());
        }

        if !comp_rhs.is_empty() {
            new_comp.extend(comp_rhs);
        }

        self.composition = new_comp;

        self.focused_cand_idx = Some(index);

        Ok(())
    }
}

fn substring(
    s: &str,
    start_char_index: usize,
    end_char_index: usize,
) -> String {
    let mut char_count = 0;
    let mut start_byte_index = None;
    let mut end_byte_index = None;

    for (i, _) in s.char_indices() {
        if char_count == start_char_index {
            start_byte_index = Some(i);
        }
        if char_count == end_char_index {
            end_byte_index = Some(i);
            break;
        }
        char_count += 1;
    }

    if let Some(start) = start_byte_index {
        if let Some(end) = end_byte_index {
            return String::from(&s[start..end]);
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::unicode::*;
    use crate::tests::*;

    fn setup() -> (Database, Dictionary, Config, BufferMgr) {
        (get_db(), get_dict(), get_conf(), BufferMgr::new())
    }

    fn preedit_text(buf: &BufferMgr) -> String {
        let pe = buf.build_preedit();
        pe.segments.into_iter().map(|s| s.value).collect()
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

    #[test]
    fn it_focuses_the_first_candidate() -> Result<()> {
        let (db, dict, conf, mut buf) = setup();
        buf.insert(&db, &dict, &conf, 'a')?;
        buf.focus_next_candidate()?;
        let text = preedit_text(&buf);
        assert_eq!(text.as_str(), "亞");
        Ok(())
    }

    #[test_log::test]
    fn it_focuses_the_second_candidate() -> Result<()> {
        let (db, dict, conf, mut buf) = setup();
        buf.insert(&db, &dict, &conf, 'a')?;
        buf.focus_next_candidate()?;
        buf.focus_next_candidate()?;
        let text = preedit_text(&buf);
        assert_eq!(text.as_str(), "亜");
        Ok(())
    }
}
