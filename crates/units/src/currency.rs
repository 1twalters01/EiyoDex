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
    EUR,
    USD,
}

// Use json config file? No if stand alone library
impl CurrencyUnit {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            CurrencyUnit::GBP => "£",
            CurrencyUnit::EUR => "€",
            CurrencyUnit::USD => "$",
        }
    }

    pub fn as_code(&self) -> &'static str {
        match self {
            CurrencyUnit::GBP => "GBP",
            CurrencyUnit::EUR => "EUR",
            CurrencyUnit::USD => "USD",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            CurrencyUnit::GBP => "pound",
            CurrencyUnit::EUR => "Euro",
            CurrencyUnit::USD => "dollar",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            CurrencyUnit::GBP => "pounds",
            CurrencyUnit::EUR => "Euro",
            CurrencyUnit::USD => "dollars",
        }
    }
}

impl FromStr for CurrencyUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.trim().to_uppercase().as_str() {
            "£" | "GBP" => Ok(CurrencyUnit::GBP),
            "€" | "EUR" => Ok(CurrencyUnit::EUR),
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
        let current_unit = self.unit;
        if current_unit == target_unit {
            self
        } else {
            exchange_rate = get_exchange_rate(current_unit, target_unit)
            Self {
                value: self.value * exchange_rate,
                unit: target_unit,
            }
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.get_symbol(), self.value)
    }
}

impl Mul<f64> for Currency {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        // Match to avoid unnecessary conversion
        match self.unit {
            CurrencyUnit::GBP => Self::from_gbp(self.as_gbp() * rhs),
            CurrencyUnit::EUR => Self::from_eur(self.as_eur() * rhs),
            CurrencyUnit::USD => Self::from_usd(self.as_usd() * rhs),
        }
    }
}

impl Div<f64> for Currency {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        // Match to avoid unnecessary conversion
        match self.unit {
            CurrencyUnit::GBP => Self::from_gbp(self.as_gbp() / rhs),
            CurrencyUnit::EUR => Self::from_eur(self.as_eur() / rhs),
            CurrencyUnit::USD => Self::from_usd(self.as_usd() / rhs),
        }
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            match self.unit {
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() + rhs.as_gbp()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() + rhs.as_eur()),
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() + rhs.as_usd()),
            }
        } else {
            match self.unit {
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() + rhs.as_gbp()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() + rhs.as_eur()),
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() + rhs.as_usd()),
            }
        }
    }
}

impl Sub for Currency {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            match self.unit {
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() - rhs.as_gbp()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() - rhs.as_eur()),
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() - rhs.as_usd()),
            }
        } else {
            match self.unit {
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() - rhs.as_gbp()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() - rhs.as_eur()),
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() - rhs.as_usd()),
            }
        }
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_gbp().partial_cmp(&other.as_gbp())

        if self.unit == other.unit {
            match self.unit {
                CurrencyUnit::GBP => return self.as_gbp().partial_cmp(&other.as_gbp()),
                CurrencyUnit::EUR => return self.as_eur().partial_cmp(&other.as_eur()),
                CurrencyUnit::USD => return self.as_usd().partial_cmp(&other.as_usd()),
            }
        } else {
            match self.unit {
                CurrencyUnit::GBP => return self.as_gbp().partial_cmp(&other.as_gbp()),
                CurrencyUnit::EUR => return self.as_eur().partial_cmp(&other.as_eur()),
                CurrencyUnit::USD => return self.as_usd().partial_cmp(&other.as_usd()),
            }
        }
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// Use a library for good error typing / an enum
pub fn get_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
    if current_unit == target_unit{
        return Ok(1.0)
    }

    // connect to redis

    let key = format!("currency_exchange_rate:{}_to_{}", current.as_code(), target.as_code());

    // get cached exchange rate

    if let Ok(cached_exchange_rate) = cached_result {
        return Ok(cached_exchange_rate)
    }

    let live_exchange_rate_result = fetch_exchange_rate(current_unit, target_unit);

    match live_exchange_rate_result {
        Err(err) => return Err(err)
        Ok(live_exchange_rate) => {
            let exchange_rate_expiry_in_seconds = fetch_exchange_rate_expiry_in_seconds();

            // Cache result for exchange_rate_expiry_in_seconds seconds

            return Ok(live_exchange_rate)
        }
    }
}

pub fn fetch_current_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
    // fetch from an api using current unit, target unit and time now using reqwest
    // return result of fetch
}

// Don't know what datetime I will use, probably chrone
pub fn fetch_past_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit, datetime: Datetime) -> Result<f64, String> {
    // fetch from an api using current unit, target unit and datetime using reqwest
    // return result of fetch
}

pub fn fetch_exchange_rate_expiry_in_seconds() -> i8 {
    // fetch from somewhere e.g. toml file
    // success or default is 30 mins
}