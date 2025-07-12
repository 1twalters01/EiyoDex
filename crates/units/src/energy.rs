use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Sub, Mul, Div}
};
use std::str::FromStr;
use std::convert::TryFrom;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EnergyUnit {
    Kcal,
    KJ,
}

impl EnergyUnit {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            EnergyUnit::Kcal => "kcal"
            EnergyUnit::KJ => "kJ"
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            EnergyUnit::Kcal => "kilocalorie"
            EnergyUnit::KJ => "kilojoule"
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            EnergyUnit::Kcal => "kilocalories"
            EnergyUnit::KJ => "kilojoules"
        }
    }
}

impl FromStr for EnergyUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "kcal" | "kilocalorie" | "kilocalories" => Ok(EnergyUnit::Kcal),
            "kj" | "kilojoule" | "kilojoules" => Ok(EnergyUnit::KJ),
            _ => Err("Unknown energy unit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Energy {
    value: f64,
    unit: EnergyUnit,
}

impl Default for Energy {
    fn default() -> Self {
        Energy::from_kcal(0.0)
    }
}

impl Energy {
    pub fn new(value: f64, unit: EnergyUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_kcal(kcal: f64) -> Self {
        Self::new(kcal, EnergyUnit::Kcal)
    }

    pub fn from_kj(kj: f64) -> Self {
        Self::new(kj, EnergyUnit::KJ)
    }

    pub fn as_kcal(&self) -> f64 {
        match self.unit {
            EnergyUnit::Kcal => self.value,
            EnergyUnit::KJ => self.value / 4.184,
        }
    }

    pub fn as_kj(&self) -> f64 {
        match self.unit {
            EnergyUnit::Kcal => self.value * 4.184,
            EnergyUnit::KJ => self.value,
        }
    }

    pub fn to_unit(&self, unit: EnergyUnit) -> Self {
        let value = match unit {
            EnergyUnit::Kcal => self.as_kcal(),
            EnergyUnit::KJ => self.as_kj(),
        };
        Self { value, unit }
    }

    pub fn to_kcal(&self) -> Self {
        self.to_unit(EnergyUnit::Kcal)
    }

    pub fn to_kj(&self) -> Self {
        self.to_unit(EnergyUnit::KJ)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
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

    pub fn to_string(&self) -> Strint {
        format!("{} {}", self.value, self.get_symbol())
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Energy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_kcal(self.as_kcal() + rhs.as_kcal())
    }
}

impl Sub for Energy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_kcal(self.as_kcal() - rhs.as_kcal())
    }
}

impl Mul<f64> for Energy {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_kcal(self.as_kcal() * rhs)
    }
}

impl Div<f64> for Energy {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_kcal(self.as_kcal() / rhs)
    }
}

impl PartialOrd for Energy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_kcal().partial_cmp(&other.as_kcal())
    }
}

impl Ord for Energy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}