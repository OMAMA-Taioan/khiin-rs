use std::cmp::min;
use std::sync::Arc;
use std::{borrow::Borrow, cell::RefCell};

use num::Num;
use windows::core::Result;

use khiin_protos::command::Candidate;
use khiin_protos::command::Command;

use super::candidate_window::DisplayMode;

type CandidateGrid = Vec<Vec<Candidate>>;

#[derive(Default)]
pub struct Pager {
    pub command: Arc<Command>,
    pub num_candidates: usize,
    pub display_mode: RefCell<DisplayMode>,
    pub focused_id: RefCell<i32>,
    pub focused_index: RefCell<usize>,
    pub focused_col: RefCell<usize>,
    pub grid: RefCell<CandidateGrid>,
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
            focused_col: RefCell::new(0),
            grid: RefCell::new(CandidateGrid::default()),
        }
    }

    pub fn get_page(&self) -> CandidateGrid {
        let mut grid: Vec<Vec<Candidate>> = Vec::new();

        if self.num_candidates == 0 {
            return grid;
        }

        let candidates = self.candidates();
        let mut start = self.start_index();
        let end = self.end_index();
        let mut col: Vec<Candidate> = Vec::new();
        for (i, candidate) in candidates.iter().skip(start).take(end - start + 1).enumerate() {
            if i == start + self.max_col_size() {
                grid.push(col);
                col = Vec::new();
                start = i
            }

            col.push(candidate.clone())
        }
        grid.push(col);
        grid
    }

    pub fn set_focus(&self, index: i32) -> Result<()> {
        Ok(())
    }
}

// internal helpers
impl Pager {
    fn candidates(&self) -> &Vec<Candidate> {
        &self.command.response.candidate_list.candidates
    }

    fn focused_index(&self) -> usize {
        self.focused_index.borrow().clone()
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
            DisplayMode::Grid => 4,
            _ => 1,
        }
    }

    fn max_col_size(&self) -> usize {
        match &*self.display_mode.borrow() {
            DisplayMode::ShortColumn => 5,
            DisplayMode::LongColumn => 10,
            DisplayMode::Grid => 10,
        }
    }

    fn current_page(&self) -> usize {
        self.focused_index().div_euclid(self.max_page_size())
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

    fn max_page_size(&self) -> usize {
        self.max_cols_per_page() * self.max_col_size()
    }
}
