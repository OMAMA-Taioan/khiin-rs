use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfEditSession_Impl;

use super::text_service::TextService;

pub fn do_composition(
    ec: u32,
    service: &TextService,
    context: &ITfContext,
) -> Result<()> {
    // here is where we can edit the text
    Ok(())
}

#[implement(ITfEditSession)]
pub struct CallbackEditSession<'a> {
    callback: Box<dyn Fn(u32) -> Result<()> + 'a>,
}

impl<'a> CallbackEditSession<'a> {
    pub fn new(callback: impl Fn(u32) -> Result<()> + 'a) -> Self {
        CallbackEditSession {
            callback: Box::new(callback),
        }
    }
}

impl<'a> ITfEditSession_Impl for CallbackEditSession<'a> {
    fn DoEditSession(&self, ec: u32) -> Result<()> {
        (self.callback)(ec)
    }
}
