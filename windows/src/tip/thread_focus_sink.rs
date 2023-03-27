use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfThreadFocusSink;
use windows::Win32::UI::TextServices::ITfThreadFocusSink_Impl;

#[implement(ITfThreadFocusSink)]
struct ThreadFocusSink;

impl ITfThreadFocusSink_Impl for ThreadFocusSink {
    fn OnSetThreadFocus(&self) -> Result<()> {
        todo!()
    }

    fn OnKillThreadFocus(&self) -> Result<()> {
        todo!()
    }
}
