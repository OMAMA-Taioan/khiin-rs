use super::StringElem;
use super::TaiText;

pub trait BufferElement {
    fn raw_text(&self) -> &str;
    
    fn raw_char_count(&self) -> usize;
    
    fn raw_caret_from(&self, caret: usize) -> usize;

    fn composed_text(&self) -> &str;
    
    fn composed_char_count(&self) -> usize;

    fn caret_from(&self, raw_caret: usize) -> usize;

    fn converted(&self) -> &str;

    fn is_converted(&self) -> bool;

    fn is_selected(&self) -> bool;

    fn set_khin(&self);

    fn candidate(&self) -> Option<&str>;

    fn insert(&mut self, idx: usize, ch: char);

    fn erase(&mut self, idx: usize);
}


// pub(crate) enum BufferElement {
//     Plaintext(StringElem),
//     Conversion(TaiText),
// }

// impl From<&str> for BufferElement {
//     fn from(value: &str) -> Self {
//         BufferElement::Plaintext(value.into())
//     }
// }

// impl Insertable for BufferElement {
//     fn raw_char_count(&self) -> usize {
//         match self {
//             BufferElement::Plaintext(elem) => elem.raw_char_count(),
//             BufferElement::Conversion(elem) => elem.raw_char_count(),
//         }
//     }

//     fn composed_char_count(&self) -> usize {
//         match self {
//             BufferElement::Plaintext(elem) => elem.composed_char_count(),
//             BufferElement::Conversion(elem) => elem.composed_char_count(),
//         }
//     }

//     fn insert_at(&mut self, idx: usize, ch: char) {
//         match self {
//             BufferElement::Plaintext(elem) => elem.insert_at(idx, ch),
//             BufferElement::Conversion(elem) => elem.insert_at(idx, ch),
//         }
//     }

//     fn raw(&self) -> &str {
//         match self {
//             BufferElement::Plaintext(elem) => elem.as_str(),
//             BufferElement::Conversion(elem) => elem.raw(),
//         }
//     }
// }
