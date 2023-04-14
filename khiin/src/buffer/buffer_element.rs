use enum_dispatch::enum_dispatch;

use super::StringElem;
use super::TaiText;

#[enum_dispatch(BufferElement)]
pub enum BufferElementEnum {
    StringElem,
    TaiText,
}

#[enum_dispatch]
pub trait BufferElement {
    fn raw_text(&self) -> &str;

    fn raw_len(&self) -> usize;

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
