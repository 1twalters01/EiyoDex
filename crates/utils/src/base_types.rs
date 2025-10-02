use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Angle {
    pub value: f64,
}

impl Angle {
    pub fn new(degrees: f64) -> Self {
        let value = degrees.rem_euclid(360f64);
        Angle { value }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
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

pub struct Normalized {
    pub value: f64,
}

impl Normalized {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self { value })
        } else {
            Err("Value must be between 0 and 1")
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_one_revolution() {
        let angle_1 = Angle::new(360f64);
        assert_eq!(angle_1.get_value(), 0f64);
    }

    #[test]
    fn test_angle_negative_angle() {
        let angle_1 = Angle::new(-10.5);
        let err = (angle_1.get_value() + Angle::new(10.5).get_value())
            .abs()
            .rem_euclid(360f64);
        println!("{}", err);
        assert!(err < 0.0001);
    }

    #[test]
    fn test_angle_over_one_revolution() {
        let angle_1 = Angle::new(400.52);
        let err = (angle_1.get_value() - Angle::new(40.52).get_value()).abs();
        assert!(err < 0.0001);
    }

    #[test]
    fn test_angle_over_two_revolution() {
        let angle_1 = Angle::new(760f64);
        let err = (angle_1.get_value() - Angle::new(40f64).get_value()).abs();
        assert!(err < 0.0001);
    }

    #[test]
    fn test_normalization() {
        let too_large = Normalized::new(2f64);
        assert!(too_large.is_err());

        let too_low = Normalized::new(-0.5);
        assert!(too_low.is_err());

        let normalized = Normalized::new(0.62);
        assert!(normalized.is_ok());
        assert_eq!(normalized.unwrap().get_value(), 0.62);
    }

    #[test]
    fn test_percentages() {
        let large = Percentage::new(100.5f64);
        assert!(!large.is_proportion());

        let low = Percentage::new(-0.5);
        assert!(!low.is_proportion());

        let proportion_1 = Percentage::new(62.8);
        assert!(proportion_1.is_proportion());

        let proportion_2 = Percentage::new(0f64);
        assert!(proportion_2.is_proportion());

        let proportion_3 = Percentage::new(0.1);
        assert!(proportion_3.is_proportion());

        let proportion_4 = Percentage::new(100f64);
        assert!(proportion_4.is_proportion());

        let proportion_5 = Percentage::new(99.99999);
        assert!(proportion_5.is_proportion());

        assert_eq!(proportion_1 + proportion_3, Percentage::new(62.9));
        assert!((proportion_1 - proportion_3).get_value() - 62.7 < 0.000001);
    }
}
