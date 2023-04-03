use std::sync::Arc;

use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::core::AsImpl;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::TF_ES_READWRITE;
use windows::Win32::UI::TextServices::TF_ES_SYNC;

use khiin_protos::command::Command;

use crate::tip::edit_session::CallbackEditSession;
use crate::tip::key_event::KeyEvent;
use crate::tip::text_service::TextService;
use crate::winerr;

fn open_session_for_commit(
    tip: ITfTextInputProcessor,
    context: ITfContext,
    command: Arc<Command>,
) -> Result<()> {
    let session: ITfEditSession =
        CallbackEditSession::new(|ec| -> Result<()> {
            let service = tip.as_impl();
            service.notify_command(ec, context.clone(), command.clone())
        })
        .into();

    let service = tip.as_impl();
    let result = unsafe {
        context.RequestEditSession(
            service.clientid()?,
            &session,
            TF_ES_SYNC | TF_ES_READWRITE,
        )
    };

    match result {
        Ok(_) => Ok(()),
        Err(_) => panic!("Something bad happened!"),
    }
}

pub fn handle_key(
    tip: ITfTextInputProcessor,
    context: ITfContext,
    key_event: KeyEvent,
) -> Result<()> {
    let service = tip.as_impl();
    if let Ok(engine) = service.engine().read() {
        let command = engine.on_key(key_event)?;
        open_session_for_commit(tip, context, command)
    } else {
        winerr!(E_FAIL)
    }
}
