use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;
use windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS;

pub(crate) mod candidates;
pub(crate) mod colors;
pub(crate) mod render_factory;
pub(crate) mod systray;
pub(crate) mod window;
pub(crate) mod wndproc;

pub(crate) use render_factory::RenderFactory;

mod dpi;
mod dwm;

pub unsafe fn vcenter_textlayout(
    layout: &IDWriteTextLayout,
    available_height: f32,
) -> f32 {
    let mut metrics = DWRITE_TEXT_METRICS::default();
    layout.GetMetrics(&mut metrics).ok();
    (available_height - metrics.height) / 2.0
}
