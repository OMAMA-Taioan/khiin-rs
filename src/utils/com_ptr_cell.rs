use std::ffi::c_void;
use std::marker::PhantomData;
use std::cell::UnsafeCell;

use windows::core::ComInterface;

// When implementing a COM interface, Windows sometimes
// supplies an interface to one method (such as `Activate`)
// that must be held for the interface's lifetime. In particular,
// it may be needed in other methods or places where Windows
// does not supply it again. In these cases, depending on the
// implementation, it may not be possible to directly store
// the reference supplied by Windows, e.g. due to the need
// to specify lifetime parameters which are not available
// when implementing COM interfaces. Therefore, we can use
// an UnsafeCell to directly store the raw pointer.
//
// This feels a bit hacky to me but unless there is a better way,
// this is how we will do it for now.
pub struct ComPtrCell<T: ComInterface> {
    cell: UnsafeCell<*mut c_void>,
    phantom: PhantomData<T>,
}

impl<T: ComInterface> ComPtrCell<T> {
    pub fn new() -> ComPtrCell<T> {
        ComPtrCell {
            cell: UnsafeCell::new(std::ptr::null_mut()),
            phantom: PhantomData,
        }
    }

    pub fn set(&self, obj: T) {
        let unsafe_cell = self.cell.get();
        unsafe {
            *unsafe_cell = obj.into_raw();
        }
    }

    pub fn get(&self) -> T {
        unsafe { T::from_raw(*self.cell.get()) }
    }
}
