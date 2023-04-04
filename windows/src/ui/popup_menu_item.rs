use windows::Win32::Graphics::{
    Direct2D::Common::D2D_RECT_F, DirectWrite::IDWriteTextLayout,
};

use crate::geometry::Rect;

#[derive(Default, Clone)]
pub struct PopupMenuItem {
    pub separator: bool,
    pub checked: bool,
    pub icon_rid: u32,
    pub text_rid: u32,
    pub rect: Rect<i32>,
    pub layout: Option<IDWriteTextLayout>,
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

    pub fn d2d_rect_f(&self) -> D2D_RECT_F {
        let r = &self.rect;

        D2D_RECT_F {
            left: r.w() as f32,
            top: r.n() as f32,
            right: r.e() as f32,
            bottom: r.s() as f32,
        }
    }
}
