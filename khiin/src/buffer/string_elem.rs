use std::ops::Deref;
use std::ops::DerefMut;

use crate::buffer::BufferElement;
use crate::data::models::Conversion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringElem {
    value: String,
    converted: bool,
    selected: bool,
}

impl Deref for StringElem {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for StringElem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl From<String> for StringElem {
    fn from(value: String) -> Self {
        Self {
            converted: false,
            selected: false,
            value,
        }
    }
}

impl From<&str> for StringElem {
    fn from(value: &str) -> Self {
        Self {
            converted: false,
            selected: false,
            value: String::from(value),
        }
    }
}

impl BufferElement for StringElem {
    fn raw_char_count(&self) -> usize {
        self.chars().count()
    }

    fn raw_text(&self) -> String {
        self.value.clone()
    }

    fn composed_text(&self) -> String {
        self.value.clone()
    }

    fn composed_char_count(&self) -> usize {
        self.chars().count()
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

    fn is_converted(&self) -> bool {
        self.converted
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn set_khin(&self) {
        // noop
    }

    fn candidate(&self) -> Option<&Conversion> {
        None
    }

    fn insert(&mut self, idx: usize, ch: char) {
        self.value.insert(idx, ch);
    }

    fn erase(&mut self, idx: usize) {
        let start = self.value.char_indices().nth(idx).unwrap().0;
        let end = self.value.char_indices().nth(idx + 1).unwrap().0;
        self.value.replace_range(start..end, "");
    }

    fn set_converted(&mut self, converted: bool) {
        self.converted = converted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let el = StringElem::from("ho");
        assert_eq!(el.raw_text(), "ho");
    }
}
