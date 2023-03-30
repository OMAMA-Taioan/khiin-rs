use num::Num;

use super::point::Point;

pub struct Size<T: Copy + Num> {
    pub w: T,
    pub h: T,
}

#[derive(Default)]
pub struct Rect<T: Copy + Num> {
    o: Point<T>, // top left
    w: T,
    h: T,
}

impl<T: Copy + Num> Rect<T> {
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
