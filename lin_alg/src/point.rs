mod add;
mod div;
mod mul;
mod sub;

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point(x, y)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn set_x(&mut self, x: f64) -> &Self {
        self.0 = x;
        self
    }

    pub fn set_y(&mut self, y: f64) -> &Self {
        self.1 = y;
        self
    }

    pub fn abs(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }

    pub fn angle(&self) -> f64 {
        self.1.atan2(self.0)
    }

    pub fn distance(&self, other: Self) -> f64 {
        (self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)
    }

    pub fn normalize(&mut self) {
        let abs = self.abs();
        self.0 /= abs;
        self.1 /= abs;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
