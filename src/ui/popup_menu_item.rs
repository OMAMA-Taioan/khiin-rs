use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;

use crate::geometry::rect::Rect;

#[derive(Default)]
pub struct PopupMenuItem {
    separator: bool,
    checked: bool,
    icon_rid: u32,
    text_rid: u32,
    rect: Rect<u32>,
    layout: Option<IDWriteTextLayout>,
}

impl PopupMenuItem {
    pub fn button(text_rid: u32, icon_rid: u32, checked: bool) -> Self {
        Self {
            separator: false,
            checked,
            icon_rid,
            text_rid,
            rect: Rect::default(),
            layout: None,
        }
    }

    pub fn sep() -> Self {
        let mut item = Self::default();
        item.separator = true;
        item
    }
}
