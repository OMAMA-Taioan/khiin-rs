use std::cell::RefCell;
use std::rc::Rc;

use windows::core::Result;

use crate::fail;

pub trait CloneInner<T>
where
    T: Clone,
{
    fn try_clone_inner(&self) -> Result<T>;
}

impl<T> CloneInner<T> for Rc<RefCell<T>>
where
    T: Clone,
{
    fn try_clone_inner(&self) -> Result<T> {
        Ok(self.try_borrow().map_err(|_| fail!())?.clone())
    }
}
