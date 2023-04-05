static FONT_SIZE: Range = Range {
    min: 16.0,
    max: 32.0,
};
static FONT_SIZE_SM: Range = Range {
    min: 16.0,
    max: 20.0,
};
static PADDING: Range = Range {
    min: 8.0,
    max: 12.0,
};
static PADDING_SM: Range = Range { min: 4.0, max: 6.0 };

static BUBBLE_HEIGHT: Range = Range {
    min: 16.0,
    max: 24.0,
};

static BUBBLE_WIDTH: f32 = 4.0;

#[derive(Default)]
struct Range {
    min: f32,
    max: f32,
}

impl Range {
    fn scale(&self, s: f32) -> f32 {
        let s = if s > 1.0 {
            1.0
        } else if s < 0.0 {
            0.0
        } else {
            s
        };

        s * (self.max - self.min) + self.min
    }
}

pub struct Metrics {
    pub padding: f32,
    pub padding_sm: f32,
    pub marker_w: f32,
    pub marker_h: f32,
    pub font_size: f32,
    pub font_size_sm: f32,
    pub row_height: f32,
    pub min_col_w_single: i32,
    pub min_col_w_multi: i32,
    pub qs_col_w: i32,
}

impl Metrics {
    pub fn new(s: f32) -> Self {
        let font_size = FONT_SIZE.scale(s);
        let padding = PADDING.scale(s);
        let padding_sm = PADDING_SM.scale(s);
        let font_size_sm = FONT_SIZE_SM.scale(s);
        let marker_w = 4.0;
        let marker_h = BUBBLE_WIDTH;
        let row_height = font_size + padding;

        Self {
            padding,
            padding_sm,
            marker_w,
            marker_h,
            font_size,
            font_size_sm,
            row_height,
            min_col_w_single: 80,
            min_col_w_multi: 160,
            qs_col_w: 44,
        }
    }
}
