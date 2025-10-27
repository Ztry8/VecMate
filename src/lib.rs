//! [![GitHub last commit](https://img.shields.io/github/last-commit/ztry8/vecmate)](https://github.com/ztry8/vecmate/)
//! [![crates.io](https://img.shields.io/crates/v/vecmate)](https://crates.io/crates/vecmate)
//! [![docs.rs](https://img.shields.io/docsrs/vecmate)](https://docs.rs/vecmate)
//! [![License](https://img.shields.io/github/license/ztry8/vecmate)](https://github.com/ztry8/vecmate/blob/main/LICENSE)
//! ## Lightweight, zero-dependency, type-agnostic library for vector math.   
//! ```rust
//! let mut position = consts::f32::ZERO;
//! let target = vec2(10.0, 5.0);
//! let speed = 2.0;
//!
//! let direction = (target - position).normalize();
//! position += direction * speed;
//!
//! println!("Moving towards {target}");
//! println!("New position: {position}");
//! ```

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
