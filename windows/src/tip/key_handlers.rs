use windows::core::Result;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::TF_ES_READWRITE;
use windows::Win32::UI::TextServices::TF_ES_SYNC;

use crate::tip::edit_session::CallbackEditSession;
use crate::tip::key_event::KeyEvent;
use crate::tip::text_service::TextService;

fn handle_commit(
    ec: u32,
    service: &TextService,
    context: &ITfContext,
    key_event: &KeyEvent,
) -> Result<()> {
    Ok(())
}

fn prepare_commit(
    service: &TextService,
    context: &ITfContext,
    key_event: KeyEvent,
) {
    let session: ITfEditSession =
        CallbackEditSession::new(|ec| -> Result<()> {
            handle_commit(ec, service, context, &key_event)
        })
        .into();

    let res = unsafe {
        context.RequestEditSession(
            service.clientid(),
            &session,
            TF_ES_SYNC | TF_ES_READWRITE,
        )
    };

    match res {
        Ok(_) => {}
        Err(_) => panic!("Something bad happened!"),
    }
}

pub fn handle_key(
    service: &TextService,
    context: &ITfContext,
    key_event: KeyEvent,
) {
    prepare_commit(service, context, key_event);
}
