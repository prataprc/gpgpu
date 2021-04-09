use std::ops::{Add, Div, Mul, Neg};

use crate::math::{ACos, Identity, Polynomial, Sqrt};

pub fn origin<T>() -> Point<T>
where
    T: Default,
{
    Point {
        x: T::default(),
        y: T::default(),
    }
}

/// 2-dimentional point.
#[derive(Copy, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Point<T>;

    fn mul(self, scalar: T) -> Self {
        Point {
            x: self.x * scalar.clone(),
            y: self.y * scalar.clone(),
        }
    }
}

impl<T> Neg for Point<T>
where
    T: Neg<Output = T>,
{
    type Output = Point<T>;

    fn neg(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Point<T> {
    #[inline]
    pub fn length(&self) -> T
    where
        T: Copy + Mul<Output = T> + Sqrt<Output = T> + Add<Output = T>,
    {
        let a = (self.x * self.x) + (self.y + self.y);
        a.sqrt()
    }

    #[inline]
    pub fn dot(self, other: Self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        (self.x * other.x) + (self.y * other.y)
    }

    #[inline]
    pub fn normalize(self) -> Self
    where
        T: PartialEq
            + Default
            + Copy
            + Identity
            + Mul<Output = T>
            + Sqrt<Output = T>
            + Add<Output = T>
            + Div<Output = T>,
    {
        let l = self.length();
        if l == T::default() {
            self
        } else {
            self * (T::identity() / l)
        }
    }

    #[inline]
    pub fn translate(self, other: Self) -> Self
    where
        T: Add<Output = T>,
    {
        self + other
    }

    #[inline]
    pub fn scale(self, other: T) -> Self
    where
        T: Copy + Mul<Output = T>,
    {
        self * other
    }

    #[inline]
    pub fn angle(self, other: Self) -> f64
    where
        T: PartialEq
            + Default
            + Copy
            + Identity
            + Mul<Output = T>
            + Sqrt<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + ACos<Output = f64>,
    {
        let a = self.normalize();
        let b = other.normalize();
        a.dot(b).acos()
    }

    // TODO: rotate, distance between points.
}

#[macro_export]
macro_rules! plane {
    ($x:expr, $y:expr) => {
        Plane { xfn: $x, yfn: $y }
    };
}

pub struct Plane<T> {
    xfn: Polynomial<T>,
    yfn: Polynomial<T>,
}

impl<T> Plane<T> {
    pub fn point_for(&self, t: T) -> Point<T>
    where
        T: Clone + Default + Identity + Mul<Output = T> + Add<Output = T>,
    {
        Point {
            x: self.xfn.eval(t.clone()),
            y: self.yfn.eval(t),
        }
    }
}
