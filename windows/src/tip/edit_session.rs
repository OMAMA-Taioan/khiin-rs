use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfEditSession_Impl;

use super::text_service::TextService;

// type EditSessionCb = Fn(u32) -> Result<()>;

pub fn do_composition(
    ec: u32,
    service: &TextService,
    context: &ITfContext,
) -> Result<()> {
    // here is where we can edit the text
    Ok(())
}

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
