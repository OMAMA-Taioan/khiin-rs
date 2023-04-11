use std::cell::RefCell;
use std::mem::transmute;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::JoinHandle;

use protobuf::Message;
use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;

use khiin_protos::command::*;

use crate::winerr;

use super::message_handler::WM_KHIIN_COMMAND;

struct EngineCoordinator {
    engine: khiin::Engine,
}

impl EngineCoordinator {
    fn new() -> Result<Self> {
        if let Some(engine) = khiin::Engine::new() {
            Ok(Self { engine })
        } else {
            winerr!(E_FAIL)
        }
    }

    fn send_command(&mut self, cmd: Command) -> Result<Command> {
        let bytes = cmd.write_to_bytes().map_err(|_| Error::from(E_FAIL))?;
        let cmd = self
            .engine
            .send_command_bytes(&bytes)
            .map_err(|_| Error::from(E_FAIL))?;
        Command::parse_from_bytes(&cmd).map_err(|_| Error::from(E_FAIL))
    }
}

pub struct AsyncEngine {
    tx: Sender<Command>,
    thread: RefCell<Option<JoinHandle<()>>>,
}

impl AsyncEngine {
    pub fn run(callback_handle: HWND) -> Result<Self> {
        let (tx, rx) = channel::<Command>();
        let thread = std::thread::spawn(move || {
            let engine = EngineCoordinator::new();
            if engine.is_err() {
                return;
            }
            let mut engine = engine.unwrap();

            loop {
                match rx.recv() {
                    Ok(cmd) => {
                        if cmd.request.type_.enum_value().unwrap()
                            == CommandType::CMD_SHUTDOWN
                        {
                            break;
                        }

                        let cmd = engine.send_command(cmd);

                        if cmd.is_err() {
                            continue;
                        }

                        let raw_ptr = Arc::into_raw(Arc::new(cmd.unwrap()));
                        let wparam = WPARAM(0);

                        unsafe {
                            let lparam: LPARAM = transmute(raw_ptr);
                            PostMessageW(
                                callback_handle,
                                WM_KHIIN_COMMAND,
                                wparam,
                                lparam,
                            );
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        Ok(Self {
            tx,
            thread: RefCell::new(Some(thread)),
        })
    }

    pub fn sender(&self) -> Sender<Command> {
        self.tx.clone()
    }

    pub fn shutdown(&self) -> Result<()> {
        let mut cmd = Command::default();
        let mut req = Request::default();
        req.type_ = CommandType::CMD_SHUTDOWN.into();
        cmd.request = Some(req).into();

        self.tx.send(cmd).map_err(|_| Error::from(E_FAIL))?;
        let thread = self.thread.replace(None).unwrap();
        thread.join().map_err(|_| Error::from(E_FAIL))
    }
}
