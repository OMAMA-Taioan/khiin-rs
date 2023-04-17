use std::ops::Deref;
use std::ops::DerefMut;

use crate::buffer::BufferElement;
use crate::buffer::BufferElementEnum;

#[derive(Default, Debug, Clone)]
pub(crate) struct Buffer {
    elems: Vec<BufferElementEnum>,
}

impl Deref for Buffer {
    type Target = Vec<BufferElementEnum>;

    fn deref(&self) -> &Self::Target {
        &self.elems
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elems
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn split_off(&mut self, index: usize) -> Buffer {
        let elems = self.split_off(index);
        Buffer::from(elems)
    }

    pub fn eq_display(&self, other: &Buffer) -> bool {
        self.display_text() == other.display_text()
    }

    pub fn iter(&self) -> BufferIter<'_> {
        BufferIter {
            elems: &self.elems,
            index: 0,
        }
    }

    pub fn raw_text(&self) -> String {
        self.elems.iter().fold(String::default(), |mut acc, elem| {
            acc.push_str(elem.raw_text().as_str());
            acc
        })
    }

    pub fn raw_char_count(&self) -> usize {
        self.elems
            .iter()
            .fold(0, |acc, elem| acc + elem.raw_char_count())
    }

    pub fn display_text(&self) -> String {
        self.elems.iter().fold(String::default(), |mut acc, elem| {
            if elem.is_converted() {
                acc.push_str(&elem.converted_text());
            } else {
                acc.push_str(&elem.composed_text());
            }

            acc
        })
    }

    pub fn composed_char_count(&self) -> usize {
        self.elems
            .iter()
            .fold(0, |acc, elem| acc + elem.composed_char_count())
    }

    pub fn set_converted(&mut self, converted: bool) {
        for elem in self.elems.iter_mut() {
            elem.set_converted(converted);
        }
    }

    // Example carets:
    // raw:       "pengan"  
    // composed:  "pengan" 6
    // converted: "平安"    2 
    pub fn raw_caret_from(&self, char_caret: usize) -> usize {
        if char_caret == self.composed_char_count() {
            return self.raw_char_count()
        }

        let caret_at_index = self.elem_index_at_char_caret(char_caret);

        assert!(caret_at_index < self.elems.len());

        let mut pre = self.clone();

        // Index 0 covered by the assert above
        let at = pre.split_off(caret_at_index);
        let at = &at[0];

        let pre_caret_char_count = pre.display_text().chars().count();

        let remainder = char_caret - pre_caret_char_count;
        
        pre.raw_char_count() + at.raw_caret_from(remainder)
    }

    fn elem_index_at_char_caret(&self, char_caret: usize) -> usize {
        let mut remainder = char_caret;
        let mut index = 0;
        for (i, elem) in self.elems.iter().enumerate() {
            index = i;
            let elem_char_count = elem.composed_char_count();
            if remainder > elem_char_count {
                remainder -= elem_char_count;
            } else {
                break;
            }
        }

        index
    }

    
}

impl From<BufferElementEnum> for Buffer {
    fn from(value: BufferElementEnum) -> Self {
        let mut buf = Self::new();
        buf.push(value);
        buf
    }
}

impl From<Vec<BufferElementEnum>> for Buffer {
    fn from(value: Vec<BufferElementEnum>) -> Self {
        let mut buf = Self::new();
        for elem in value.into_iter() {
            buf.push(elem);
        }
        buf
    }
}

pub(crate) struct BufferIter<'a> {
    elems: &'a Vec<BufferElementEnum>,
    index: usize,
}

impl<'a> Iterator for BufferIter<'a> {
    type Item = &'a BufferElementEnum;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.elems.len() {
            let elem = &self.elems[self.index];
            self.index += 1;
            Some(elem)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::StringElem;

    #[test]
    fn it_works() {
        let buf = Buffer::default();
        assert_eq!(buf.elems.len(), 0);
    }

    #[test]
    fn foo() {
        let mut buf = Buffer::default();
        let el = StringElem::from("ho");
        buf.elems.push(el.into());
        assert_eq!(buf.raw_text().as_str(), "ho");
    }
}
