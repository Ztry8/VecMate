use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

mod tests;

/// Module for working with two-dimensional vectors
pub mod vec2;

/// Trait that defines that the type is a number
pub trait Number<T>:
    Clone
    + Copy
    + PartialEq
    + Sized
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
    + Neg
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
}

impl<
    T: Clone
        + Copy
        + PartialEq
        + Sized
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Neg
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign,
> Number<T> for T
{
}

/// Trait that specifies that the root of a number can be calculated
pub trait Float<T>: Number<T> {
    fn sqrt(self) -> T;
}

impl Float<f32> for f32 {
    fn sqrt(self) -> f32 {
        f32::sqrt(self)
    }
}

impl Float<f64> for f64 {
    fn sqrt(self) -> f64 {
        f64::sqrt(self)
    }
}
