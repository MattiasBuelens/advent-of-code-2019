use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn zero() -> Position {
        Position::new(0, 0)
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn compare_by_manhattan_distance(&self, other: &Position) -> Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self: Position, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self: Position, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
