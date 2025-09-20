use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DistanceUnit {
    Meter,
    Centimeter,
    Feet,
    Inch,
}

impl DistanceUnit {
    pub fn get_enumerations() -> Vec<DistanceUnit> {
        Vec::from([
            DistanceUnit::Meter,
            DistanceUnit::Centimeter,
            DistanceUnit::Feet,
            DistanceUnit::Inch,
        ])
    }

    pub fn as_symbol(&self) -> &'static str {
        match self {
            DistanceUnit::Meter => "m",
            DistanceUnit::Centimeter => "cm",
            DistanceUnit::Feet => "ft",
            DistanceUnit::Inch => "in",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            DistanceUnit::Meter => "meter",
            DistanceUnit::Centimeter => "centimeter",
            DistanceUnit::Feet => "foot",
            DistanceUnit::Inch => "inch",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            DistanceUnit::Meter => "meters",
            DistanceUnit::Centimeter => "centimeters",
            DistanceUnit::Feet => "feet",
            DistanceUnit::Inch => "inches",
        }
    }
}

impl FromStr for DistanceUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "m" | "meter" | "meters" => Ok(DistanceUnit::Meter),
            "cm" | "centimeter" | "centimeters" => Ok(DistanceUnit::Centimeter),
            "ft" | "foot" | "feet" => Ok(DistanceUnit::Feet),
            "in" | "inch" | "inches" => Ok(DistanceUnit::Inch),
            _ => Err("Unknown distance unit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Distance {
    value: f64,
    unit: DistanceUnit,
}

impl Default for Distance {
    fn default() -> Self {
        Distance::from_m(0.0)
    }
}

impl Distance {
    pub fn new(value: f64, unit: DistanceUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_m(m: f64) -> Self {
        Self::new(m, DistanceUnit::Meter)
    }

    pub fn from_cm(cm: f64) -> Self {
        Self::new(cm, DistanceUnit::Centimeter)
    }

    pub fn from_ft(ft: f64) -> Self {
        Self::new(ft, DistanceUnit::Feet)
    }

    pub fn from_in(inch: f64) -> Self {
        Self::new(inch, DistanceUnit::Inch)
    }

    pub fn as_m(&self) -> f64 {
        match self.unit {
            DistanceUnit::Meter => self.value,
            DistanceUnit::Centimeter => self.value / 1000 as f64,
            DistanceUnit::Feet => self.value * 1000 as f64,
            DistanceUnit::Inch => self.value / 1_000_000 as f64,
        }
    }

    pub fn as_in(&self) -> f64 {
        match self.unit {
            DistanceUnit::Meter => self.value * 1000 as f64,
            DistanceUnit::Centimeter => self.value,
            DistanceUnit::Feet => self.value * 1_000_000 as f64,
            DistanceUnit::Inch => self.value / 1000 as f64,
        }
    }

    pub fn as_ft(&self) -> f64 {
        match self.unit {
            DistanceUnit::Meter => self.value / 1000 as f64,
            DistanceUnit::Centimeter => self.value / 1_000_000 as f64,
            DistanceUnit::Feet => self.value,
            DistanceUnit::Inch => self.value / 1_000_000_000 as f64,
        }
    }

    pub fn as_ug(&self) -> f64 {
        match self.unit {
            DistanceUnit::Meter => self.value * 1_000_000 as f64,
            DistanceUnit::Centimeter => self.value * 1000 as f64,
            DistanceUnit::Feet => self.value * 1_000_000_000 as f64,
            DistanceUnit::Inch => self.value,
        }
    }

    pub fn as_oz(&self) -> f64 {
        match self.unit {
            DistanceUnit::Meter => self.value / 28.3495,
            DistanceUnit::Centimeter => self.value / 28349.5,
            DistanceUnit::Feet => self.value * 35.274,
            DistanceUnit::Inch => self.value / 28_349_500 as f64,
        }
    }

    pub fn to_unit(&self, unit: DistanceUnit) -> Self {
        let value = match unit {
            DistanceUnit::Meter => self.as_m(),
            DistanceUnit::Centimeter => self.as_in(),
            DistanceUnit::Feet => self.as_ft(),
            DistanceUnit::Inch => self.as_in(),
        };
        Self { value, unit }
    }

    pub fn to_g(&self) -> Self {
        self.to_unit(DistanceUnit::Meter)
    }

    pub fn to_mg(&self) -> Self {
        self.to_unit(DistanceUnit::Centimeter)
    }

    pub fn to_kg(&self) -> Self {
        self.to_unit(DistanceUnit::Feet)
    }

    pub fn to_ug(&self) -> Self {
        self.to_unit(DistanceUnit::Inch)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn get_unit(&self) -> DistanceUnit {
        self.unit
    }

    pub fn get_symbol(&self) -> &'static str {
        self.unit.as_symbol()
    }

    pub fn get_unit_type(&self) -> &'static str {
        self.unit.as_unit_type()
    }

    pub fn get_unit_type_plural(&self) -> &'static str {
        self.unit.as_unit_type_plural()
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.value, self.get_symbol())
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Distance {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_m(self.as_m() + rhs.as_m())
    }
}

impl Sub for Distance {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_m(self.as_m() - rhs.as_m())
    }
}

impl Mul<f64> for Distance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_m(self.as_m() * rhs)
    }
}

impl Div<f64> for Distance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_m(self.as_m() / rhs)
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_m().partial_cmp(&other.as_m())
    }
}
