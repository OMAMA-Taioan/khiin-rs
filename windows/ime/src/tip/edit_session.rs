use windows::core::implement;
use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfEditSession_Impl;
use windows::Win32::UI::TextServices::TF_ES_READWRITE;
use windows::Win32::UI::TextServices::TF_ES_SYNC;

pub type TfEditCookie = u32;

pub fn open_edit_session<CB>(
    clientid: u32,
    context: ITfContext,
    cb: CB,
) -> Result<()>
where
    CB: Fn(u32) -> Result<()>,
{
    let session: ITfEditSession =
        CallbackEditSession::new(|ec| -> Result<()> { cb(ec) }).into();

    let result = unsafe {
        context.RequestEditSession(
            clientid,
            &session,
            TF_ES_SYNC | TF_ES_READWRITE,
        )
    };

    match result {
        Ok(_) => Ok(()),
        Err(_) => panic!("Something bad happened!"),
    }
}

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
