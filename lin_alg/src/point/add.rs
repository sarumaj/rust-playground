use super::Point;
use std::ops::{Add, AddAssign};

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Add<f64> for Point {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Point(self.0 + other, self.1 + other)
    }
}

impl Add<Point> for f64 {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self + other.0, self + other.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Point(self.0 + other.0, self.1 + other.1);
    }
}

impl AddAssign<f64> for Point {
    fn add_assign(&mut self, other: f64) {
        *self = Point(self.0 + other, self.1 + other);
    }
}

