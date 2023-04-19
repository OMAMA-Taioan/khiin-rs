use std::fmt::Debug;

use enum_dispatch::enum_dispatch;

use crate::buffer::KhiinElem;
use crate::buffer::StringElem;
use crate::data::models::Conversion;

#[enum_dispatch(BufferElement)]
#[derive(Debug, Clone)]
pub(crate) enum BufferElementEnum {
    StringElem,
    KhiinElem,
}

#[enum_dispatch]
pub trait BufferElement {
    fn raw_text(&self) -> String;
    fn raw_char_count(&self) -> usize;

    fn composed_text(&self) -> String;
    fn composed_char_count(&self) -> usize;

    fn display_text(&self) -> String;
    fn display_char_count(&self) -> usize;

    fn raw_caret_from(&self, caret: usize) -> usize;
    fn caret_from(&self, raw_caret: usize) -> usize;

    fn set_converted(&mut self, converted: bool);
    fn is_converted(&self) -> bool;
    fn is_selected(&self) -> bool;

    fn set_khin(&self);

    fn candidate(&self) -> Option<Conversion>;

    fn insert(&mut self, idx: usize, ch: char);
    fn erase(&mut self, idx: usize);
}
