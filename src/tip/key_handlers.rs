use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::TF_ES_READWRITE;
use windows::Win32::UI::TextServices::TF_ES_SYNC;

use crate::protos::command::Command;
use crate::tip::edit_session::CallbackEditSession;
use crate::tip::key_event::KeyEvent;
use crate::tip::text_service::TextService;

fn handle_commit(
    ec: u32,
    service: &TextService,
    context: &ITfContext,
    command: Command,
) -> Result<()> {
    Ok(())
}

fn prepare_commit(
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
            service.clientid(),
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
    let command = service.engine().on_key(key_event);
    prepare_commit(service, context, command)
}
