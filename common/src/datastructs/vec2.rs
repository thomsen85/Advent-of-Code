use std::ops::{Add, Div, Mul, RangeBounds, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

// x is row, y is col
impl Vec2 {
    pub const CART_UP: Self = Self { x: 0, y: 1 };
    pub const CART_UP_RIGHT: Self = Self { x: 1, y: 1 };
    pub const CART_RIGHT: Self = Self { x: 1, y: 0 };
    pub const CART_DOWN_RIGHT: Self = Self { x: 1, y: -1 };
    pub const CART_DOWN: Self = Self { x: 0, y: -1 };
    pub const CART_DOWN_LEFT: Self = Self { x: -1, y: -1 };
    pub const CART_LEFT: Self = Self { x: -1, y: 0 };
    pub const CART_UP_LEFT: Self = Self { x: -1, y: 1 };

    pub const ARR_UP: Self = Self { x: -1, y: 0 };
    pub const ARR_UP_RIGHT: Self = Self { x: -1, y: 1 };
    pub const ARR_RIGHT: Self = Self { x: 0, y: 1 };
    pub const ARR_DOWN_RIGHT: Self = Self { x: 1, y: 1 };
    pub const ARR_DOWN: Self = Self { x: 1, y: 0 };
    pub const ARR_DOWN_LEFT: Self = Self { x: 1, y: -1 };
    pub const ARR_LEFT: Self = Self { x: 0, y: -1 };
    pub const ARR_UP_LEFT: Self = Self { x: -1, y: -1 };

    pub const EIGHT_CONNECTNESS: [Self; 8] = [
        Self::CART_UP,
        Self::CART_UP_RIGHT,
        Self::CART_RIGHT,
        Self::CART_DOWN_RIGHT,
        Self::CART_DOWN,
        Self::CART_DOWN_LEFT,
        Self::CART_LEFT,
        Self::CART_UP_LEFT,
    ];
    pub const FOUR_CONNECTNESS: [Self; 4] = [
        Self::CART_UP,
        Self::CART_RIGHT,
        Self::CART_DOWN,
        Self::CART_LEFT,
    ];

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Shortcut for not casting to i32 and calling new
    pub fn from_row_col(row: usize, col: usize) -> Self {
        Self {
            x: row as i32,
            y: col as i32,
        }
    }

    pub fn neighbours_8_ranged<R1, R2>(&self, x_range: R1, y_range: R2) -> Vec<Self>
    where
        R1: RangeBounds<i32>,
        R2: RangeBounds<i32>,
    {
        Self::EIGHT_CONNECTNESS
            .into_iter()
            .map(|diff| *self + diff)
            .filter(|neighbour| x_range.contains(&neighbour.x) && y_range.contains(&neighbour.y))
            .collect()
    }

    pub fn neighbours_4_ranged<T, R1, R2>(&self, x_range: R1, y_range: R2) -> Vec<Self>
    where
        R1: RangeBounds<i32>,
        R2: RangeBounds<i32>,
    {
        Self::FOUR_CONNECTNESS
            .into_iter()
            .map(|diff| *self + diff)
            .filter(|neighbour| x_range.contains(&neighbour.x) && y_range.contains(&neighbour.y))
            .collect()
    }

    pub fn magnitude_f32(self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }

    pub fn decomposition(&self) -> (Self, Self) {
        (Self::new(self.x, 0), Self::new(0, self.y))
    }

    /// Mini function for working with arrays, this is just the x val casted to usize
    pub fn row(&self) -> usize {
        self.x as usize
    }
    /// Mini function for working with arrays, this is just the y val casted to usize
    pub fn col(&self) -> usize {
        self.y as usize
    }

    pub fn arr_rot_90_clockwise(&self) -> Self {
        Vec2::new(self.y, -self.x)
    }

    pub fn arr_rot_90_counter_clockwise(&self) -> Self {
        Vec2::new(-self.y, self.x)
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
