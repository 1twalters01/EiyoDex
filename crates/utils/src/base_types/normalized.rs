use std::ops::{Add, Sub};

/// A wrapper around an f64 representing a Normalized Value
#[derive(Debug, PartialEq)]
pub struct Normalized {
    pub value: f64,
}

impl Normalized {
    /// Constructs a new Normalized container type
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self { value })
        } else {
            Err("Value must be between 0 and 1")
        }
    }

    /// Update the inner value of a normalized value
    pub fn get_value(&self) -> f64 {
        self.value
    }
}