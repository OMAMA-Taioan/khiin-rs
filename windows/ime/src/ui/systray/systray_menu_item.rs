use windows::Win32::Graphics::Direct2D::Common::D2D_RECT_F;
use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;

use crate::geometry::Rect;

#[derive(Default, Clone)]
pub struct SystrayMenuItem {
    pub separator: bool,
    pub checked: bool,
    pub icon_rid: u16,
    pub string_key: String,
    pub rect: Rect<i32>,
    pub layout: Option<IDWriteTextLayout>,
}

impl SystrayMenuItem {
    pub fn button(string_key: &str, icon_rid: u16, checked: bool) -> Self {
        Self {
            separator: false,
            checked,
            icon_rid,
            string_key: string_key.to_owned(),
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
            left: r.left() as f32,
            top: r.top() as f32,
            right: r.right() as f32,
            bottom: r.bottom() as f32,
        }
    }
}
