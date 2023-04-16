use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
use std::sync::Arc;

use khiin_protos::command::EditState;
use windows::core::Result;

use khiin_protos::command::Candidate;
use khiin_protos::command::Command;

use super::candidate_window::DisplayMode;

static SHORT_COL_SIZE: usize = 5;
static LONG_COL_SIZE: usize = 10;
static NUM_GRID_COLS: usize = 4;

pub type CandidateCols = Vec<Vec<Rc<Candidate>>>;

#[derive(Default)]
pub struct CandidatePage {
    pub display_mode: DisplayMode,
    pub focused_id: i32,
    pub focused_index: usize,
    pub focused_col: usize,
    pub quickselect_active: bool,
    pub candidates: CandidateCols,
}

#[derive(Default)]
pub struct Pager {
    pub command: Arc<Command>,
    pub num_candidates: usize,
    pub display_mode: RefCell<DisplayMode>,
    pub focused_id: RefCell<i32>,
    pub focused_index: RefCell<usize>,
}

impl Pager {
    pub fn new(command: Arc<Command>) -> Self {
        let num_candidates = command.response.candidate_list.candidates.len();

        Self {
            command,
            num_candidates,
            display_mode: RefCell::new(DisplayMode::default()),
            focused_id: RefCell::new(0),
            focused_index: RefCell::new(0),
        }
    }

    pub fn get_page(&self) -> CandidatePage {
        let mut grid: Vec<Vec<Rc<Candidate>>> = Vec::new();

        if self.num_candidates == 0 {
            return CandidatePage::default();
        }

        let candidates = self.candidates();
        let mut start = self.start_index();
        let end = self.end_index();
        let mut col: Vec<Rc<Candidate>> = Vec::new();
        for (i, candidate) in candidates
            .iter()
            .skip(start)
            .take(end - start + 1)
            .enumerate()
        {
            if i == start + self.max_col_size() {
                grid.push(col);
                col = Vec::new();
                start = i
            }

            col.push(candidate.clone().into())
        }
        grid.push(col);

        let quickselect_active =
            self.command.response.edit_state.enum_value_or_default()
                == EditState::ES_SELECTING;

        CandidatePage {
            display_mode: self.display_mode.borrow().clone(),
            focused_id: self.focused_id.borrow().clone(),
            focused_index: self.focused_index(),
            focused_col: self.focused_col(),
            quickselect_active,
            candidates: grid,
        }
    }

    pub fn set_focus(&self, focused_id: i32) -> Result<()> {
        self.focused_id.replace(focused_id);

        for (i, candidate) in self.candidates().iter().enumerate() {
            if candidate.id as i32 == focused_id {
                self.focused_index.replace(i);
                break;
            }
        }

        if self.focused_index() >= SHORT_COL_SIZE {
            self.display_mode.replace(DisplayMode::LongColumn);
        }

        Ok(())
    }

    pub fn candidate_count(&self) -> usize {
        self.num_candidates
    }

    pub fn page_count(&self) -> usize {
        let n = self.num_candidates;
        let p = self.max_page_size();
        (n + p - 1) / p
    }

    pub fn max_page_size(&self) -> usize {
        self.max_cols_per_page() * self.max_col_size()
    }

    pub fn current_page(&self) -> usize {
        self.focused_index().div_euclid(self.max_page_size())
    }

    pub fn focused_col(&self) -> usize {
        (self.focused_index() - self.start_index()) / self.max_col_size()
    }

    // internal helpers
    fn candidates(&self) -> &Vec<Candidate> {
        &self.command.response.candidate_list.candidates
    }

    fn focused_index(&self) -> usize {
        *self.focused_index.borrow()
    }

    fn candidate_id_at_index(&self, idx: usize) -> Option<i32> {
        if 0 <= idx && idx < self.num_candidates {
            Some(self.candidates().get(idx)?.id)
        } else {
            None
        }
    }

    fn max_cols_per_page(&self) -> usize {
        match &*self.display_mode.borrow() {
            DisplayMode::Grid => NUM_GRID_COLS,
            _ => 1,
        }
    }

    fn max_col_size(&self) -> usize {
        match &*self.display_mode.borrow() {
            DisplayMode::ShortColumn => SHORT_COL_SIZE,
            _ => LONG_COL_SIZE,
        }
    }

    fn current_col(&self) -> usize {
        self.focused_index().div_euclid(self.max_col_size())
    }

    fn total_cols(&self) -> usize {
        let n = self.num_candidates;
        let d = self.max_col_size();
        (n + d - 1) / d
    }

    fn start_index(&self) -> usize {
        self.max_page_size() * self.current_page()
    }

    fn end_index(&self) -> usize {
        min(
            self.num_candidates,
            self.max_page_size() * (self.current_page() + 1),
        )
    }
}
