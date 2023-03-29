use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfEditSession_Impl;

#[implement(ITfEditSession)]
pub struct CallbackEditSession<CB>
where
    CB: Fn(u32) -> Result<()>,
{
    callback: CB,
}

impl<CB> CallbackEditSession<CB>
where
    CB: Fn(u32) -> Result<()>,
{
    pub fn new(callback: CB) -> Self {
        CallbackEditSession { callback }
    }
}

impl<CB> ITfEditSession_Impl for CallbackEditSession<CB>
where
    CB: Fn(u32) -> Result<()>,
{
    fn DoEditSession(&self, ec: u32) -> Result<()> {
        (self.callback)(ec)
    }
}
