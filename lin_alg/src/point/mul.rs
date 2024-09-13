use super::Point;
use std::ops::{Mul, MulAssign};

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Point(self.0 * other.0, self.1 * other.1)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Point(self.0 * other, self.1 * other)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point(self * other.0, self * other.1)
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        *self = Point(self.0 * other.0, self.1 * other.1);
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, other: f64) {
        *self = Point(self.0 * other, self.1 * other);
    }
}
