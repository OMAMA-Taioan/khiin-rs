use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;
use windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;

use crate::geometry::Point;
use crate::geometry::Rect;

pub use render_factory::RenderFactory;

pub(crate) mod candidates;
pub(crate) mod colors;
pub(crate) mod render_factory;
pub(crate) mod systray;
pub(crate) mod window;
pub(crate) mod wndproc;

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

pub unsafe fn vcenter_textlayout(
    layout: &IDWriteTextLayout,
    available_height: f32,
) -> f32 {
    let mut metrics = DWRITE_TEXT_METRICS::default();
    layout.GetMetrics(&mut metrics).ok();
    (available_height - metrics.height) / 2.0
}
