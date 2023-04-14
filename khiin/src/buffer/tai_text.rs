use crate::data::models::Conversion;
use crate::input::{Syllable, Tone};

use super::BufferElement;

const SYL_SEPS: [char; 2] = ['-', ' '];

pub struct TaiText {
    elems: Vec<Syllable>,
}

fn get_first_syllable(target: &str) -> &str {
    for (i, c) in target.char_indices() {
        if SYL_SEPS.contains(&c) {
            return &target[..i];
        }
    }
    target
}

impl TaiText {
    pub fn from_conversion(raw_input: &str, conv: Conversion) -> Self {
        // let syls = Vec::new();
        let target = conv.input.as_str();

        for target_syl in target.split(&SYL_SEPS) {
            let s = Syllable::from_composed(target_syl);
            
        }

        todo!()
    }
}

impl BufferElement for TaiText {
    fn raw_text(&self) -> &str {
        todo!()
    }

    fn raw_char_count(&self) -> usize {
        todo!()
    }

    fn raw_caret_from(&self, caret: usize) -> usize {
        todo!()
    }

    fn composed_text(&self) -> &str {
        todo!()
    }

    fn composed_char_count(&self) -> usize {
        todo!()
    }

    fn caret_from(&self, raw_caret: usize) -> usize {
        todo!()
    }

    fn converted(&self) -> &str {
        todo!()
    }

    fn is_converted(&self) -> bool {
        todo!()
    }

    fn is_selected(&self) -> bool {
        todo!()
    }

    fn set_khin(&self) {
        todo!()
    }

    fn candidate(&self) -> Option<&str> {
        todo!()
    }

    fn insert(&mut self, idx: usize, ch: char) {
        todo!()
    }

    fn erase(&mut self, idx: usize) {
        todo!()
    }
}
