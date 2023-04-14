use super::buffer_element::BufferElementEnum;
use super::BufferElement;

#[derive(Default)]
pub struct Buffer {
    elems: Vec<BufferElementEnum>,
}

impl Buffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> BufferIter<'_> {
        BufferIter { elems: &self.elems, index: 0 }
    }

    pub fn push(&mut self, elem: BufferElementEnum) {
        self.elems.push(elem)
    }

    pub fn composition(&self) -> String {
        self.elems.iter().fold(String::default(), |mut acc, elem| {
            acc.push_str(elem.raw_text());
            acc
        })
    }
}

pub struct BufferIter<'a> {
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
        assert_eq!(buf.composition().as_str(), "ho");
    }
}
