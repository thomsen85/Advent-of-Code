use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const DOWN_LEFT: Self = Self { x: -1, y: 1 };
    pub const DOWN_RIGHT: Self = Self { x: 1, y: 1 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, c: i32) -> Self {
        Self {
            x: self.x * c,
            y: self.y * c,
        }
    }
}

impl Div<i32> for Vec2 {
    type Output = Self;

    fn div(self, c: i32) -> Self {
        Self {
            x: self.x / c,
            y: self.y / c,
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}