use std::ops::{Add, Sub, Mul, Div};

/// A wrapper around an f64 representing an Angle
#[derive(Debug, PartialEq)]
pub struct Angle {
    pub value: f64,
}

impl Angle {
    /// Constructs a new Angle container type
    pub fn new(degrees: f64) -> Self {
        let value = degrees.rem_euclid(360f64);
        Angle { value }
    }

    /// Get the value of an Angle
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Update the value of an Angle
    pub fn set_value(&mut self, value: f64) {
        self.value = value.rem_euclid(360f64);
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.get_value() + rhs.get_value())
    }
}

impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.get_value() - rhs.get_value())
    }
}

impl<T> Mul<T> for Angle
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into())
    }
}

impl<T> Div<T> for Angle
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into())
    }
}
