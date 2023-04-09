use std::rc::Rc;

use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::LPARAM;
use windows::Win32::Foundation::WPARAM;
use windows::Win32::UI::Controls::PROPSHEETPAGEW;

pub trait Handler {
    fn set_handle(&self, handle: HWND);

    fn on_message(
        &self,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> isize {
        0
    }
}

// cf Appending additional payload to a PROPSHEETPAGE structure
// https://devblogs.microsoft.com/oldnewthing/20211124-00/?p=105961
#[repr(C)]
pub struct PropSheetPage<T>
where
    T: Handler,
{
    pub winapi: PROPSHEETPAGEW,
    pub handler: Rc<T>,
}

impl<T> core::ops::Deref for PropSheetPage<T>
where
    T: Handler,
{
    type Target = PROPSHEETPAGEW;
    fn deref(&self) -> &Self::Target {
        &self.winapi
    }
}

impl<T> core::ops::DerefMut for PropSheetPage<T>
where
    T: Handler,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.winapi
    }
}

impl<T> PropSheetPage<T>
where
    T: Handler,
{
    pub fn as_winapi(&mut self) -> *mut PROPSHEETPAGEW {
        let p: *mut Self = self;
        p.cast()
    }
}
