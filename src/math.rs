use std::ops::{Add, Mul};

pub trait Identity {
    fn identity() -> Self;
}

/// 2-dimentional vector.
pub struct V2<T>(T, T);

impl<T> Add for V2<T>
where
    T: Add<Output = T>,
{
    type Output = V2<T>;

    fn add(self, other: Self) -> Self {
        V2(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> Mul<T> for V2<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = V2<T>;

    fn mul(self, scalar: T) -> Self {
        V2(self.0 * scalar.clone(), self.1 * scalar.clone())
    }
}

/// 3-dimentional vector.
pub struct V3<T>(T, T, T);

impl<T> Add for V3<T>
where
    T: Add<Output = T>,
{
    type Output = V3<T>;

    fn add(self, other: Self) -> Self {
        V3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<T> Mul<T> for V3<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = V3<T>;

    fn mul(self, scalar: T) -> Self {
        V3(
            self.0 * scalar.clone(),
            self.1 * scalar.clone(),
            self.2 * scalar.clone(),
        )
    }
}

#[macro_export]
macro_rules! polynomial {
    ($($name:expr),*) => {
        let coeffs = = vec![$($name)*];
        Polynomial{ coeffs: coeffs.skip(|c| c == 0).collect() }
    };
}

pub struct Polynomial<T> {
    coeffs: Vec<T>,
}

impl<T> Polynomial<T> {
    pub fn degree(&self) -> usize {
        self.coeffs.len()
    }

    pub fn eval(&self, x: T) -> T
    where
        T: Clone + Default + Identity + Mul<Output = T> + Add<Output = T>,
    {
        let mut acc = T::identity();
        let mut sum = T::default();
        for c in self.coeffs.iter() {
            sum = sum + (c.clone() * acc.clone());
            acc = acc * x.clone();
        }
        sum
    }
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
    pub fn to_vector(&self, t: T) -> V2<T>
    where
        T: Clone + Default + Identity + Mul<Output = T> + Add<Output = T>,
    {
        V2(self.xfn.eval(t.clone()), self.yfn.eval(t))
    }
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

pub struct Space<T> {
    xfn: Polynomial<T>,
    yfn: Polynomial<T>,
    zfn: Polynomial<T>,
}

impl<T> Space<T> {
    pub fn to_vector(&self, t: T) -> V3<T>
    where
        T: Clone + Default + Identity + Mul<Output = T> + Add<Output = T>,
    {
        V3(
            self.xfn.eval(t.clone()),
            self.yfn.eval(t.clone()),
            self.zfn.eval(t),
        )
    }
}

//pub struct Term<A, X, R> {
//    pub coeff: A,
//    pub exp: Ratio<R>,
//    pub var: marker::PhantomData<X>,
//}
//
//impl<A, X, R> Fn<()> for Term<A, X, R> {
//    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
//        use std::sync::mpsc::TryRecvError;
//    }
//}
