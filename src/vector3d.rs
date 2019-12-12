use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3D {
    pub fn new(x: i32, y: i32, z: i32) -> Vector3D {
        Vector3D { x, y, z }
    }

    pub fn zero() -> Vector3D {
        Vector3D::new(0, 0, 0)
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Vector3D::zero()
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self: Vector3D, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self: Vector3D, other: Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Vector3D {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Self) {
        self.x.add_assign(other.x);
        self.y.add_assign(other.y);
        self.z.add_assign(other.z);
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, other: Self) {
        self.x.sub_assign(other.x);
        self.y.sub_assign(other.y);
        self.z.sub_assign(other.z);
    }
}
