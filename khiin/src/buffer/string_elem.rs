use std::ops::Deref;
use std::ops::DerefMut;

use crate::db::models::KeyConversion;

use crate::buffer::BufferElement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringElem {
    value: String,
    keys: String,
    converted: bool,
    selected: bool,
}

impl StringElem {
    pub fn from_raw_input(raw_input: String, value: String) -> Self {
        Self {
            converted: false,
            selected: false,
            keys: raw_input,
            value,
        }
    }
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
            keys: value.clone(),
            value,
        }
    }
}

impl From<&str> for StringElem {
    fn from(value: &str) -> Self {
        Self {
            converted: false,
            selected: false,
            keys: String::from(value),
            value: String::from(value),
        }
    }
}

impl BufferElement for StringElem {
    fn raw_char_count(&self) -> usize {
        self.keys.chars().count()
    }

    fn raw_text(&self) -> String {
        self.keys.clone()
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

    fn candidate(&self) -> Option<&KeyConversion> {
        None
    }

    // fn insert(&mut self, idx: usize, ch: char) {
    //     self.value.insert(idx, ch);
    // }

    // fn erase(&mut self, idx: usize) {
    //     let start = self.value.char_indices().nth(idx).unwrap().0;
    //     let end = self.value.char_indices().nth(idx + 1).unwrap().0;
    //     self.value.replace_range(start..end, "");
    // }

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
