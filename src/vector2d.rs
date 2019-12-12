use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Vector2D {
        Vector2D { x, y }
    }

    pub fn zero() -> Vector2D {
        Vector2D::new(0, 0)
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Default for Vector2D {
    fn default() -> Self {
        Vector2D::zero()
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self: Vector2D, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self: Vector2D, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Vector2D {
        Vector2D::new(-self.x, -self.y)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        self.x.sub_assign(other.x);
        self.y.sub_assign(other.y);
    }
}
