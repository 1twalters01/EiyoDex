use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percentage {
    pub value: f64,
}

impl Percentage {
    pub fn new(value: f64) -> Self {
        Percentage { value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn is_proportion(&self) -> bool {
        self.value <= 100f64 && self.value >= 0f64
    }

    pub fn as_fraction(&self) -> f64 {
        self.value as f64 / 100.0
    }
}

impl Add for Percentage {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.get_value() + rhs.get_value())
    }
}

impl Sub for Percentage {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.get_value() - rhs.get_value())
    }
}

impl<T> Mul<T> for Percentage
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into())
    }
}

impl<T> Div<T> for Percentage
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into())
    }
}
