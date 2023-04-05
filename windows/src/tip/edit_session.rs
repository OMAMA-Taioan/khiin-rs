use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfEditSession_Impl;

pub type TfEditCookie = u32;

#[implement(ITfEditSession)]
pub struct CallbackEditSession<CB>
where
    CB: Fn(TfEditCookie) -> Result<()>,
{
    callback: CB,
}

impl<CB> CallbackEditSession<CB>
where
    CB: Fn(TfEditCookie) -> Result<()>,
{
    pub fn new(callback: CB) -> Self {
        CallbackEditSession { callback }
    }
}

impl<CB> ITfEditSession_Impl for CallbackEditSession<CB>
where
    CB: Fn(TfEditCookie) -> Result<()>,
{
    fn DoEditSession(&self, ec: TfEditCookie) -> Result<()> {
        (self.callback)(ec)
    }
}
