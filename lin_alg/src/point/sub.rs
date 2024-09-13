use super::Point;
use std::ops::{Sub, SubAssign};

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl Sub<f64> for Point {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Point(self.0 - other, self.1 - other)
    }
}

impl Sub<Point> for f64 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self - other.0, self - other.1)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Point(self.0 - other.0, self.1 - other.1);
    }
}

impl SubAssign<f64> for Point {
    fn sub_assign(&mut self, other: f64) {
        *self = Point(self.0 - other, self.1 - other);
    }
}
