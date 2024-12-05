use crate::db::models::KeyConversion;
use super::BufferElement;

#[derive(Debug, Clone)]
pub struct ActionElem {
    value: String,
}

impl ActionElem {
    pub fn new() -> Self {
        ActionElem {
            value: "...".to_string(),
        }
    }
}

impl BufferElement for ActionElem {
    fn raw_text(&self) -> String {
        self.value.clone()
    }

    fn raw_char_count(&self) -> usize {
        0
    }

    fn composed_text(&self) -> String {
        self.value.clone()
    }

    fn composed_char_count(&self) -> usize {
        0
    }

    fn display_text(&self) -> String {
        self.value.clone()
    }

    fn display_char_count(&self) -> usize {
        self.value.chars().count()
    }

    fn raw_caret_from(&self, caret: usize) -> usize {
        caret
    }

    fn caret_from(&self, raw_caret: usize) -> usize {
        raw_caret
    }

    fn set_converted(&mut self, converted: bool) {
        // Not impl
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

    // fn insert(&mut self, idx: usize, ch: char) {
    //     // Not impl
    // }

    // fn erase(&mut self, idx: usize) {
    //     if idx == 0 {
    //         self.deleted = true
    //     }
    // }
}
