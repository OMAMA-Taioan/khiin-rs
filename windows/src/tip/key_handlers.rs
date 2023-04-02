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

fn handle_commit(
    ec: u32,
    service: &TextService,
    context: &ITfContext,
    command: Command,
) -> Result<()> {
    Ok(())
}

fn open_session_for_commit(
    service: &TextService,
    context: &ITfContext,
    command: Command,
) -> Result<()> {
    let session: ITfEditSession =
        CallbackEditSession::new(|ec| -> Result<()> {
            handle_commit(ec, service, context, command.clone())
        })
        .into();

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
    service: &TextService,
    context: &ITfContext,
    key_event: KeyEvent,
) -> Result<()> {
    if let Ok(guard) = service.engine().read() {
        guard
            .as_ref()
            .and_then(|engine| Some(engine.on_key(key_event)))
            .and_then(|command| Some(open_session_for_commit(service, context, command)))
            .unwrap_or_else(|| winerr!(E_FAIL))
    } else {
        winerr!(E_FAIL)
    }
}
