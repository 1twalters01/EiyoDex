use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::TryFrom,
    fmt,
    ops::{Add, Sub, Mul, Div},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub struct CurrencyMetadata {
    symbol: &'static str
    code: &'static str
    unit_type: &'static str
    unit_type_plural: &'static str
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CurrencyUnit {
    USD,
    EUR,
    JYP,
    GBP,
}

// Use json config file? No if stand alone library
impl CurrencyUnit {
    pub fn metadata(&self) -> CurrencyMetadata {
        match self {
            CurrencyUnit::USD => CurrencyMetadata {
                symbol: "$",
                code: "USD",
                unit_type: "Dollar",
                unit_type_plural: "Dollars",
            },
            CurrencyUnit::EUR => CurrencyMetadata {
                symbol: "€",
                code: "EUR",
                unit_type: "Euro",
                unit_type_plural: "Euros",
            },
            CurrencyUnit::JYP => CurrencyMetadata {
                symbol: "¥",
                code: "JYP",
                unit_type: "Yen",
                unit_type_plural: "Yen",
            },
            CurrencyUnit::GBP => CurrencyMetadata {
                symbol: "£",
                code: "GBP",
                unit_type: "Pounds",
                unit_type_plural: "Pounds",
            },
        }
    }

    pub fn variants() -> &'static [CurrencyUnit] {
        const VARIANTS: &[CurrencyUnit] = &[
            CurrencyUnit::USD,
            CurrencyUnit::EUR,
            CurrencyUnit::JPY,
            CurrencyUnit::GBP,
        ];
        VARIANTS
    }

    pub fn as_symbol(&self) -> &'static str {
        self.metadata().symbol
    }

    pub fn as_code(&self) -> &'static str {
        self.metadata().code
    }

    pub fn as_unit_type(&self) -> &'static str {
        self.metadata().unit_type
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        self.metadata().unit_type_plural
    }
}



impl FromStr for CurrencyUnit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let formatted_s = trim().uppercase();
        for variant in CurrencyUnit::variants() {
            let variant_metadata = variant.metadata();
            if formatted_s == variant_metadata.code || formatted_s == variant_metadata.symbol {
                return Ok(*variant);
            }
        }
        Err("Unknown currency unit")
    }
}

impl fmt::Display for CurrencyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_code())
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
            CurrencyUnit::USD => Self::from_usd(self.as_usd() * rhs),
            CurrencyUnit::EUR => Self::from_eur(self.as_eur() * rhs),
            CurrencyUnit::YEN => Self::from_yen(self.as_yen() * rhs),
            CurrencyUnit::GBP => Self::from_gbp(self.as_gbp() * rhs),
        }
    }
}

impl Div<f64> for Currency {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        // Match to avoid unnecessary conversion
        match self.unit {
            CurrencyUnit::USD => Self::from_usd(self.as_usd() / rhs),
            CurrencyUnit::EUR => Self::from_eur(self.as_eur() / rhs),
            CurrencyUnit::YEN => Self::from_yen(self.as_yen() / rhs),
            CurrencyUnit::GBP => Self::from_gbp(self.as_gbp() / rhs),
        }
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            match self.unit {
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() + rhs.as_usd()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() + rhs.as_eur()),
                CurrencyUnit::YEN => return Self::from_yen(self.as_yen() + rhs.as_yen()),
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() + rhs.as_gbp()),
            }
        } else {
            match self.unit {
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() + rhs.as_usd()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() + rhs.as_eur()),
                CurrencyUnit::YEN => return Self::from_yen(self.as_yen() + rhs.as_yen()),
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() + rhs.as_gbp()),
            }
        }
    }
}

impl Sub for Currency {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.unit == rhs.unit {
            match self.unit {
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() - rhs.as_usd()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() - rhs.as_eur()),
                CurrencyUnit::YEN => return Self::from_yen(self.as_yen() - rhs.as_yen()),
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() - rhs.as_gbp()),
            }
        } else {
            match self.unit {
                CurrencyUnit::USD => return Self::from_usd(self.as_usd() - rhs.as_usd()),
                CurrencyUnit::EUR => return Self::from_eur(self.as_eur() - rhs.as_eur()),
                CurrencyUnit::YEN => return Self::from_yen(self.as_yen() - rhs.as_yen()),
                CurrencyUnit::GBP => return Self::from_gbp(self.as_gbp() - rhs.as_gbp()),
            }
        }
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.unit == other.unit {
            match self.unit {
                CurrencyUnit::USD => return self.as_usd().partial_cmp(&other.as_usd()),
                CurrencyUnit::EUR => return self.as_eur().partial_cmp(&other.as_eur()),
                CurrencyUnit::YEN => return self.as_yen().partial_cmp(&other.as_yen()),
                CurrencyUnit::GBP => return self.as_gbp().partial_cmp(&other.as_gbp()),
            }
        } else {
            match self.unit {
                CurrencyUnit::USD => return self.as_usd().partial_cmp(&other.as_usd()),
                CurrencyUnit::EUR => return self.as_eur().partial_cmp(&other.as_eur()),
                CurrencyUnit::YEN => return self.as_yen().partial_cmp(&other.as_yen()),
                CurrencyUnit::GBP => return self.as_gbp().partial_cmp(&other.as_gbp()),
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