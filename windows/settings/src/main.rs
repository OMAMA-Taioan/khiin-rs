#![cfg(windows)]

use windows::Win32::UI::Controls::InitCommonControls;

pub fn main() {
    unsafe {
        InitCommonControls();
    }

    println!("Hello, world!");
}
