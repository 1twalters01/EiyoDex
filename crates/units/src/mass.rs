use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt,
    ops::{Add, Sub, Mul, Div},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MassUnit {
    Gram,
    Milligrams,
    Kilogram,
    Ounce,
}

impl struct MassUnit {
    pub fn as_symbol(&self) -> String {
        match self {
            MassUnit::Gram => String::new("g")
            MassUnit::Milligram => String::new("mg")
            MassUnit::Kilogram => String::new("kg")
            MassUnit::Ounce => String::new("oz")
        }
    }

    pub fn as_unit_type(&self) -> String {
        match self {
            MassUnit::Gram => String::new("gram")
            MassUnit::Milligram => String::new("milligram")
            MassUnit::Kilogram => String::new("kilogram")
            MassUnit::Ounce => String::new("ounce")
        }
    }

    pub fn as_unit_type_plural(&self) -> String {
        match self {
            MassUnit::Gram => String::new("grams")
            MassUnit::Milligram => String::new("milligrams")
            MassUnit::Kilogram => String::new("kilograms")
            MassUnit::Ounce => String::new("ounces")
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

impl Default for Energy {
    fn default() -> Self {
        Mass::from_grams(0.0)
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
            MassUnit::Milligram => self.value / 1000,
            MassUnit::Kilogram => self.value * 1000,
            MassUnit::Ounce => self.value * 28.3495,
        }
        self.value_in_grams
    }

    pub fn as_mg(&self) -> Self {
        match self.unit {
            MassUnit::Gram => self.value * 1000,
            MassUnit::Milligram => self.value,
            MassUnit::Kilogram => self.value * 1,000,000,
            MassUnit::Ounce => self.value * 28,349.5,
        }
    }

    pub fn as_kg(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value / 1000,
            MassUnit::Milligram => self.value / 1,000,000,
            MassUnit::Kilogram => self.value,
            MassUnit::Ounce => self.value * 0.0283495,
        }
    }

    pub fn as_oz(&self) -> f64 {
        match self.unit {
            MassUnit::Gram => self.value / 28.3495,
            MassUnit::Milligram => self.value / 28349.5,
            MassUnit::Kilogram => self.value * 35.274,
            MassUnit::Ounce => self.value,
        }
    }

    pub fn to_unit(&self, unit: EnergyUnit) -> Self {
        let value = match unit {
            MassUnit::Gram => self.as_grams(),
            MassUnit::Milligram => self.as_milligrams(),
            MassUnit::Kilogram => self.as_kilograms(),
            MassUnit::Ounce => self.as_ounces(),
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

    pub fn to_string(&self) -> Strint {
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
        Self::from_grams(self.as_grams() + rhs.as_grams())
    }
}

impl Sub for Mass {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_grams(self.as_grams() - rhs.as_grams())
    }
}

impl Mul for Mass {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_grams(self.as_grams() * rhs)
    }
}

impl Div for Mass {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_grams(self.as_grams() / rhs)
    }
}

impl PartialOrd for Mass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_kcal().partial_cmp(&other.as_grams())
    }
}

impl Ord for Mass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}