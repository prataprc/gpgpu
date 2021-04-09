use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::{ACos, Identity, Polynomial, Sqrt};

// TODO: should differentiate between point, vector and normal ?

pub fn origin<T>() -> Point<T>
where
    T: Default,
{
    Point {
        x: T::default(),
        y: T::default(),
        z: T::default(),
    }
}

/// 3-dimentional point, can be seen a vector in relation to origin.
#[derive(Copy, Clone)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> From<(T, T, T)> for Point<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Point { x, y, z }
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
            z: self.z + other.z,
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
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
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
            z: -self.z,
        }
    }
}

impl<T> Point<T> {
    #[inline]
    pub fn length(&self) -> T
    where
        T: Copy + Mul<Output = T> + Sqrt<Output = T> + Add<Output = T>,
    {
        let a = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
        a.sqrt()
    }

    #[inline]
    pub fn dot(self, other: Self) -> T
    where
        T: Copy + Mul<Output = T> + Add<Output = T>,
    {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline]
    pub fn x(self, other: Self) -> Self
    where
        T: Copy + Mul<Output = T> + Sub<Output = T>,
    {
        Point {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
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
macro_rules! space {
    ($x:expr, $y:expr, $z:expr) => {
        Space {
            xfn: $x,
            yfn: $y,
            zfn: $z,
        }
    };
}

pub struct Matrix3<T> {
    data: [T; 9],
}

impl<T> Identity for Matrix3<T>
where
    T: Copy + Identity + Default,
{
    fn identity() -> Self {
        let (i, d) = (T::identity(), T::default());
        Matrix3 {
            data: [i, d, d, d, i, d, d, d, i],
        }
    }
}

impl<'a, T> Mul<Point<T>> for &'a Matrix3<T>
where
    T: Copy + Default + Identity + Mul<Output = T> + Add<Output = T>,
{
    type Output = Point<T>;

    fn mul(self, p: Point<T>) -> Point<T> {
        Point {
            x: p.x * self.data[0] + p.y * self.data[3] + p.z * self.data[6],
            y: p.x * self.data[1] + p.y * self.data[4] + p.z * self.data[7],
            z: p.x * self.data[2] + p.y * self.data[5] + p.z * self.data[8],
        }
    }
}

impl<T> Matrix3<T> {
    pub fn rotatez(radian: f64) -> Self
    where
        T: Copy + Default + From<f64> + Identity + Neg<Output = T>,
    {
        let (sin, cos): (T, T) = (radian.sin().into(), radian.cos().into());
        let (i, d) = (T::identity(), T::default());
        Matrix3 {
            data: [cos, sin, d, -sin, cos, d, d, d, i],
        }
    }

    pub fn rotatex(radian: f64) -> Self
    where
        T: Copy + Default + From<f64> + Identity + Neg<Output = T>,
    {
        let (sin, cos): (T, T) = (radian.sin().into(), radian.cos().into());
        let (i, d) = (T::identity(), T::default());
        Matrix3 {
            data: [i, d, d, d, cos, sin, d, -sin, cos],
        }
    }

    pub fn rotatey(radian: f64) -> Self
    where
        T: Copy + Default + From<f64> + Identity + Neg<Output = T>,
    {
        let (sin, cos): (T, T) = (radian.sin().into(), radian.cos().into());
        let (i, d) = (T::identity(), T::default());
        Matrix3 {
            data: [cos, d, -sin, d, i, d, sin, d, cos],
        }
    }

    pub fn scale_by(r00: T, r11: T, r22: T) -> Self
    where
        T: Copy + Default,
    {
        let d = T::default();
        Matrix3 {
            data: [r00, d, d, d, r11, d, d, d, r22],
        }
    }

    pub fn flip(mut self, x: bool, y: bool, z: bool) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        if x {
            self.data[0] = -self.data[0]
        }
        if y {
            self.data[4] = -self.data[4]
        }
        if z {
            self.data[8] = -self.data[8]
        }
        self
    }
}

pub struct Space<T> {
    xfn: Polynomial<T>,
    yfn: Polynomial<T>,
    zfn: Polynomial<T>,
}

impl<T> Space<T> {
    pub fn point_for(&self, t: T) -> Point<T>
    where
        T: Copy + Default + Identity + Mul<Output = T> + Add<Output = T>,
    {
        Point {
            x: self.xfn.eval(t),
            y: self.yfn.eval(t),
            z: self.zfn.eval(t),
        }
    }
}
