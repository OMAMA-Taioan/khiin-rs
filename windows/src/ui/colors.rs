use windows::Win32::Graphics::Direct2D::Common::D2D1_COLOR_F;

#[derive(Debug)]
pub enum SystemTheme {
    Light,
    Dark,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub const fn color(color: u32) -> Color {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    Color { r, g, b, a: 0xFF }
}

pub fn color_f(color: &Color) -> D2D1_COLOR_F {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;
    let a = color.a as f32 / 255.0;
    D2D1_COLOR_F { r, g, b, a }
}

pub trait AsD2D1_F {
    fn f(&self) -> D2D1_COLOR_F;
}

impl AsD2D1_F for Color {
    fn f(&self) -> D2D1_COLOR_F {
        color_f(self)
    }
}

pub struct ColorScheme {
    pub text: Color,
    pub text_disabled: Color,
    pub text_extended: Color,
    pub text_hint: Color,
    pub background: Color,
    pub background_selected: Color,
    pub accent: Color,
}

#[derive(Clone)]
pub struct ColorScheme_F {
    pub text: D2D1_COLOR_F,
    pub text_disabled: D2D1_COLOR_F,
    pub text_extended: D2D1_COLOR_F,
    pub text_hint: D2D1_COLOR_F,
    pub background: D2D1_COLOR_F,
    pub background_selected: D2D1_COLOR_F,
    pub accent: D2D1_COLOR_F,
}

impl Into<ColorScheme_F> for ColorScheme {
    fn into(self) -> ColorScheme_F {
        ColorScheme_F {
            text: color_f(&self.text),
            text_disabled: color_f(&self.text_disabled),
            text_extended: color_f(&self.text_extended),
            text_hint: color_f(&self.text_hint),
            background: color_f(&self.background),
            background_selected: color_f(&self.background_selected),
            accent: color_f(&self.accent),
        }
    }
}

pub const COLOR_BLACK: Color = color(0);
pub const COLOR_GRAY: Color = color(0x808080);
pub const COLOR_DARK_GRAY: Color = color(0xA9A9A9);
pub const COLOR_FOREST_GREEN: Color = color(0x228B22);
pub const COLOR_CORNFLOWER_BLUE: Color = color(0x6495ED);
pub const COLOR_GOLDRENROD_YELLOW: Color = color(0xFAFAD2);
pub const COLOR_LIGHT_GRAY: Color = color(0xD3D3D3);
pub const COLOR_WHITE_SMOKE: Color = color(0xF5F5F5);
pub const COLOR_SKY_BLUE: Color = color(0x87CEFA);
pub const COLOR_NEAR_WHITE: Color = color(0xFAFAFA);

pub const COLOR_SCHEME_LIGHT: ColorScheme = ColorScheme {
    text: COLOR_BLACK,
    text_disabled: COLOR_GRAY,
    text_extended: COLOR_DARK_GRAY,
    text_hint: COLOR_FOREST_GREEN,
    background: COLOR_NEAR_WHITE,
    background_selected: color(0xEBEBEB),
    accent: COLOR_CORNFLOWER_BLUE,
};

pub const COLOR_SCHEME_DARK: ColorScheme = ColorScheme {
    text: COLOR_NEAR_WHITE,
    text_disabled: COLOR_LIGHT_GRAY,
    text_extended: COLOR_WHITE_SMOKE,
    text_hint: COLOR_GOLDRENROD_YELLOW,
    background: color(0x2B2B2B),
    background_selected: color(0x1E1E1E),
    accent: COLOR_SKY_BLUE,
};
