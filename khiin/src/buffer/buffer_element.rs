use std::fmt::Debug;

use enum_dispatch::enum_dispatch;

use super::StringElem;
use super::KhiinElem;

#[enum_dispatch(BufferElement)]
pub enum BufferElementEnum {
    StringElem,
    KhiinElem,
}

impl Debug for BufferElementEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringElem(arg0) => f.debug_tuple("StringElem").field(arg0).finish(),
            Self::KhiinElem(arg0) => f.debug_tuple("KhiinElem").field(arg0).finish(),
        }
    }
}

#[enum_dispatch]
pub trait BufferElement {
    fn raw_text(&self) -> String;

    fn raw_len(&self) -> usize;

    fn raw_char_count(&self) -> usize;

    fn raw_caret_from(&self, caret: usize) -> usize;

    fn composed_text(&self) -> String;

    fn composed_char_count(&self) -> usize;

    fn caret_from(&self, raw_caret: usize) -> usize;

    fn converted(&self) -> String;

    fn is_converted(&self) -> bool;

    fn is_selected(&self) -> bool;

    fn set_khin(&self);

    fn candidate(&self) -> Option<&str>;

    fn insert(&mut self, idx: usize, ch: char);

    fn erase(&mut self, idx: usize);
}
