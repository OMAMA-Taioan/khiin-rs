use std::fmt::format;
use std::fmt::Debug;

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
use crate::utils::CharSubstr;

use super::StringElem;

pub(crate) struct BufferMgr {
    composition: Buffer,
    candidates: Vec<Buffer>,
    edit_state: EditState,

    /// Position of the text input caret, in chars of the preedit displayed text
    char_caret: usize,

    /// Focused preedit buffer segment (thick underline)
    focused_elem_idx: usize,

    /// Focused candidate in pager, also shows in preedit
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
                    composing_segment.clear();
                }

                let mut segment = Segment::default();
                segment.value = elem.display_text();
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
            cand.id = i as i32;
            list.candidates.push(cand);
        }

        list.focused = if self.focused_cand_idx.is_none() {
            -1
        } else {
            self.focused_cand_idx.unwrap() as i32
        };

        list
    }

    pub fn edit_state(&self) -> EditState {
        self.edit_state
    }

    pub fn reset(&mut self) -> Result<()> {
        self.composition.clear();
        self.candidates.clear();
        self.focused_cand_idx = None;
        self.focused_elem_idx = 0;
        Ok(())
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

        assert!(composition.is_ascii());

        self.composition = convert_all(db, dict, conf, &composition)?;
        self.candidates = get_candidates(db, dict, conf, &composition)?;

        let mut first = self.composition.clone();
        first.set_converted(true);

        if !self
            .candidates
            .iter()
            .any(move |cand| first.eq_display(cand))
        {
            let mut first = self.composition.clone();
            first.set_converted(true);
            self.candidates.insert(0, first);
        }
        self.char_caret = self.composition.display_char_count();

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

        let mut to_focus = match self.focused_cand_idx {
            Some(i) if i >= self.candidates.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        };

        self.focus_candidate(to_focus);

        Ok(())
    }

    pub fn focus_prev_candidate(&mut self) -> Result<()> {
        let mut to_focus = match self.focused_cand_idx {
            Some(i) if i == 0 => self.candidates.len() - 1,
            Some(i) => i - 1,
            None => self.candidates.len() - 1,
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
        let lhs_remainder =
            lhs_raw.char_substr(cand_raw_count, lhs_raw.chars().count());

        let mut new_comp = candidate;

        if !lhs_remainder.is_empty() {
            new_comp.push(StringElem::from(lhs_remainder).into());
        }

        if !comp_rhs.is_empty() {
            new_comp.extend(comp_rhs);
        }

        self.composition = new_comp;

        self.focused_cand_idx = Some(index);
        self.char_caret = self.composition.display_char_count();

        Ok(())
    }

    fn _fmt_preedit(&self, sep: char) -> String {
        let preedit = self.build_preedit();
        let mut display_text = String::new();
        let mut char_count = 0;
        display_text.push(sep);
        display_text.push_str(&format!("Raw: {}", self.composition.raw_text()));

        let state = match self.edit_state {
            EditState::ES_EMPTY => "Empty",
            EditState::ES_COMPOSING => "Composing",
            EditState::ES_CONVERTED => "Converted",
            EditState::ES_SELECTING => "Selecting",
        };

        display_text.push(sep);
        display_text.push_str(state);
        display_text.push_str(": ");

        for (i, elem) in preedit.segments.iter().enumerate() {
            if preedit.focused_caret == i as i32 {
                display_text.push('>');
            }

            for ch in elem.value.chars() {
                if char_count as i32 == preedit.caret {
                    display_text.push('|');
                }
                display_text.push(ch);
                char_count += 1;
            }

            if char_count as i32 == preedit.caret {
                display_text.push('|');
            }

            if elem.status.unwrap() == SegmentStatus::SS_CONVERTED {
                display_text.push('^');
            }

            if elem.status.unwrap() == SegmentStatus::SS_FOCUSED {
                display_text.push('*');
            }
        }

        display_text
    }
}

impl Debug for BufferMgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sep = if f.alternate() { '\n' } else { ' ' };

        let mut display_text = self._fmt_preedit(sep);

        let cands = self.get_candidates();

        if cands.candidates.is_empty() {
            return write!(f, "{}", display_text);
        }

        display_text.push(sep);
        display_text.push_str("Candidates:");
        display_text.push(sep);

        for (i, cand) in cands.candidates.iter().enumerate() {
            display_text.push_str(&format!("{}. {}", i + 1, cand.value));
            display_text.push(sep);
        }

        write!(f, "{}", display_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::unicode::*;
    use crate::tests::*;
    use crate::utils::Unique;

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
        assert_eq!(buf.focused_cand_idx, Some(0));
        assert_eq!(buf.get_candidates().focused, 0);
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

    #[test_log::test]
    fn it_excludes_duplicates() -> Result<()> {
        let (db, dict, conf, mut buf) = setup();
        for ch in "pengan".chars().collect::<Vec<char>>() {
            buf.insert(&db, &dict, &conf, ch)?;
        }

        assert!(buf.candidates.len() > 1);

        let display_texts: Vec<String> =
            buf.candidates.iter().map(|ea| ea.display_text()).collect();

        assert!(display_texts.all_unique());

        Ok(())
    }

    #[test_log::test]
    fn it_positions_caret_at_end_during_composition() -> Result<()> {
        let (db, dict, conf, mut buf) = setup();
        for ch in "pengan".chars().collect::<Vec<char>>() {
            buf.insert(&db, &dict, &conf, ch)?;
        }
        log::debug!("{:?}", buf);
        Ok(())
    }
}
