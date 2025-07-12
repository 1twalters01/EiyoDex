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
pub enum CurrencyUnit {
    GBP,
    USD,
}

impl CurrencyUnit {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            CurrencyUnit::GBP => "£"
            CurrencyUnit::USD => "$"
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            currencyunit::GBP => "pound"
            currencyunit::USD => "dollar"
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            currencyunit::GBP => "pounds"
            currencyunit::USD => "dollars"
        }
    }

    pub fn as_code(&self) -> &'static str {
        match self {
            currencyunit::GBP => "GBP"
            currencyunit::USD => "USD"
        }
    }
}

impl FromStr for CurrencyUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.trim().to_uppercase().as_str() {
            "£" | "GBP" => Ok(CurrencyUnit::GBP),
            "$" | "USD" => Ok(CurrencyUnit::USD),
            _ => Err("Unknown currency unit"),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Currency {
    value: f64,
    unit: CurrencyUnit,
}

impl Currency {
    pub fn new(value: f64, unit: CurrencyUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_gbp(value: f64) -> Self {
        Self::new(value, CurrencyUnit::GBP)
    }

    pub fn from_usd(value: f64) -> Self {
        Self::new(value, CurrencyUnit::USD)
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

    pub fn get_code(&self) -> &'static str {
        self.unit.as_code()
    }

    pub fn to_unit(&self, target_unit: CurrencyUnit) -> Self {
        current_unit = self.unit;
        if current_unit == target_unit {
            self
        } else {
            exchange_rate = Currency::get_exchange_rate(current_unit, target_unit)
            Self {
                value: self.value * exchange_rate,
                unit: target_unit,
            }
        }
    }

    pub fn to_gbp(&self) -> Self {
        self.to_unit(CurrencyUnit::GBP)
    }

    pub fn to_usd(&self) -> Self {
        self.to_unit(CurrencyUnit::USD)
    }

    pub fn as_gbp(&self) -> f64 {
        self.to_gbp().value
    }

    pub fn as_usd(&self) -> f64 {
        self.to_usd().value
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.get_symbol(), self.value)
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_gbp(self.as_gbp() + rhs.as_gbp())
    }
}

impl Sub for Currency {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_gbp(self.as_kcal() - rhs.as_kcal())
    }
}

impl Mul<f64> for Currency {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        // Match to avoid unnecessary conversion
        match self.unit {
            CurrencyUnit::GBP => Self::from_gbp(self.as_gbp() * rhs)
            CurrencyUnit::GBP => Self::from_usd(self.as_usd() * rhs)
        }
    }
}

impl Div<f64> for Currency {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        // Match to avoid unnecessary conversion
        match self.unit {
            CurrencyUnit::GBP => Self::from_gbp(self.as_gbp() / rhs)
            CurrencyUnit::GBP => Self::from_usd(self.as_usd() / rhs)
        }
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_gbp().partial_cmp(&other.as_gbp())
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}