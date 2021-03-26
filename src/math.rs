use std::ops::{Add, Mul};

/// 2-dimentional vector.
pub struct V2<T>(T, T);

/// 3-dimentional vector.
pub struct V3<T>(T, T, T);

macro_rules! ops {
    ($($name:ident),*) => {
        $(
            impl<T> Add for $name<T>
            where
                T: Add<Output = T>,
            {
                type Output = $name<T>;

                fn add(self, other: Self) -> Self {
                    $name(self.0 + other.0, self.1 + other.1)
                }
            }

            impl<T> Mul<T> for $name<T>
            where
                T: Mul<Output=T> + Clone
            {
                type Output = $name<T>;

                fn mul(self, scalar: T) -> Self {
                    $name(self.0 * scalar.clone(), self.1 * scalar.clone())
                }
            }
        )*
    };
}

ops! {V2}

pub struct Polynomial<T>(Vec<T>);

#[macro_export]
macro_rules! polynomial {
    ($($name:expr),*) => {
        let coeffs = = vec![$($name)*];
        Polynomial(coeffs.skip(|c| c == 0).collect())
    };
}

impl<T> Polynomial<T> {
    pub fn degree(&self) -> usize {
        self.0.len()
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
