use super::{BufferElement, buffer_element::BufferElementEnum};

#[derive(Default)]
pub(crate) struct Buffer {
    elems: Vec<BufferElementEnum>,
}

impl Buffer {
    pub fn composition(&self) -> String {
        self.elems.iter().fold(String::default(), |mut acc, elem| {
            acc.push_str(elem.raw_text());
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::StringElem;
    use super::*;

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
