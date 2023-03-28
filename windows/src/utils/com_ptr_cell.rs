use std::ffi::c_void;
use std::marker::PhantomData;
use std::cell::UnsafeCell;

use windows::core::ComInterface;

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

    pub fn set(&self, obj: &T) {
        let unsafe_cell = self.cell.get();
        unsafe {
            *unsafe_cell = obj.clone().into_raw();
        }
    }

    pub fn get(&self) -> Option<&T> {
        unsafe { T::from_raw_borrowed(&*self.cell.get()) }
    }
}