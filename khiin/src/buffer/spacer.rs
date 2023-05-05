use crate::db::models::KeyConversion;

use super::BufferElement;
#[derive(Debug, Clone)]
pub struct Spacer {
    pub deleted: bool,
    converted: bool,
}

impl Spacer {
    pub fn new() -> Self {
        Spacer {
            converted: false,
            deleted: false,
        }
    }
}

impl BufferElement for Spacer {
    fn raw_text(&self) -> String {
        String::new()
    }

    fn raw_char_count(&self) -> usize {
        0
    }

    fn composed_text(&self) -> String {
        if self.deleted {
            String::new()
        } else {
            String::from(" ")
        }
    }

    fn composed_char_count(&self) -> usize {
        if self.deleted {
            0
        } else {
            1
        }
    }

    fn display_text(&self) -> String {
        self.composed_text()
    }

    fn display_char_count(&self) -> usize {
        self.composed_char_count()
    }

    fn raw_caret_from(&self, caret: usize) -> usize {
        caret
    }

    fn caret_from(&self, raw_caret: usize) -> usize {
        raw_caret
    }

    fn set_converted(&mut self, converted: bool) {
        self.converted = converted;
    }

    fn is_converted(&self) -> bool {
        true
    }

    fn is_selected(&self) -> bool {
        false
    }

    fn set_khin(&self) {
        // Not impl
    }

    fn candidate(&self) -> Option<&KeyConversion> {
        None
    }

    fn insert(&mut self, idx: usize, ch: char) {
        // Not impl
    }

    fn erase(&mut self, idx: usize) {
        if idx == 0 {
            self.deleted = true
        }
    }
}
