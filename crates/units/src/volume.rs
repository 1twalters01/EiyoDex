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
pub enum VolumeUnit {
    Liter,
    Milliliter,
}

impl VolumeUnit {
    pub fn get_enumerations() -> Vec<VolumeUnit> {
        Vec::from([
            VolumeUnit::Liter,
            VolumeUnit::Milliliter,
        ])
    }

    pub fn as_symbol(&self) -> &'static str {
        match self {
            VolumeUnit::Liter => "l",
            VolumeUnit::Milliliter => "ml",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            VolumeUnit::Liter => "liter",
            VolumeUnit::Milliliter => "milliliter",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            VolumeUnit::Liter => "liters",
            VolumeUnit::Milliliter => "milliliters",
        }
    }
}

impl FromStr for VolumeUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "l" | "liter" | "liters" => Ok(VolumeUnit::Liter),
            "ml" | "milliliter" | "milliliters" => Ok(VolumeUnit::Milliliter),
            _ => Err("Unknown volume unit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Volume {
    value: f64,
    unit: VolumeUnit,
}

impl Default for Volume {
    fn default() -> Self {
        Volume::from_ml(0.0)
    }
}

impl Volume {
    pub fn new(value: f64, unit: VolumeUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_ml(ml: f64) -> Self {
        Self::new(ml, VolumeUnit::Milliliter)
    }

    pub fn from_l(l: f64) -> Self {
        Self::new(l, VolumeUnit::Liter)
    }

    pub fn as_ml(&self) -> f64 {
        match self.unit {
            VolumeUnit::Liter => self.value * 1000 as f64,
            VolumeUnit::Milliliter => self.value,
        }
    }

    pub fn as_l(&self) -> f64 {
        match self.unit {
            VolumeUnit::Liter => self.value,
            VolumeUnit::Milliliter => self.value / 1000 as f64,
        }
    }

    pub fn to_unit(&self, unit: VolumeUnit) -> Self {
        let value = match unit {
            VolumeUnit::Liter => self.as_l(),
            VolumeUnit::Milliliter => self.as_ml(),
        };
        Self { value, unit }
    }

    pub fn to_ml(&self) -> Self {
        self.to_unit(VolumeUnit::Milliliter)
    }

    pub fn to_l(&self) -> Self {
        self.to_unit(VolumeUnit::Liter)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn get_unit(&self) -> VolumeUnit {
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

impl fmt::Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Volume {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_l(self.as_l() + rhs.as_l())
    }
}

impl Sub for Volume {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_l(self.as_l() - rhs.as_l())
    }
}

impl Mul<f64> for Volume {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_l(self.as_l() * rhs)
    }
}

impl Div<f64> for Volume {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_l(self.as_l() / rhs)
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ml().partial_cmp(&other.as_ml())
    }
}
