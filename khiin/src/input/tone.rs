#[repr(i32)]
#[derive(Default, PartialEq, Eq, Hash)]
pub enum Tone {
    #[default]
    None = 0,
    T1 = 1,
    T2 = 2,
    T3 = 3,
    T4 = 4,
    T5 = 5,
    T6 = 6,
    T7 = 7,
    T8 = 8,
    T9 = 9,
}

impl From<i32> for Tone {
    fn from(value: i32) -> Self {
        match value {
            x if x == Tone::T1 as i32 => Tone::T1,
            x if x == Tone::T2 as i32 => Tone::T2,
            x if x == Tone::T3 as i32 => Tone::T3,
            x if x == Tone::T4 as i32 => Tone::T4,
            x if x == Tone::T5 as i32 => Tone::T5,
            x if x == Tone::T6 as i32 => Tone::T6,
            x if x == Tone::T7 as i32 => Tone::T7,
            x if x == Tone::T8 as i32 => Tone::T8,
            x if x == Tone::T9 as i32 => Tone::T9,
            _ => Tone::None,
        }
    }
}
