use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfEditSession_Impl;

#[implement(ITfEditSession)]
struct EditSession;

impl ITfEditSession_Impl for EditSession {
    fn DoEditSession(&self, ec: u32) -> Result<()> {
        todo!()
    }
}
