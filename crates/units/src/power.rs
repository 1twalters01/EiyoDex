use crate::energy::Energy;
use chrono::Duration;
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
pub enum PowerUnit {
    KcalPerSecond,
    Watts,
}

impl PowerUnit {
    pub fn get_enumerations() -> Vec<PowerUnit> {
        Vec::from([PowerUnit::KcalPerSecond, PowerUnit::Watts])
    }

    pub fn as_symbol(&self) -> &'static str {
        match self {
            PowerUnit::KcalPerSecond => "kcal_per_second",
            PowerUnit::Watts => "W",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            PowerUnit::KcalPerSecond => "kcal per second",
            PowerUnit::Watts => "watt",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            PowerUnit::KcalPerSecond => "kcals per second",
            PowerUnit::Watts => "watts",
        }
    }
}

impl FromStr for PowerUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "kcal_per_second" | "kcal per second" | "kcals per second" => {
                Ok(PowerUnit::KcalPerSecond)
            }
            "W" | "watt" | "watts" => Ok(PowerUnit::Watts),
            _ => Err("Unknown power unit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Power {
    value: f64,
    unit: PowerUnit,
}

impl Default for Power {
    fn default() -> Self {
        Power::from_watts(0.0)
    }
}

impl Power {
    pub fn new(value: f64, unit: PowerUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_watts(watts: f64) -> Self {
        Self::new(watts, PowerUnit::Watts)
    }

    pub fn from_kcal_per_second(kcal_per_second: f64) -> Self {
        Self::new(kcal_per_second, PowerUnit::KcalPerSecond)
    }

    pub fn as_watts(&self) -> f64 {
        match self.unit {
            PowerUnit::KcalPerSecond => self.value * 1000 as f64,
            PowerUnit::Watts => self.value,
        }
    }

    pub fn as_kcal_per_second(&self) -> f64 {
        match self.unit {
            PowerUnit::KcalPerSecond => self.value,
            PowerUnit::Watts => self.value / 1000 as f64,
        }
    }

    pub fn to_unit(&self, unit: PowerUnit) -> Self {
        let value = match unit {
            PowerUnit::KcalPerSecond => self.as_kcal_per_second(),
            PowerUnit::Watts => self.as_watts(),
        };
        Self { value, unit }
    }

    pub fn to_ml(&self) -> Self {
        self.to_unit(PowerUnit::Watts)
    }

    pub fn to_l(&self) -> Self {
        self.to_unit(PowerUnit::KcalPerSecond)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn get_unit(&self) -> PowerUnit {
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

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_kcal_per_second(self.as_kcal_per_second() + rhs.as_kcal_per_second())
    }
}

impl Sub for Power {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_kcal_per_second(self.as_kcal_per_second() - rhs.as_kcal_per_second())
    }
}

impl Mul<f64> for Power {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::from_kcal_per_second(self.as_kcal_per_second() * rhs)
    }
}

impl Mul<Duration> for Power {
    type Output = Energy;

    fn mul(self, rhs: Duration) -> Energy {
        let seconds = rhs.num_seconds() as f64;
        Energy::from_kcal(self.as_kcal_per_second() * seconds)
    }
}

impl Mul<Power> for Duration {
    type Output = Energy;

    fn mul(self, rhs: Power) -> Energy {
        let seconds: f64 = self.num_seconds() as f64;
        Energy::from_kcal(seconds as f64 * rhs.as_kcal_per_second())
    }
}

impl Div<f64> for Power {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::from_kcal_per_second(self.as_kcal_per_second() / rhs)
    }
}

impl Div<Duration> for Energy {
    type Output = Power;

    fn div(self, duration: Duration) -> Power {
        let seconds: f64 = duration.num_seconds() as f64;
        let kcal = self.as_kcal();
        Power::from_kcal_per_second(kcal / seconds)
    }
}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_kcal_per_second()
            .partial_cmp(&other.as_kcal_per_second())
    }
}
