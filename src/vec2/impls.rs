use super::{Vec2, vec2};
use crate::Number;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl<T> Add for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Mul for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        vec2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Div for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn div(self, rhs: Self) -> Self::Output {
        vec2(self.x / rhs.x, self.y / rhs.y)
    }
}

/*

*/

impl<T> Add<T> for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: T) -> Self::Output {
        vec2(self.x + rhs, self.y + rhs)
    }
}

impl<T> Sub<T> for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: T) -> Self::Output {
        vec2(self.x - rhs, self.y - rhs)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Number<T>,
{
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        vec2(self.x / rhs, self.y / rhs)
    }
}

/*

*/

impl<T> AddAssign for Vec2<T>
where
    T: Number<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: Number<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> MulAssign for Vec2<T>
where
    T: Number<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> DivAssign for Vec2<T>
where
    T: Number<T>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

/*

*/

impl<T> AddAssign<T> for Vec2<T>
where
    T: Number<T>,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> SubAssign<T> for Vec2<T>
where
    T: Number<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T> MulAssign<T> for Vec2<T>
where
    T: Number<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> DivAssign<T> for Vec2<T>
where
    T: Number<T>,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Neg for Vec2<T>
where
    T: Number<T> + Neg<Output = T>,
{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        vec2(-self.x, -self.y)
    }
}

impl<T> PartialEq for Vec2<T>
where
    T: Number<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
