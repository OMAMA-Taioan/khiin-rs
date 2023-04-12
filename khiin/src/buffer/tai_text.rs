use crate::input::Syllable;

use super::BufferElement;

pub struct TaiText {
    elems: Vec<Syllable>,
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
