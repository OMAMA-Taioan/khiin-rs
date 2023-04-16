use std::sync::Arc;

use windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F;
use windows::Win32::Graphics::Direct2D::ID2D1DCRenderTarget;
use windows::Win32::Graphics::Direct2D::ID2D1SolidColorBrush;
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT;
use windows::Win32::Graphics::Direct2D::D2D1_DRAW_TEXT_OPTIONS_NONE;
use windows::Win32::Graphics::DirectWrite::IDWriteTextFormat;
use windows::Win32::Graphics::DirectWrite::IDWriteTextLayout;

use crate::geometry::Point;
use crate::geometry::Rect;
use crate::ui::colors::ColorScheme_F;
use crate::ui::vcenter_textlayout;
use crate::ui::RenderFactory;

use super::layout::CandidateLayout;
use super::CandidatePage;
use super::Metrics;

pub(super) struct CandidateRenderer<'a> {
    pub factory: &'a Arc<RenderFactory>,
    pub target: &'a ID2D1DCRenderTarget,
    pub textformat_sm: &'a IDWriteTextFormat,
    pub brush: &'a ID2D1SolidColorBrush,
    pub colors: &'a ColorScheme_F,
    pub page_data: &'a CandidatePage,
    pub metrics: &'a Metrics,
    pub cand_layout: &'a CandidateLayout,
    pub mouse_focused_id: u32,
}

impl<'a> CandidateRenderer<'a> {
    unsafe fn draw_focused_bg(&self, rect: &Rect<f32>) {
        self.brush.SetColor(&self.colors.background_selected);
        let rr = rect.to_d2d1_rounded(self.metrics.padding_sm);
        self.target.FillRoundedRectangle(&rr, self.brush);
    }

    unsafe fn draw_focused_bubble(&self, origin: &Point<f32>) {
        let o = Point::<f32>::new(
            origin.x + self.metrics.marker_w,
            origin.y + (self.metrics.row_height - self.metrics.marker_h) / 2.0,
        );
        let rect =
            Rect::<f32>::new(o, self.metrics.marker_w, self.metrics.marker_h);
        let rr = rect.to_d2d1_rounded(2.0);

        self.brush.SetColor(&self.colors.accent);
        self.target.FillRoundedRectangle(&rr, self.brush);
    }

    unsafe fn draw_quick_select(&self, label: String, origin: &Point<f32>) {
        let layout = self
            .factory
            .create_text_layout(
                label.as_str(),
                self.textformat_sm.clone(),
                self.metrics.qs_col_w as f32,
                self.metrics.row_height,
            )
            .unwrap();
        let x = origin.x + self.metrics.marker_w * 2.0 + self.metrics.padding;
        let y = origin.y + vcenter_textlayout(&layout, self.metrics.row_height);
        self.target.DrawTextLayout(
            D2D_POINT_2F { x, y },
            &layout,
            self.brush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
        );
    }

    unsafe fn draw_candidate(
        &self,
        layout: &IDWriteTextLayout,
        origin: &Point<f32>,
    ) {
        let x = origin.x + self.metrics.qs_col_w as f32;
        let y = origin.y + vcenter_textlayout(layout, self.metrics.row_height);
        self.brush.SetColor(&self.colors.text);
        self.target.DrawTextLayout(
            D2D_POINT_2F { x, y },
            layout,
            self.brush,
            D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT,
        );
    }

    pub unsafe fn draw(&self) {
        self.target.Clear(Some(&self.colors.background));
        let grid = &self.cand_layout.grid;
        let mut qs_label = 0;
        for (col_idx, col) in self.cand_layout.items.iter().enumerate() {
            for (row_idx, row) in col.iter().enumerate() {
                qs_label += 1;
                let cand = row.0.clone();
                let text_layout = row.1.clone();

                let rect = grid.cell_rect(row_idx, col_idx).to_float();

                let is_focused = cand.id == self.page_data.focused_id
                    || cand.id == self.mouse_focused_id as i32;
                let has_bubble = cand.id == self.page_data.focused_id;

                if is_focused {
                    self.draw_focused_bg(&rect);
                    if has_bubble {
                        self.draw_focused_bubble(&rect.origin);
                    }
                }

                if col_idx == self.page_data.focused_col {
                    self.brush.SetColor(if self.page_data.quickselect_active {
                        &self.colors.text
                    } else {
                        &self.colors.text_disabled
                    });
                    self.draw_quick_select(qs_label.to_string(), &rect.origin);
                }

                self.draw_candidate(&text_layout, &rect.origin);
            }
        }
    }
}
