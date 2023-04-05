use std::cell::RefCell;
use std::sync::Arc;

use windows::core::Result;

use khiin_protos::command::Command;

use super::candidate_window::DisplayMode;

#[derive(Default)]
pub struct Pager {
    pub candidate_list: RefCell<Arc<Command>>,
    pub display_mode: DisplayMode,
    pub focused_id: i32,
    pub focused_index: usize,
    pub focused_col: usize,
}

impl Pager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_candidate_list(&self, cl: Arc<Command>) -> Result<()> {
        Ok(())
    }

    pub fn set_focus(&self, index: i32) -> Result<()> {
        Ok(())
    }
}
