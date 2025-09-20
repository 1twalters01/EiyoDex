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
pub enum MassUnit {
    Gram,
    Milligram,
    Kilogram,
    Microgram,
    Ounce,
}

impl MassUnit {
    pub fn get_enumerations() -> Vec<MassUnit> {
        Vec::from([
            MassUnit::Gram,
            MassUnit::Milligram,
            MassUnit::Kilogram,
            MassUnit::Microgram,
            MassUnit::Ounce,
        ])
    }

    pub fn as_symbol(&self) -> &'static str {
        match self {
            MassUnit::Gram => "g",
            MassUnit::Milligram => "mg",
            MassUnit::Kilogram => "kg",
            MassUnit::Microgram => "µg",
            MassUnit::Ounce => "oz",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            MassUnit::Gram => "gram",
            MassUnit::Milligram => "milligram",
            MassUnit::Kilogram => "kilogram",
            MassUnit::Microgram => "microgram",
            MassUnit::Ounce => "ounce",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            MassUnit::Gram => "grams",
            MassUnit::Milligram => "milligrams",
            MassUnit::Kilogram => "kilograms",
            MassUnit::Microgram => "micrograms",
            MassUnit::Ounce => "ounces",
        }
    }
}

impl FromStr for MassUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "g" | "gram" | "grams" => Ok(MassUnit::Gram),
            "mg" | "milligram" | "milligrams" => Ok(MassUnit::Milligram),
            "kg" | "kilogram" | "kilograms" => Ok(MassUnit::Kilogram),
            "µg" | "microgram" | "micrograms" => Ok(MassUnit::Microgram),
            "oz" | "ounce" | "ounces" => Ok(MassUnit::Ounce),
            _ => Err("Unknown mass unit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mass {
    value: f64,
    unit: MassUnit,
}

impl Default for Mass {
    fn default() -> Self {
        Mass::from_g(0.0)
    }
}

impl Mass {
    pub fn new(value: f64, unit: MassUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_g(g: f64) -> Self {
        Self::new(g, MassUnit::Gram)
    }

    pub fn from_ml(mg: f64) -> Self {
        Self::new(mg, MassUnit::Milligram)
    }

    pub fn from_kg(kg: f64) -> Self {
        Self::new(kg, MassUnit::Kilogram)
    }

    pub fn from_oz(oz: f64) -> Self {
        Self::new(oz, MassUnit::Ounce)
    }

    pub fn as_g(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value,
            MassUnit::Milligram => self.value / 1000 as f64,
            MassUnit::Kilogram => self.value * 1000 as f64,
            MassUnit::Microgram => self.value / 1_000_000 as f64,
            MassUnit::Ounce => self.value * 28.3495,
        }
    }

    pub fn as_mg(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value * 1000 as f64,
            MassUnit::Milligram => self.value,
            MassUnit::Kilogram => self.value * 1_000_000 as f64,
            MassUnit::Microgram => self.value / 1000 as f64,
            MassUnit::Ounce => self.value * 28_349.5,
        }
    }

    pub fn as_kg(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value / 1000 as f64,
            MassUnit::Milligram => self.value / 1_000_000 as f64,
            MassUnit::Kilogram => self.value,
            MassUnit::Microgram => self.value / 1_000_000_000 as f64,
            MassUnit::Ounce => self.value * 0.0283495,
        }
    }

    pub fn as_ug(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value * 1_000_000 as f64,
            MassUnit::Milligram => self.value * 1000 as f64,
            MassUnit::Kilogram => self.value * 1_000_000_000 as f64,
            MassUnit::Microgram => self.value,
            MassUnit::Ounce => self.value * 28_349_500 as f64,
        }
    }

    pub fn as_oz(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value / 28.3495,
            MassUnit::Milligram => self.value / 28349.5,
            MassUnit::Kilogram => self.value * 35.274,
            MassUnit::Microgram => self.value / 28_349_500 as f64,
            MassUnit::Ounce => self.value,
        }
    }

    pub fn to_unit(&self, unit: MassUnit) -> Self {
        let value = match unit {
            MassUnit::Gram => self.as_g(),
            MassUnit::Milligram => self.as_mg(),
            MassUnit::Kilogram => self.as_kg(),
            MassUnit::Microgram => self.as_mg(),
            MassUnit::Ounce => self.as_oz(),
        };
        Self { value, unit }
    }

    pub fn to_g(&self) -> Self {
        self.to_unit(MassUnit::Gram)
    }

    pub fn to_mg(&self) -> Self {
        self.to_unit(MassUnit::Milligram)
    }

    pub fn to_kg(&self) -> Self {
        self.to_unit(MassUnit::Kilogram)
    }

    pub fn to_ug(&self) -> Self {
        self.to_unit(MassUnit::Microgram)
    }

    pub fn to_oz(&self) -> Self {
        self.to_unit(MassUnit::Ounce)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn get_unit(&self) -> MassUnit {
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

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Mass {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_g(self.as_g() + rhs.as_g())
    }
}

impl Sub for Mass {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_g(self.as_g() - rhs.as_g())
    }
}

impl Mul<f64> for Mass {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_g(self.as_g() * rhs)
    }
}

impl Div<f64> for Mass {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_g(self.as_g() / rhs)
    }
}

impl PartialOrd for Mass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_g().partial_cmp(&other.as_g())
    }
}
