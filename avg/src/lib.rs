use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// Define the Avg struct with generic type N
pub struct Avg<N>
where
    N: Add<Output = N>
        + Div<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + AddAssign
        + DivAssign
        + MulAssign
        + SubAssign
        + Into<f64>
        + Copy,
{
    list: Vec<N>,
    value: f64,
}

// Implement methods for the Avg struct
impl<N> Avg<N>
where
    N: Add<Output = N>
        + Div<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + AddAssign
        + DivAssign
        + MulAssign
        + SubAssign
        + Into<f64>
        + Copy,
{
    // Constructor for Avg
    pub fn new() -> Avg<N> {
        Avg {
            list: Vec::new(),
            value: 0.0,
        }
    }

    pub fn add(&mut self, value: N) {
        self.list.push(value);
        self.update();
    }

    pub fn get(&self) -> f64 {
        self.value
    }

    pub fn remove(&mut self) {
        let result = self.list.pop();
        if result.is_some() {
            self.update();
        }
    }

    fn update(&mut self) {
        if self.list.is_empty() {
            self.value = 0.0;
        } else {
            let sum: f64 = self.list.iter().map(|&x| x.into()).sum();
            self.value = sum / self.list.len() as f64;
        }
    }
}
