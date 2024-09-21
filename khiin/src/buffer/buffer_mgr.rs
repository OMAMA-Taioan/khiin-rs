use std::fmt::format;
use std::fmt::Debug;
use std::str::Chars;

use anyhow::anyhow;
use anyhow::Result;

use khiin_ji::lomaji::is_legal_lomaji;
use khiin_ji::IsHanji;
use khiin_protos::command::preedit::Segment;
use khiin_protos::command::Candidate;
use khiin_protos::command::CandidateList;
use khiin_protos::command::EditState;
use khiin_protos::command::Preedit;
use khiin_protos::command::SegmentStatus;
use log::debug;
use log::trace;
use protobuf::SpecialFields;
use regex::Regex;

use crate::buffer::Buffer;
use crate::buffer::BufferElement;
use crate::config::Config;
use crate::config::InputMode;
use crate::data::Dictionary;
use crate::db::Database;
use crate::engine::EngInner;
use crate::input::converter::convert_all;
use crate::input::converter::convert_to_telex;
use crate::input::converter::get_candidates;
use crate::input::converter::get_candidates_for_word;
use crate::input::converter::get_candidates_for_word_with_tone;
use crate::input::converter::get_numberic_tone_char;
use crate::utils::CharSubstr;

use super::Spacer;
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

    /// candidate page number
    cand_page: usize,

    /// Previous committed word, only for classic mode
    pre_committed: String,
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
            cand_page: 0,
            pre_committed: String::new(),
        }
    }

    pub fn build_preedit(&self) -> Preedit {
        let mut preedit = Preedit::default();

        let mut composing_segment = String::new();

        for (i, elem) in self.composition.iter().enumerate() {
            if let super::BufferElementEnum::Spacer(s) = elem {
                if s.deleted {
                    continue;
                }

                if !composing_segment.is_empty() {
                    composing_segment.push(' ');
                } else {
                    preedit.segments.push(Segment {
                        status: SegmentStatus::SS_UNMARKED.into(),
                        value: elem.display_text(),
                        special_fields: SpecialFields::default(),
                    });
                }

                continue;
            }

            if !elem.is_converted() {
                composing_segment.push_str(&elem.composed_text());
            } else {
                if !composing_segment.is_empty() {
                    preedit.segments.push(Segment {
                        status: SegmentStatus::SS_COMPOSING.into(),
                        value: composing_segment.clone(),
                        special_fields: SpecialFields::default(),
                    });
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
            preedit.segments.push(Segment {
                status: SegmentStatus::SS_COMPOSING.into(),
                value: composing_segment.clone(),
                special_fields: SpecialFields::default(),
            });
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
            cand.annotation = c.display_annotation();
            list.candidates.push(cand);
        }

        list.focused = if self.focused_cand_idx.is_none() {
            -1
        } else {
            self.focused_cand_idx.unwrap() as i32
        };

        list.page = self.cand_page as i32;

        list
    }

    pub fn edit_state(&self) -> EditState {
        self.edit_state
    }

    pub fn reset(&mut self) -> Result<()> {
        self.edit_state = EditState::ES_EMPTY;
        self.composition.clear();
        self.candidates.clear();
        self.focused_cand_idx = None;
        self.focused_elem_idx = 0;
        self.cand_page = 0;
        self.pre_committed.clear();
        Ok(())
    }

    pub fn insert(&mut self, engine: &EngInner, ch: char) -> Result<()> {
        match engine.conf.input_mode() {
            InputMode::Continuous => self.insert_continuous(engine, ch),
            InputMode::Classic => self.insert_classic(engine, ch),
            InputMode::Manual => self.insert_manual(engine, ch),
        }
    }

    pub fn pop(&mut self, engine: &EngInner) -> Result<()> {
        if self.edit_state == EditState::ES_EMPTY {
            return Ok(());
        }

        self.edit_state = EditState::ES_COMPOSING;

        match engine.conf.input_mode() {
            InputMode::Continuous => self.pop_continuous(engine),
            InputMode::Classic => self.pop_classic(engine),
            InputMode::Manual => self.pop_manual(engine),
        }
    }

    pub fn commit_candidate_and_comosite_remainder(
        &mut self,
        engine: &EngInner,
    ) -> Result<String> {
        if (engine.conf.input_mode() != InputMode::Classic) {
            // only classic mode need comosite remainder
            return Ok(String::new());
        }
        if self.candidates.is_empty() {
            let mut raw_input = self.composition.raw_text();
            let mut ret = String::new();
            if raw_input.starts_with("--") {
                raw_input.drain(0..2);
                ret.push_str("--");
            } else if raw_input.starts_with("-") {
                raw_input.drain(0..1);
                ret.push_str("-");
            } else {
                ret.push_str(&raw_input);
                raw_input.clear()
            }
            self.reset();
            if (!raw_input.is_empty()) {
                self.edit_state = EditState::ES_COMPOSING;
                let ch = raw_input.chars().last().unwrap();
                raw_input.pop();

                self.build_composition_classic(engine, raw_input, ch);
            }
            return Ok(ret);
        }
        let mut index = match self.focused_cand_idx {
            Some(i) if i >= self.candidates.len() => 0,
            Some(i) => i,
            None => self.cand_page * 9,
        };

        self.composition.clear_autospace();
        let candidate = self
            .candidates
            .get(index)
            .ok_or(anyhow!("Candidate index out of bounds"))?
            .clone();

        let mut cand_raw_count = candidate.raw_char_count();
        let mut candi_text = candidate.display_text();
        let comp_raw = self.composition.raw_text();
        let mut has_hyphen = false;
        if comp_raw.starts_with("--") {
            candi_text = format!("--{}", candi_text);
            cand_raw_count = cand_raw_count + 2;
            has_hyphen = true;
        } else if comp_raw.starts_with("-") {
            candi_text = format!("-{}", candi_text);
            cand_raw_count = cand_raw_count + 1;
            has_hyphen = true;
        } else if candi_text.starts_with("-") {
            has_hyphen = true;
        }
        let pre_committed = self.pre_committed.clone();
        let mut remainder =
            comp_raw.char_substr(cand_raw_count, comp_raw.chars().count());

        self.reset();
        if !remainder.is_empty() {
            self.edit_state = EditState::ES_COMPOSING;
            self.pre_committed.push_str(&candidate.display_text());

            let ch = remainder.chars().last().unwrap();
            remainder.pop();

            self.build_composition_classic(engine, remainder, ch);
        }
        if pre_committed.is_empty() || has_hyphen || pre_committed.ends_with("-") {
            return Ok(candi_text);
        }
        let is_hanji = pre_committed.chars().last().unwrap().is_hanji();
        if is_hanji && candi_text.chars().last().unwrap().is_hanji() {
            return Ok(candi_text);
        }
        let mut ret = String::from(' ');
        ret.push_str(&candi_text);
        return Ok(ret);
    }

    fn insert_continuous(&mut self, engine: &EngInner, ch: char) -> Result<()> {
        self.edit_state = EditState::ES_COMPOSING;
        debug!("BufferMgr::insert_continuous ({})", ch);
        let mut composition = self.composition.raw_text();
        composition.push(ch);

        self.build_composition_continuous(engine, composition)
    }

    fn pop_continuous(&mut self, engine: &EngInner) -> Result<()> {
        debug!("BufferMgr::pop_continuous ");
        let mut composition = self.composition.raw_text();
        composition.pop();

        if composition.is_empty() {
            return self.reset();
        }

        self.build_composition_continuous(engine, composition)
    }

    fn build_composition_continuous(
        &mut self,
        engine: &EngInner,
        composition: String,
    ) -> Result<()> {
        assert!(composition.is_ascii());

        self.composition = convert_all(engine, &composition)?;
        self.candidates = get_candidates(engine, &composition)?;

        debug!("Number of candidates found: {}", self.candidates.len());

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

        for c in self.candidates.iter_mut() {
            c.autospace();
        }

        self.composition.autospace();
        self.char_caret = self.composition.display_char_count();

        self.reset_focus();

        Ok(())
    }

    fn insert_classic(&mut self, engine: &EngInner, ch: char) -> Result<()> {
        debug!("BufferMgr::insert_classic ({})", ch);
        self.edit_state = EditState::ES_COMPOSING;

        let mut raw_input = self.composition.raw_text();
        self.build_composition_classic(engine, raw_input, ch)
    }

    fn pop_classic(&mut self, engine: &EngInner) -> Result<()> {
        debug!("BufferMgr::pop_classic ");
        let mut raw_input = self.composition.raw_text();
        raw_input.pop();

        if raw_input.is_empty() {
            return self.reset();
        }
        self.reset();
        self.edit_state = EditState::ES_COMPOSING;

        let ch = raw_input.chars().last().unwrap();
        raw_input.pop();

        self.build_composition_classic(engine, raw_input, ch)
    }

    fn attach_hypen_candicate(&mut self) {
        if self.candidates.is_empty() || self.pre_committed.is_empty() {
            return;
        }
        if self.pre_committed.ends_with("-") || self.composition.raw_text().starts_with("-") {
            return;
        }
        for ch in self.pre_committed.chars() {
            if ch.is_hanji() {
                return;
            }
        }
        let mut hypen_candi = Buffer::new();
        hypen_candi.push(StringElem::from_raw_input("".to_string(), "-".to_string()).into());
        if (self.candidates.len() >= 3) {
            self.candidates.insert(2, hypen_candi);
        } else {
            self.candidates.push(hypen_candi);
        }
    }

    fn build_composition_classic(
        &mut self,
        engine: &EngInner,
        mut raw_input: String,
        ch: char,
    ) -> Result<()> {
        let mut key = ch.to_ascii_lowercase();
        if (key == engine.conf.hyphon()) {
            if raw_input.ends_with("--") {
                let len = raw_input.len();
                raw_input.replace_range(len - 2..len, "");
                raw_input.push(ch);
                self.candidates.clear();
            } else {
                raw_input.push('-');
            }
            self.composition = Buffer::new();
            self.composition.push(StringElem::from(raw_input).into());
            self.char_caret = self.composition.display_char_count();
            return Ok(());
        } else if (key == engine.conf.khin()) {
            if raw_input.starts_with("--") {
                raw_input.drain(0..2);
                raw_input.push(key);
                self.candidates.clear();
            } else {
                raw_input.insert(0, '-');
                raw_input.insert(1, '-');
            }
            self.composition = Buffer::new();
            self.composition.push(StringElem::from(raw_input).into());
            self.char_caret = self.composition.display_char_count();
            return Ok(());
        }

        let mut word: String = raw_input.to_lowercase();
        let mut query = raw_input.clone();
        if query.starts_with("--") {
            query.drain(0..2);
        } else if query.starts_with("-") {
            query.drain(0..1);
        }

        let mut tone_char = key;
        if !engine.conf.is_tone_char(key) {
            word.push(key);
            query.push(ch);
            if "htpk".contains(key) {
                tone_char = '4';
            } else {
                tone_char = '1';
            }
        }
        word = word.replace("ou", "o͘");
        
        // to handle NASAL
        let re_single_nasal: Regex =
            Regex::new(r"(?i)[aeiouptkhmo͘]nn$").unwrap();
        if re_single_nasal.is_match(&word) {
            word = word.replace("nn", "ⁿ");
        }
        // one syllable
        if engine.dict.is_illegal_syllable(&word) {
            // convert to number tone
            if let Ok(candidates) =
                get_candidates_for_word_with_tone(engine, &query, tone_char)
            {
                if (!candidates.is_empty()) {
                    raw_input.push(ch);
                    self.candidates = candidates;
                    self.composition = Buffer::new();
                    self.composition.push(StringElem::from(raw_input).into());
                    self.char_caret = self.composition.display_char_count();
                    self.attach_hypen_candicate();
                    return Ok(());
                }
            }
            // get romaji
            let (ret_com, ret) = convert_to_telex(engine, &raw_input, key);
            if ret == true {
                raw_input.push(ch);
                let mut buf = Buffer::new();
                buf.push(
                    StringElem::from_raw_input(
                        raw_input.clone(),
                        ret_com?.display_text(),
                    )
                    .into(),
                );
                self.candidates = Vec::new();
                self.candidates.push(buf);
                self.composition = Buffer::new();
                self.composition.push(StringElem::from(raw_input).into());
                self.char_caret = self.composition.display_char_count();
                self.attach_hypen_candicate();
                return Ok(());
            }
        }

        raw_input.push(ch);
        query = raw_input.clone();
        if query.starts_with("--") {
            query.drain(0..2);
        } else if query.starts_with("-") {
            query.drain(0..1);
        }

        // add punctuation
        if query.starts_with("?") {
            self.candidates = Vec::new();
            let mut buf = Buffer::new();
            buf.push(StringElem::from("?").into());
            self.candidates.push(buf);

            buf = Buffer::new();
            buf.push(StringElem::from_raw_input("?".to_string(), "？".to_string()).into());
            self.candidates.push(buf);
        } else if query.starts_with(".") {
            self.candidates = Vec::new();
            let mut buf = Buffer::new();
            buf.push(StringElem::from(".").into());
            self.candidates.push(buf);

            buf = Buffer::new();
            buf.push(StringElem::from_raw_input(".".to_string(), "。".to_string()).into());
            self.candidates.push(buf);
        } else if query.starts_with(",") {
            self.candidates = Vec::new();
            let mut buf = Buffer::new();
            buf.push(StringElem::from(",").into());
            self.candidates.push(buf);

            buf = Buffer::new();
            buf.push(StringElem::from_raw_input(",".to_string(), "、".to_string()).into());
            self.candidates.push(buf);
        } else if query.starts_with("!") {
            self.candidates = Vec::new();
            let mut buf = Buffer::new();
            buf.push(StringElem::from("!").into());
            self.candidates.push(buf);

            buf = Buffer::new();
            buf.push(StringElem::from_raw_input("!".to_string(), "！".to_string()).into());
            self.candidates.push(buf);
        } else {
            let size = query.chars().count();
            for i in (0..size).rev() {
                let end = i + 1;
                let substr = &query[0..end];
                if let Ok(candidates) = get_candidates_for_word(engine, substr) {
                    if (!candidates.is_empty()) {
                        self.candidates = candidates;
                        break;
                    }
                };
            }
        }
        self.composition = Buffer::new();
        self.composition.push(StringElem::from(raw_input).into());
        self.char_caret = self.composition.display_char_count();
        self.attach_hypen_candicate();
        Ok(())
    }

    fn insert_manual(&mut self, engine: &EngInner, ch: char) -> Result<()> {
        debug!("BufferMgr::insert_manual ({})", ch);
        let mut raw_input = self.composition.raw_text();
        let mut key = ch;
        if self.edit_state == EditState::ES_ILLEGAL {
            raw_input.push(ch);
            self.composition = Buffer::new();
            self.composition.push(StringElem::from(raw_input).into());
            return Ok(());
        } else if ch.to_ascii_lowercase() == engine.conf.done()
            && self.edit_state == EditState::ES_COMPOSING
        {
            self.edit_state = EditState::ES_EMPTY;
            return Ok(());
        } else {
            self.edit_state = EditState::ES_COMPOSING;
        }

        if ch.to_ascii_lowercase() == engine.conf.hyphon()
            && self.edit_state == EditState::ES_COMPOSING
        {
            if raw_input.ends_with("--") {
                let len = raw_input.len();
                raw_input.replace_range(len - 2..len, "");
                raw_input.push(ch);
                self.edit_state = EditState::ES_ILLEGAL;
            } else {
                raw_input.push('-');
            }

            self.composition = Buffer::new();
            self.composition.push(StringElem::from(raw_input).into());
        } else {
            let (ret_com, ret) = convert_to_telex(engine, &raw_input, key);
            if ret == false {
                self.edit_state = EditState::ES_ILLEGAL
            }
            self.composition = ret_com?;
        }

        self.char_caret = self.composition.display_char_count();

        Ok(())
    }

    fn pop_manual(&mut self, engine: &EngInner) -> Result<()> {
        debug!("BufferMgr::pop_manual ");
        let mut raw_input = self.composition.raw_text();
        raw_input.pop();

        if raw_input.is_empty() {
            return self.reset();
        } else if self.edit_state == EditState::ES_ILLEGAL {
            self.composition = Buffer::new();
            self.composition.push(StringElem::from(raw_input).into());
        } else {
            let (ret_com, _) = convert_to_telex(engine, &raw_input, ' ');
            self.composition = ret_com?;
            self.edit_state = EditState::ES_COMPOSING;
        }

        self.char_caret = self.composition.display_char_count();

        Ok(())
    }

    pub fn focus_next_candidate(&mut self, engine: &EngInner) -> Result<()> {
        if self.edit_state == EditState::ES_COMPOSING {
            self.edit_state = EditState::ES_SELECTING;
        }

        let mut to_focus = match self.focused_cand_idx {
            Some(i) if i >= self.candidates.len() - 1 => 0,
            Some(i) => i + 1,
            None => self.cand_page * 9,
        };

        self.focus_candidate(engine, to_focus);

        Ok(())
    }

    pub fn focus_prev_candidate(&mut self, engine: &EngInner) -> Result<()> {
        let mut to_focus = match self.focused_cand_idx {
            Some(i) if i == 0 => self.candidates.len() - 1,
            Some(i) => i - 1,
            None => self.cand_page * 9,
        };

        self.focus_candidate(engine, to_focus);
        Ok(())
    }

    pub fn show_next_page_candidate(
        &mut self,
        engine: &EngInner,
    ) -> Result<()> {
        if (self.candidates.is_empty()) {
            return Ok(());
        }
        if (self.candidates.len() <= 9) {
            return Ok(());
        }

        let total_page = (self.candidates.len() + 8) / 9;
        let to_page = if self.cand_page >= total_page - 1 {
            0
        } else {
            self.cand_page + 1
        };

        if self.focused_cand_idx.is_none() {
            self.cand_page = to_page;
        } else {
            let mut to_focus = to_page * 9;
            self.focus_candidate(engine, to_focus);
        }
        Ok(())
    }

    pub fn show_prev_page_candidate(
        &mut self,
        engine: &EngInner,
    ) -> Result<()> {
        if (self.candidates.is_empty()) {
            return Ok(());
        }
        if (self.candidates.len() <= 9) {
            return Ok(());
        }

        let total_page = (self.candidates.len() + 8) / 9;
        let to_page = if self.cand_page == 0 {
            total_page - 1
        } else {
            self.cand_page - 1
        };

        if self.focused_cand_idx.is_none() {
            self.cand_page = to_page;
        } else {
            let mut to_focus = to_page * 9;
            self.focus_candidate(engine, to_focus);
        }
        Ok(())
    }

    fn reset_focus(&mut self) {
        self.focused_cand_idx = None;
        self.focused_elem_idx = 0;
        self.cand_page = 0;
    }

    // When focusing a candidate, we must construct the new composition to be
    // displayed in the preedit.
    //
    // Steps:
    // 1. Get the raw text from the candidate
    // 2. Get the remaining text after removing the candidate from the current
    //    composition
    // 4. Make a new composition
    // 5. Add the candidate into the composition
    // 6. Auto-split the remaining text into a Buffer
    // 7. Extend the new composition
    fn focus_candidate(
        &mut self,
        engine: &EngInner,
        index: usize,
    ) -> Result<()> {
        if (engine.conf.input_mode() == InputMode::Classic) {
            return self.focus_candidate_classic(engine, index);
        }
        self.composition.clear_autospace();
        let candidate = self
            .candidates
            .get(index)
            .ok_or(anyhow!("Candidate index out of bounds"))?
            .clone();

        let cand_raw_count = candidate.raw_char_count();
        let comp_raw = self.composition.raw_text();
        let mut remainder =
            comp_raw.char_substr(cand_raw_count, comp_raw.chars().count());

        let mut new_comp = candidate;
        let remainder_split = convert_all(engine, &remainder)?;

        new_comp.extend(remainder_split);

        self.composition = new_comp;

        self.focused_cand_idx = Some(index);
        self.cand_page = index / 9;
        self.composition.autospace();
        self.char_caret = self.composition.display_char_count();

        Ok(())
    }

    fn focus_candidate_classic(
        &mut self,
        engine: &EngInner,
        index: usize,
    ) -> Result<()> {
        if (self.candidates.is_empty()) {
            self.edit_state = EditState::ES_ILLEGAL;
            return Ok(());
        }
        let candidate = self
            .candidates
            .get(index)
            .ok_or(anyhow!("Candidate index out of bounds"))?
            .clone();

        let mut cand_raw_count = candidate.raw_char_count();
        let mut candi_text = candidate.display_text();
        let comp_raw = self.composition.raw_text();
        if comp_raw.starts_with("--") {
            candi_text = format!("--{}", candi_text);
            cand_raw_count = cand_raw_count + 2;
        } else if comp_raw.starts_with("-") {
            candi_text = format!("-{}", candi_text);
            cand_raw_count = cand_raw_count + 1;
        }
        let mut remainder =
            comp_raw.char_substr(cand_raw_count, comp_raw.chars().count());

        candi_text.push_str(&remainder);

        self.composition = Buffer::new();
        self.composition
            .push(StringElem::from_raw_input(comp_raw, candi_text).into());

        self.focused_cand_idx = Some(index);
        self.cand_page = index / 9;
        self.char_caret = self.composition.display_char_count();

        Ok(())
    }
}

// Just for debugging
impl BufferMgr {
    fn _debug_preedit(&self, sep: char) -> String {
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
            EditState::ES_ILLEGAL => "Illegal",
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

        let mut display_text = self._debug_preedit(sep);

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
    use khiin_ji::contains_hanji;

    use super::*;
    use crate::tests::*;
    use crate::utils::Unique;

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
        let (e, mut buf) = test_harness();
        buf.insert_continuous(&e, 'a')?;
        assert_eq!(buf.composition.raw_text().as_str(), "a");
        assert_eq!(buf.composition.display_text().as_str(), "a");
        assert!(buf.candidates.len() > 0);
        assert!(contains_hanji(&buf.candidates[0].display_text()));
        Ok(())
    }

    #[test]
    fn it_focuses_the_first_candidate() -> Result<()> {
        let (e, mut buf) = test_harness();
        buf.insert(&e, 'a')?;
        buf.focus_next_candidate(&e)?;
        let text = preedit_text(&buf);
        assert_eq!(text.as_str(), "亞");
        assert_eq!(buf.focused_cand_idx, Some(0));
        assert_eq!(buf.get_candidates().focused, 0);
        Ok(())
    }

    #[test_log::test]
    fn it_focuses_the_second_candidate() -> Result<()> {
        let (e, mut buf) = test_harness();
        buf.insert(&e, 'a')?;
        buf.focus_next_candidate(&e)?;
        buf.focus_next_candidate(&e)?;
        let text = preedit_text(&buf);
        assert_eq!(text.as_str(), "亜");
        Ok(())
    }

    #[test_log::test]
    fn it_excludes_duplicates() -> Result<()> {
        let (e, mut buf) = test_harness();
        for ch in "pengan".chars().collect::<Vec<char>>() {
            buf.insert(&e, ch)?;
        }

        assert!(buf.candidates.len() > 1);

        let display_texts: Vec<String> =
            buf.candidates.iter().map(|ea| ea.display_text()).collect();

        assert!(display_texts.all_unique());

        Ok(())
    }

    #[test_log::test]
    fn it_positions_caret_at_end_during_composition() -> Result<()> {
        let (e, mut buf) = test_harness();
        for ch in "pengan".chars().collect::<Vec<char>>() {
            buf.insert(&e, ch)?;
        }
        log::debug!("{:?}", buf);
        Ok(())
    }
}
