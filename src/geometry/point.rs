use num::Num;

#[derive(Copy, Clone, Default)]
pub struct Point<T: Copy + Num> {
    pub x: T,
    pub y: T,
}
