use std::sync::Arc;

use windows::core::AsImpl;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::ITfContext;
use windows::Win32::UI::TextServices::ITfEditSession;
use windows::Win32::UI::TextServices::ITfTextInputProcessor;
use windows::Win32::UI::TextServices::TF_ES_READWRITE;
use windows::Win32::UI::TextServices::TF_ES_SYNC;

use khiin_protos::command::Command;

use crate::tip::edit_session::CallbackEditSession;
use crate::tip::key_event::KeyEvent;
use crate::winerr;

fn handle_composition(
    tip: ITfTextInputProcessor,
    context: ITfContext,
    command: Arc<Command>,
) -> Result<()> {
    let session: ITfEditSession =
        CallbackEditSession::new(|ec| -> Result<()> {
            let service = tip.as_impl();
            service.handle_composition(ec, context.clone(), command.clone())
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

fn handle_candidates(
    tip: ITfTextInputProcessor,
    context: ITfContext,
    command: Arc<Command>,
) -> Result<()> {
    let session: ITfEditSession =
        CallbackEditSession::new(|ec| -> Result<()> {
            let service = tip.as_impl();
            service.handle_composition(ec, context.clone(), command.clone())
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
    let temp = service.engine();
    let engine = temp.read().map_err(|_| Error::from(E_FAIL))?;
    let command = engine.on_key(key_event)?;
    handle_composition(tip.clone(), context.clone(), command.clone())?;
    handle_candidates(tip, context, command)
}
