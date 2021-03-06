use core::iter::{once, Chain, Once};
use core::ops;

use crate::math::EPSILON;

/// A 3-dimensional euclidean vector with `f32` elements.
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    // The x coordinate.
    x: f32,
    // The y coordinate.
    y: f32,
    // The z coordinate.
    z: f32,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self::new(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scale: f32) {
        *self = *self * scale
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scale: f32) -> Self::Output {
        Self::new(self.x / scale, self.y / scale, self.z / scale)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scale: f32) {
        *self = *self / scale
    }
}

impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < EPSILON
            && f32::abs(self.y - other.y) < EPSILON
            && f32::abs(self.z - other.z) < EPSILON
    }
}

impl Eq for Vec3 {}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(vec: Vec3) -> Self {
        (vec.x(), vec.y(), vec.z())
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(vec: Vec3) -> Self {
        [vec.x(), vec.y(), vec.z()]
    }
}

pub type IntoIter = Chain<Once<f32>, Chain<Once<f32>, Once<f32>>>;

impl IntoIterator for Vec3 {
    type Item = f32;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        once(self.x).chain(once(self.y).chain(once(self.z)))
    }
}

impl Vec3 {
    /// Creates a new `Vec3` from x and y coordinates.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the zero vector.
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Returns the x coordinate.
    pub const fn x(self) -> f32 {
        self.x
    }

    /// Returns the y coordinate.
    pub const fn y(self) -> f32 {
        self.y
    }

    /// Returns the y coordinate.
    pub const fn z(self) -> f32 {
        self.z
    }

    /// Returns the dot product.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the square of the euclidean norm of the vector.
    pub fn norm2(self) -> f32 {
        self.dot(self)
    }

    /// Returns the euclidean norm of the vector.
    pub fn norm(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Returns a normalized version of the vector, that is, a vector that points in the same direction, but has norm 1.
    pub fn normalized(self) -> Self {
        self / self.norm()
    }

    /// Returns this `Vec3` as a `Vec2`, disregarding the z component.
    pub fn into_vec2(self) -> super::Vec2 {
        super::Vec2::new(self.x(), self.y())
    }
}
