use windows::Win32::{
    Foundation::{HWND, RECT},
    UI::WindowsAndMessaging::GetClientRect,
};

use crate::geometry::{Point, Rect};

pub(crate) mod colors;
pub(crate) mod render_factory;
pub(crate) mod systray;
pub(crate) mod window;
pub(crate) mod wndproc;

mod candidate_window;
mod dpi;
mod dwm;

pub fn client_hit_test(handle: HWND, pt: Point<i32>) -> bool {
    let mut rect = RECT::default();
    unsafe {
        GetClientRect(handle, &mut rect);
    }
    let rect: Rect<i32> = (&rect).into();
    rect.contains(pt)
}
