use std::mem::transmute;

use khiin_windows::utils::pcwstr::ToPcwstr;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::WindowsAndMessaging::CB_ADDSTRING;
use windows::Win32::UI::WindowsAndMessaging::CB_SETCURSEL;
use windows::Win32::UI::WindowsAndMessaging::SendMessageW;
use windows::Win32::UI::WindowsAndMessaging::CB_RESETCONTENT;
use windows::Win32::UI::WindowsAndMessaging::SetWindowTextW;

// These are some functions (actually macros) defined in windowsx.h
// ported here for better usability, since they are not included
// in the windows crate itself.

#[inline]
pub fn ComboBox_ResetContent(hwndCtl: HWND) {
    unsafe {
        SendMessageW(hwndCtl, CB_RESETCONTENT, WPARAM(0), LPARAM(0));
    }
}

#[inline]
pub fn ComboBox_AddString(hwndCtl: HWND, string: &str) {
    let pcwstr = string.to_pcwstr();
    unsafe {
        SendMessageW(
            hwndCtl,
            CB_ADDSTRING,
            WPARAM(0),
            LPARAM(transmute(*pcwstr)),
        );
    }
}

#[inline]
pub fn ComboBox_SetCurSel(hwndCtl: HWND, index: usize) {
    unsafe {
        SendMessageW(hwndCtl, CB_SETCURSEL, WPARAM(index), LPARAM(0));
    }
}

#[inline]
pub fn Static_SetText(hwndCtl: HWND, string: &str) {
    let pcwstr = string.to_pcwstr();
    unsafe {
        SetWindowTextW(hwndCtl, *pcwstr);
    }
}
