use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const DOWN_LEFT: Self = Self { x: -1, y: 1 };
    pub const DOWN_RIGHT: Self = Self { x: 1, y: 1 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn len(self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn direction(&self) -> Option<Self> {
        if self.x == 0 && self.y == 0 {
            return None;
        } else if self.x == 0 {
            return Some(Self::new(0, self.y.signum()));
        } else if self.y == 0 {
            return Some(Self::new(self.x.signum(), 0));
        }
        None
    }

    pub fn decomposition(&self) -> (Self, Self) {
        (Self::new(self.x, 0), Self::new(0, self.y))
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
