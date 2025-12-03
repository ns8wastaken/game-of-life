use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Coord {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

// -------------------- Arithmetic with another Coord --------------------

// Coord + Coord
impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// Coord - Coord
impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// -Coord
impl Neg for Coord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

// -------------------- Arithmetic with scalars --------------------

// Coord * i64
impl Mul<i64> for Coord {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

// i64 * Coord
impl Mul<Coord> for i64 {
    type Output = Coord;
    fn mul(self, rhs: Coord) -> Self::Output {
        Coord { x: rhs.x * self, y: rhs.y * self }
    }
}

// Coord / i64
impl Div<i64> for Coord {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

// -------------------- Coord <-> tuple arithmetic --------------------

// Coord + (i64, i64)
impl Add<(i64, i64)> for Coord {
    type Output = Coord;
    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Coord { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

// (i64, i64) + Coord
impl Add<Coord> for (i64, i64) {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord { x: self.0 + rhs.x, y: self.1 + rhs.y }
    }
}

// Coord - (i64, i64)
impl Sub<(i64, i64)> for Coord {
    type Output = Coord;
    fn sub(self, rhs: (i64, i64)) -> Self::Output {
        Coord { x: self.x - rhs.0, y: self.y - rhs.1 }
    }
}

// (i64, i64) - Coord
impl Sub<Coord> for (i64, i64) {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord { x: self.0 - rhs.x, y: self.1 - rhs.y }
    }
}

// Coord * (i64, i64)
impl Mul<(i64, i64)> for Coord {
    type Output = Coord;
    fn mul(self, rhs: (i64, i64)) -> Self::Output {
        Coord { x: self.x * rhs.0, y: self.y * rhs.1 }
    }
}

// Coord / (i64, i64)
impl Div<(i64, i64)> for Coord {
    type Output = Coord;
    fn div(self, rhs: (i64, i64)) -> Self::Output {
        Coord { x: self.x / rhs.0, y: self.y / rhs.1 }
    }
}
