use std::mem::size_of;

use windows::Win32::Foundation::LRESULT;
use windows::Win32::UI::WindowsAndMessaging::DLGPROC;
use windows::Win32::UI::WindowsAndMessaging::WINDOW_LONG_PTR_INDEX as WLPI;

// WinUser.h
// #define DWLP_MSGRESULT  0
// #define DWLP_DLGPROC    DWLP_MSGRESULT + sizeof(LRESULT)
// #define DWLP_USER       DWLP_DLGPROC + sizeof(DLGPROC)

pub static DWLP_MSGRESULT: i32 = 0;
pub static DWLP_DLGPROC: i32 = DWLP_MSGRESULT + size_of::<LRESULT>() as i32;
pub static DWLP_USER: WLPI = WLPI(DWLP_DLGPROC + size_of::<DLGPROC>() as i32);

pub const ID_APPLY_NOW: u32 = 0x3021;
pub const PCSB_INITIALIZED: u32 = 1;
pub const PCSB_PRECREATE: u32 = 2;
pub const PSCB_BUTTONPRESSED: u32 = 3;
