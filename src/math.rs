use std::ops::{Add, Mul};

pub trait Identity {
    fn identity() -> Self;
}

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

pub trait ACos {
    type Output;

    fn acos(self) -> Self::Output;
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
