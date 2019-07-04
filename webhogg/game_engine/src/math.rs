use std::ops;

pub const EPSILON: f32 = 1e-8;

/// A 2-dimensional euclidean vector.
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// The x coordinate.
    pub x: f32,
    /// The y coordinate.
    pub y: f32,
}

impl ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, scale: f32) -> Self {
        Self {
            x: self.x / scale,
            y: self.y / scale,
        }
    }
}

impl ops::Div for Vec2 {
    type Output = Self;
    fn div(self, other: Vec2) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl std::cmp::PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x <= other.x && self.y <= other.y {
            Some(std::cmp::Ordering::Less)
        } else if self.x > other.x && self.y > other.y {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl std::cmp::PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.x - other.x) < EPSILON && f32::abs(self.y - other.y) < EPSILON
    }
}

impl std::cmp::Eq for Vec2 {}

impl Vec2 {
    /// Returns the euclidean norm of the vector.
    pub fn norm(&self) -> f32 {
        f32::hypot(self.x, self.y)
    }

    /// Returns the square of the euclidean norm of the vector.
    pub fn norm2(&self) -> f32 {
        self.dot(*self)
    }

    /// Returns the dot product.
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns a normalized version of the vector, that is, a vector that points in the same direction, but has length 1.
    pub fn normalized(&self) -> Vec2 {
        *self / self.norm()
    }
}

/// An axis-aligned box.
#[derive(Clone, Copy, Debug)]
pub struct AABox {
    pub pos: Vec2,
    /// the size may not be smaller than zero
    pub size: Vec2,
}

impl ops::Add<Vec2> for AABox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            size: self.size,
        }
    }
}

impl ops::AddAssign<Vec2> for AABox {
    fn add_assign(&mut self, other: Vec2) {
        self.pos += other
    }
}

impl ops::Sub<Vec2> for AABox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos - other,
            size: self.size,
        }
    }
}

impl ops::SubAssign<Vec2> for AABox {
    fn sub_assign(&mut self, other: Vec2) {
        self.pos -= other
    }
}

impl std::cmp::PartialEq for AABox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.size == other.size
    }
}

impl std::cmp::Eq for AABox {}

/// A rotated box.
#[derive(Clone, Copy, Debug)]
pub struct RBox {
    /// The origin.
    pub pos: Vec2,
    /// Vector1
    pub v1: Vec2,
    /// Vector2: this Vector has to always be orthogonal to v1
    pub v2: Vec2,
}

impl RBox {
    /// Creates a new rotated box from a position, an orientation and a width.
    pub fn new(pos: Vec2, orientation: Vec2, width: f32) -> Self {
        let scale = width / orientation.norm();
        let orth = Vec2 {
            x: orientation.x,
            y: -orientation.y,
        } / scale;
        Self {
            pos: pos,
            v1: orientation,
            v2: orth,
        }
    }
}

impl ops::Add<Vec2> for RBox {
    type Output = Self;
    fn add(self, other: Vec2) -> Self {
        Self {
            pos: self.pos + other,
            v1: self.v1,
            v2: self.v2,
        }
    }
}

impl ops::AddAssign<Vec2> for RBox {
    fn add_assign(&mut self, other: Vec2) {
        self.pos += other
    }
}

impl ops::Sub<Vec2> for RBox {
    type Output = Self;
    fn sub(self, other: Vec2) -> Self {
        Self {
            pos: self.pos - other,
            v1: self.v1,
            v2: self.v2,
        }
    }
}

impl ops::SubAssign<Vec2> for RBox {
    fn sub_assign(&mut self, other: Vec2) {
        self.pos -= other
    }
}

impl std::cmp::PartialEq for RBox {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.v1 == other.v1 && self.v2 == other.v2
    }
}

impl std::cmp::Eq for RBox {}
