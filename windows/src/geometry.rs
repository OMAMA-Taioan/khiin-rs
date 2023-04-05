use num::cast::NumCast;
use num::traits::AsPrimitive;
use num::{Num, ToPrimitive};
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Direct2D::Common::D2D_POINT_2F;
use windows::Win32::Graphics::Direct2D::D2D1_ELLIPSE;

#[derive(Copy, Clone, Default)]
pub struct Point<T>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<i32> {
    pub fn to_float(&self) -> Point<f32> {
        Point::new(self.x as f32, self.y as f32)
    }

    pub fn d2d1_point(&self) -> D2D_POINT_2F {
        D2D_POINT_2F {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    pub fn d2d1_circle(&self, radius: f32) -> D2D1_ELLIPSE {
        D2D1_ELLIPSE {
            point: D2D_POINT_2F {
                x: self.x as f32,
                y: self.y as f32,
            },
            radiusX: radius,
            radiusY: radius,
        }
    }
}

impl<T> From<POINT> for Point<T>
where
    T: Copy + Num + NumCast + PartialOrd,
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
    T: Copy + Num + NumCast + PartialOrd,
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
    T: Copy + Num + NumCast + PartialOrd,
{
    pub w: T,
    pub h: T,
}

#[derive(Default, Clone)]
pub struct Rect<T>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    pub origin: Point<T>, // top left
    pub width: T,
    pub height: T,
}

impl Rect<i32> {
    pub fn to_float(&self) -> Rect<f32> {
        Rect::new(self.origin.to_float(), self.width as f32, self.height as f32)
    }
}

impl<T> Rect<T>
where
    T: Copy + Num + NumCast + PartialOrd,
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

    pub fn contains(&self, pt: Point<T>) -> bool {
        pt.x > self.origin.x
            && pt.y > self.origin.y
            && pt.x < self.origin.x + self.width
            && pt.y < self.origin.y + self.height
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
