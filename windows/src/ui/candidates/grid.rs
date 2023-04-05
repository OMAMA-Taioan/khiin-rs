use std::cmp::max;

use crate::geometry::{Point, Rect, Size};

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
    pub fn new(num_rows: usize, num_cols: usize, min_col_width: i32) -> Self {
        Self {
            cols: num_cols,
            col_widths: vec![min_col_width; num_cols],
            rows: num_rows,
            row_height: 0,
            row_padding: 0,
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
        let height = self.rows as i32 * self.row_height
            + (self.rows as i32 + 1) * self.row_padding;
        Size {
            w: width,
            h: height,
        }
    }

    fn hit_test(&self, pt: Point<i32>) -> Option<GridCell> {
        let size = self.grid_size();

        if 0 <= pt.x && pt.x <= size.w && 0 <= pt.y && pt.y <= size.h {
            let row =
                pt.y.div_euclid(self.row_height + self.row_padding) as usize;

            let mut sum = 0;
            for (col, w) in self.col_widths.iter().enumerate() {
                sum += w;
                if pt.x <= sum {
                    return Some(GridCell { row, col });
                }
            }
        }

        None
    }
}

pub struct GridContainer<T>
where
    T: Default + Clone,
{
    pub items: Vec<Vec<T>>,
    pub grid: Grid,
}

impl<T> GridContainer<T>
where
    T: Default + Clone,
{
    pub fn new(num_rows: usize, num_cols: usize, min_col_width: i32) -> Self {
        let grid = Grid::new(num_rows, num_cols, min_col_width);
        let items = vec![vec![T::default(); num_rows]; num_cols];
        Self { items, grid }
    }

    pub fn add(&mut self, row: usize, col: usize, item: T) {
        self.items[col][row] = item;
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.items[col][row].clone()
    }

    pub fn hit_test(&self, pt: Point<i32>) -> Option<T> {
        let GridCell { row, col } = self.grid.hit_test(pt)?;
        Some(self.items[col][row].clone())
    }
}
