#[macro_export]
macro_rules! define_currencies {
    (
        $(
            $variant:ident => {
                symbol: $symbol:expr,
                code: $code:expr,
                unit_type: $unit:expr,
                unit_type_plural: $unit_plural:expr
            }
        ),+ $(,)?
    ) => {
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            str::FromStr,
        };
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        pub struct CurrencyMetadata {
            pub symbol: &'static str,
            pub code: &'static str,
            pub unit_type: &'static str,
            pub unit_type_plural: &'static str,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum CurrencyUnit {
            $($variant),+
        }

        impl CurrencyUnit {
            pub fn variants() -> &'static [CurrencyUnit] {
                &[
                    $(CurrencyUnit::$variant),+
                ]
            }

            pub fn metadata(&self) -> CurrencyMetadata {
                match self {
                    $(CurrencyUnit::$variant => CurrencyMetadata {
                        symbol: $symbol,
                        code: $code,
                        unit_type: $unit,
                        unit_type_plural: $unit_plural,
                    }),+
                }
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

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_s = s.trim().to_uppercase();
                for variant in Self::variants() {
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

            pub fn get_value(&self) -> f64 {
                self.value
            }

            fn convert_to(&self, target_unit: CurrencyUnit) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match fetch_current_exchange_rate(self.unit, target_unit) {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }
        }

        impl fmt::Display for Currency {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", self.get_symbol(), self.value)
            }
        }

        impl Add for Currency {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                match rhs.convert_to(self.unit) {
                    Ok(converted_rhs) => Self::new(self.value + converted_rhs.value, self.unit),
                    Err(_) => panic!("Currency conversion failed in addition"),
                }
            }
        }
        impl Sub for Currency {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                match rhs.convert_to(self.unit) {
                    Ok(converted_rhs) => Self::new(self.value - converted_rhs.value, self.unit),
                    Err(_) => panic!("Currency conversion failed in subtraction"),
                }
            }
        }

        impl Mul<f64> for Currency {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self::Output {
                Self::new(self.value * rhs, self.unit)
            }
        }

        impl Div<f64> for Currency {
            type Output = Self;

            fn div(self, rhs: f64) -> Self::Output {
                Self::new(self.value / rhs, self.unit)
            }
        }

        impl PartialOrd for Currency {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match other.convert_to(self.unit) {
                    Ok(converted_other) => self.value.partial_cmp(&converted_other.value),
                    Err(_) => None,
                }
            }
        }

        pub fn fetch_current_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
            // fetch from an api using current unit, target unit and time now using reqwest
            // return result of fetch
            todo!()
        }

        // Don't know what datetime I will use, probably chrone
        use std::time::SystemTime;
        pub fn fetch_past_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit, datetime: SystemTime) -> Result<f64, String> {
            // fetch from an api using current unit, target unit and datetime using reqwest
            // return result of fetch
            todo!()
        }

        pub fn fetch_exchange_rate_expiry_in_seconds() -> i8 {
            // fetch from somewhere e.g. toml file
            // success or default is 30 mins
            todo!()
        }
    };
}
