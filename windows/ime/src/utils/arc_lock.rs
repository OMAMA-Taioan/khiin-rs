use std::sync::Arc;
use std::sync::RwLock;

use windows::core::Result;

use crate::fail;

#[derive(Default)]
pub struct ArcLock<T: Copy> {
    value: Arc<RwLock<T>>,
}

impl<T: Copy> ArcLock<T> {
    pub fn new(value: T) -> Self {
        ArcLock {
            value: Arc::new(RwLock::new(value)),
        }
    }

    pub fn get(&self) -> Result<T> {
        match self.value.read() {
            Ok(guard) => Ok(*guard),
            Err(_) => Err(fail!()),
        }
    }

    pub fn set(&self, value: T) -> Result<()> {
        match self.value.write() {
            Ok(mut guard) => {
                *guard = value;
                return Ok(());
            },
            Err(_) => Err(fail!()),
        }
    }
}
