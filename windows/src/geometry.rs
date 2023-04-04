use num::cast::NumCast;
use num::traits::AsPrimitive;
use num::{Num, ToPrimitive};
use windows::Win32::Foundation::{POINT, RECT};

#[derive(Copy, Clone, Default)]
pub struct Point<T>
where
    T: Copy + Num + NumCast,
{
    pub x: T,
    pub y: T,
}

impl<T> From<POINT> for Point<T>
where
    T: Copy + Num + NumCast,
{
    fn from(value: POINT) -> Self {
        Point {
            x: NumCast::from(value.x).unwrap(),
            y: NumCast::from(value.y).unwrap(),
        }
    }
}

impl<T> From<&POINT> for Point<T>
where
    T: Copy + Num + NumCast,
{
    fn from(value: &POINT) -> Self {
        Point {
            x: NumCast::from(value.x).unwrap(),
            y: NumCast::from(value.y).unwrap(),
        }
    }
}

pub struct Size<T>
where
    T: Copy + Num + NumCast,
{
    pub w: T,
    pub h: T,
}

#[derive(Default, Clone)]
pub struct Rect<T>
where
    T: Copy + Num + NumCast,
{
    pub origin: Point<T>, // top left
    pub width: T,
    pub height: T,
}

impl<T> Rect<T>
where
    T: Copy + Num + NumCast,
{
    pub fn new(origin: Point<T>, width: T, height: T) -> Self {
        Rect {
            origin,
            width,
            height,
        }
    }

    pub fn left(&self) -> T {
        self.origin.x
    }

    pub fn right(&self) -> T {
        self.origin.x + self.width
    }

    pub fn top(&self) -> T {
        self.origin.y
    }

    pub fn bottom(&self) -> T {
        self.origin.y + self.height
    }

    pub fn size(&self) -> Size<T> {
        Size {
            w: self.width,
            h: self.height,
        }
    }

    pub fn center(&self) -> Point<T> {
        return Point {
            x: self.origin.x + self.width / (T::one() + T::one()),
            y: self.origin.y + self.height / (T::one() + T::one()),
        };
    }
}

impl From<&RECT> for Rect<i32> {
    fn from(value: &RECT) -> Self {
        Rect {
            origin: Point {
                x: value.left,
                y: value.top,
            },
            width: value.right - value.left,
            height: value.bottom - value.top,
        }
    }
}
