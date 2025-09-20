use std::{
    cmp::Ordering,
    fmt,
    ops::{Div, Mul},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    currency::{fetch_current_exchange_rate, Currency, CurrencyUnit},
    mass::Mass,
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SpecificCurrencyUnit {
    GBPPerGram,
    USDPerGram,
}

impl SpecificCurrencyUnit {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            SpecificCurrencyUnit::GBPPerGram => "£/g",
            SpecificCurrencyUnit::USDPerGram => "$/g",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            SpecificCurrencyUnit::GBPPerGram => "Pound per gram",
            SpecificCurrencyUnit::USDPerGram => "Dollar per gram",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            SpecificCurrencyUnit::GBPPerGram => "Pounds per gram",
            SpecificCurrencyUnit::USDPerGram => "Dollars per gram",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpecificCurrency {
    value: f64,
    unit: SpecificCurrencyUnit,
}

impl Default for SpecificCurrency {
    fn default() -> Self {
        SpecificCurrency::from_usd_per_g(0.0)
    }
}

impl SpecificCurrency {
    pub fn new(value: f64, unit: SpecificCurrencyUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_usd_per_g(dollars_per_g: f64) -> Self {
        Self::new(dollars_per_g, SpecificCurrencyUnit::USDPerGram)
    }

    pub fn from_gbp_per_g(pounds_per_g: f64) -> Self {
        Self::new(pounds_per_g, SpecificCurrencyUnit::GBPPerGram)
    }

    pub fn as_usd_per_g(&self) -> f64 {
        match self.unit {
            SpecificCurrencyUnit::USDPerGram => self.value,
            SpecificCurrencyUnit::GBPPerGram => {
                let current_unit = CurrencyUnit::GBP;
                let target_unit = CurrencyUnit::USD;
                self.value * fetch_current_exchange_rate(current_unit, target_unit).unwrap()
            }
        }
    }

    pub fn as_gbp_per_g(&self) -> f64 {
        match self.unit {
            SpecificCurrencyUnit::USDPerGram => self.value,
            SpecificCurrencyUnit::GBPPerGram => {
                let current_unit = CurrencyUnit::USD;
                let target_unit = CurrencyUnit::GBP;
                self.value * fetch_current_exchange_rate(current_unit, target_unit).unwrap()
            }
        }
    }

    pub fn get_currency_for_mass(&self, mass: Mass) -> Currency {
        Currency::new(self.as_usd_per_g() * mass.as_g(), CurrencyUnit::USD)
    }

    pub fn get_mass_for_currency(&self, currency: Currency) -> Mass {
        let currency = currency.convert_to(CurrencyUnit::USD).unwrap();
        Mass::from_g(currency.get_value() / self.as_usd_per_g())
    }

    pub fn to_unit(&self, unit: SpecificCurrencyUnit) -> Self {
        let value = match unit {
            SpecificCurrencyUnit::USDPerGram => self.as_usd_per_g(),
            SpecificCurrencyUnit::GBPPerGram => self.as_gbp_per_g(),
        };
        Self { value, unit }
    }

    pub fn to_usd_per_g(&self) -> Self {
        self.to_unit(SpecificCurrencyUnit::USDPerGram)
    }

    pub fn to_gbp_per_g(&self) -> Self {
        self.to_unit(SpecificCurrencyUnit::GBPPerGram)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn get_unit(&self) -> SpecificCurrencyUnit {
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

impl fmt::Display for SpecificCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Mul<Mass> for SpecificCurrency {
    type Output = Currency;

    fn mul(self, mass: Mass) -> Currency {
        self.get_currency_for_mass(mass)
    }
}

impl Mul<SpecificCurrency> for Mass {
    type Output = Currency;

    fn mul(self, specific_currency: SpecificCurrency) -> Currency {
        specific_currency.get_currency_for_mass(self)
    }
}

// d = m/v
// sc = c/m
impl Div<Mass> for Currency {
    type Output = SpecificCurrency;

    fn div(self, mass: Mass) -> SpecificCurrency {
        let usd = self.convert_to(CurrencyUnit::USD).unwrap().get_value();
        let grams = mass.as_g();
        let usd_per_gram: f64 = usd / grams;
        SpecificCurrency::from_usd_per_g(usd_per_gram)
    }
}

impl PartialOrd for SpecificCurrency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_usd_per_g().partial_cmp(&other.as_usd_per_g())
    }
}
