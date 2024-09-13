use super::Point;
use std::{
    f64::INFINITY,
    ops::{Div, DivAssign},
};

impl Div for Point {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Point(div(self.0, other.0), div(self.1, other.1))
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Point(div(self.0, other), div(self.1, other))
    }
}

impl Div<Point> for f64 {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point(div(self, other.0), div(self, other.1))
    }
}

impl DivAssign for Point {
    fn div_assign(&mut self, other: Self) {
        *self = Point(div(self.0, other.0), div(self.1, other.1));
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, other: f64) {
        *self = Point(div(self.0, other), div(self.1, other));
    }
}

fn div(num: f64, den: f64) -> f64 {
    if den == 0.0 {
        return INFINITY;
    }
    num / den
}
