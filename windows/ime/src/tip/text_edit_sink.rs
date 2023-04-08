use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditRecord;
use windows::Win32::UI::TextServices::ITfTextEditSink;
use windows::Win32::UI::TextServices::ITfTextEditSink_Impl;

#[implement(ITfTextEditSink)]
struct TextEditSink;

impl ITfTextEditSink_Impl for TextEditSink {
    fn OnEndEdit(
        &self,
        pic: Option<&ITfContext>,
        ecreadonly: u32,
        peditrecord: Option<&ITfEditRecord>,
    ) -> Result<()> {
        todo!()
    }
}
