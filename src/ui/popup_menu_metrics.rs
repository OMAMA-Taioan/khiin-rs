struct PopupMenuMetrics {
    y_adjust: f32,
    bullet_col: f32,
    icon_col: f32,
    vpad: f32,
    hpad: f32,
    sep_thickness: f32,
    font_size: f32,
    row_height: f32,
    icon_size: f32,
}

impl PopupMenuMetrics {
    fn new(scale: f32) -> Self {
        PopupMenuMetrics {
            y_adjust: 50.0 * scale,
            bullet_col: 24.0 * scale,
            icon_col: 32.0 * scale,
            vpad: 8.0 * scale,
            hpad: 16.0 * scale,
            sep_thickness: 1.0 * scale,
            font_size: 18.0 * scale,
            row_height: 32.0 * scale,
            icon_size: 16.0 * scale,
        }
    }
}