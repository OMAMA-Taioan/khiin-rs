use std::cmp::max;
use std::rc::Rc;
use std::sync::Arc;

use windows::core::Error;
use windows::core::Result;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;
use windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_METRICS;

use khiin_protos::command::Candidate;

use crate::geometry::Point;
use crate::geometry::Rect;
use crate::geometry::Size;
use crate::ui::candidates::CandidateCols;
use crate::ui::RenderFactory;

#[derive(Default)]
pub struct Grid {
    pub cols: usize,
    pub col_widths: Vec<i32>,
    pub rows: usize,
    pub row_height: i32,
    pub row_padding: i32,
}

pub struct GridCell {
    pub row: usize,
    pub col: usize,
}

impl Grid {
    pub fn new(
        num_rows: usize,
        num_cols: usize,
        min_col_width: i32,
        row_padding: i32,
    ) -> Self {
        Self {
            cols: num_cols,
            col_widths: vec![min_col_width; num_cols],
            rows: num_rows,
            row_height: 0,
            row_padding,
        }
    }

    pub fn ensure_col_width(&mut self, col: usize, width: i32) {
        assert!(col < self.cols);
        self.col_widths[col] = max(self.col_widths[col], width);
    }

    pub fn ensure_row_height(&mut self, height: i32) {
        self.row_height = max(self.row_height, height);
    }

    pub fn row_height(&self) -> i32 {
        self.row_height + self.row_padding
    }

    pub fn cell_rect(&self, row: usize, col: usize) -> Rect<i32> {
        assert!(row < self.rows);
        assert!(col < self.cols);
        let left = self.col_widths.iter().take(col).sum::<i32>();
        let top = row as i32 * self.row_height();
        let width = self.col_widths[col];
        let height = self.row_height();
        Rect {
            origin: Point { x: left, y: top },
            width,
            height,
        }
    }

    pub fn grid_size(&self) -> Size<i32> {
        let width = self.col_widths.iter().sum::<i32>();
        // double check
        let height = self.rows as i32 * self.row_height
            + (self.rows as i32 + 1) * self.row_padding;
        Size {
            w: width,
            h: height,
        }
    }

    fn hit_test(&self, pt: Point<i32>) -> Option<GridCell> {
        for col in 0..self.cols {
            for row in 0..self.rows {
                let rect = self.cell_rect(row, col);
                if rect.contains(pt) {
                    return Some(GridCell { row, col });
                }
            }
        }

        None
    }
}

#[derive(Default)]
pub struct CandidateLayout {
    pub items: Vec<Vec<(Rc<Candidate>, IDWriteTextLayout)>>,
    pub grid: Grid,
}

impl CandidateLayout {
    pub fn new(
        factory: Arc<RenderFactory>,
        textformat: IDWriteTextFormat,
        cols: &CandidateCols,
        min_col_width: i32,
        row_padding: i32,
        qs_col_width: i32,
        max_size: Size<i32>,
    ) -> Result<Self> {
        let n_cols = cols.len();
        debug_assert!(n_cols > 0);
        let n_rows = cols.get(0).ok_or(Error::from(E_FAIL))?.len();
        let mut grid = Grid::new(n_rows, n_cols, min_col_width, row_padding);
        let mut items = Vec::new();

        for (i, col) in cols.iter().enumerate() {
            let mut layout_col = Vec::new();

            for candidate in col {
                let layout = factory.create_text_layout(
                    &candidate.value[..],
                    textformat.clone(),
                    max_size.w as f32,
                    max_size.h as f32,
                )?;
                let mut metrics = DWRITE_TEXT_METRICS::default();
                unsafe {
                    layout.GetMetrics(&mut metrics)?;
                }
                grid.ensure_col_width(i, metrics.width as i32 + qs_col_width + row_padding * 2);
                grid.ensure_row_height(metrics.height as i32);
                layout_col.push((candidate.clone(), layout));
            }

            items.push(layout_col);
        }

        Ok(Self { grid, items })
    }

    pub fn get(
        &self,
        row: usize,
        col: usize,
    ) -> Option<&(Rc<Candidate>, IDWriteTextLayout)> {
        self.items.get(col)?.get(row)
    }

    pub fn hit_test(&self, pt: Point<i32>) -> Option<Rc<Candidate>> {
        let GridCell { row, col } = self.grid.hit_test(pt)?;
        Some(self.items.get(col)?.get(row)?.0.clone())
    }
}
