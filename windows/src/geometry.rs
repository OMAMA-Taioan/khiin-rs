use num::{Num, ToPrimitive};
use num::cast::NumCast;
use num::traits::AsPrimitive;
use windows::Win32::Foundation::POINT;

#[derive(Copy, Clone, Default)]
pub struct Point<T> where T: Copy + Num + NumCast {
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

pub struct Size<T> where T: Copy + Num + NumCast {
    pub w: T,
    pub h: T,
}

#[derive(Default)]
pub struct Rect<T> where T: Copy + Num + NumCast {
    o: Point<T>, // top left
    w: T,
    h: T,
}

impl<T> Rect<T> where T: Copy + Num + NumCast {
    pub fn new(origin: Point<T>, width: T, height: T) -> Self {
        Rect {
            o: origin,
            w: width,
            h: height,
        }
    }

    pub fn w(&self) -> T {
        self.o.x
    }

    pub fn e(&self) -> T {
        self.o.x + self.w
    }

    pub fn n(&self) -> T {
        self.o.y
    }

    pub fn s(&self) -> T {
        self.o.y + self.h
    }

    pub fn size(&self) -> Size<T> {
        Size {
            w: self.w,
            h: self.h,
        }
    }

    pub fn o(&self) -> Point<T> {
        self.o
    }

    pub fn center(&self) -> Point<T> {
        return Point {
            x: self.o.x + self.w / (T::one() + T::one()),
            y: self.o.y + self.h / (T::one() + T::one()),
        };
    }
}
