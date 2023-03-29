use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfContextView;
use windows::Win32::UI::TextServices::ITfTextLayoutSink;
use windows::Win32::UI::TextServices::ITfTextLayoutSink_Impl;
use windows::Win32::UI::TextServices::TfLayoutCode;

#[implement(ITfTextLayoutSink)]
struct TextLayoutSink;

impl ITfTextLayoutSink_Impl for TextLayoutSink {
    fn OnLayoutChange(
        &self,
        pic: Option<&ITfContext>,
        lcode: TfLayoutCode,
        pview: Option<&ITfContextView>,
    ) -> Result<()> {
        todo!()
    }
}
