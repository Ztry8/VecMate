use crate::{Float, Number};

/// Module with the most common vectors represented as constants
pub mod consts;

/// Module that implements traits
mod impls;

/// This structure representing a two-dimensional vector. Type-agnostic
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    /// X coordinate
    pub x: T,
    /// Y coordinate
    pub y: T,
}

/// Function to create a new vector, the vector type depends on the argument type (the arguments must be of the same type)
pub fn vec2<T>(x: T, y: T) -> Vec2<T>
where
    T: Number<T>,
{
    Vec2 { x, y }
}

impl<S> Vec2<S>
where
    S: Float<S>,
{
    /// Returns vector's length
    pub fn length(&self) -> S {
        (self.x * self.x + self.y + self.y).sqrt()
    }

    /// Normalize the current vector
    pub fn normalize(&self) -> Self {
        let len = self.length();
        vec2(self.x / len, self.y / len)
    }

    /// Returns distance between two vectors
    pub fn distance(&self, rhs: &Self) -> S {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        (x * x + y * y).sqrt()
    }
}

impl<T> Vec2<T>
where
    T: Number<T>,
{
    /// Returns dot product of two vectors
    pub fn dot(&self, rhs: &Self) -> T {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    /// Returns cross product of two vectors
    pub fn cross(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    // Returns manhattan distance between two vectors
    // pub fn manhattan_distance(&self, rhs: &Self) -> T {
    //     (self.x - rhs.x)
    // }
}
