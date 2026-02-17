#[derive(Deserialize)]
struct ExchangeResponse {
    rates: Rates,
}

#[derive(Deserialize)]
struct Rates {
    #[serde(rename = "USD")]
    usd: f64,
}

#[macro_export]
macro_rules! define_currencies {
    (
        $(
            $variant:ident => {
                from_fn_name: $from_fn_name:ident,
                as_fn_name: $as_fn_name:ident,
                to_fn_name: $to_fn_name:ident,
            }
        ),+ $(,)?
    ) => {
        use chrono::NaiveDate;
        use crate::currency_unit::CurrencyUnit;
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct Currency {
            value: f64,
            unit: CurrencyUnit,
        }

        impl Currency {
            pub fn new(value: f64, unit: CurrencyUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $from_fn_name(value: f64) -> Self {
                    Self::new(value, CurrencyUnit::$variant)
                }
            )+

            pub fn round(&mut self, dp: u8) -> Self {
                let factor = 10f64.powi(dp as i32);
                self.value = (self.value * factor).round()/factor;
                return *self
            }

            pub fn is_zero(&self) -> bool {
                self.value == 0.0
            }

            pub fn is_negative(&self) -> bool {
                self.value < 0.0
            }

            pub fn get_value(&self) -> f64 {
                self.value
            }

            pub fn set_value(&mut self, value: f64) {
                self.value = value;
            }

            pub fn get_unit(&self) -> CurrencyUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: CurrencyUnit) {
                self.unit = unit;
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

            pub async fn convert_to_async(&self, target_unit: CurrencyUnit) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_current_exchange_rate_async(&target_unit).await {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub fn convert_to_sync(&self, target_unit: CurrencyUnit) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_current_exchange_rate_sync(&target_unit) {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub async fn convert_to_historic_async(&self, target_unit: CurrencyUnit, date: NaiveDate) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_past_exchange_rate_async(&target_unit, date).await {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub fn convert_to_historic_sync(&self, target_unit: CurrencyUnit, date: NaiveDate) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_past_exchange_rate_sync(&target_unit, date) {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            $(
                pub fn $as_fn_name(&self) -> Result<f64, String> {
                    self.convert_to_sync(CurrencyUnit::$variant).map(|currency| currency.value)
                }
            )+

            $(
                pub fn $to_fn_name(&self) -> Result<Currency, String> {
                    self.convert_to_sync(CurrencyUnit::$variant)
                }
            )+
        }

        impl fmt::Display for Currency {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", self.get_symbol(), self.value)
            }
        }

        impl Add for Currency {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                match rhs.convert_to_sync(self.unit) {
                    Ok(converted_rhs) => Self::new(self.value + converted_rhs.value, self.unit),
                    Err(_) => panic!("Currency conversion failed in addition"),
                }
            }
        }

        impl Sub for Currency {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                match rhs.convert_to_sync(self.unit) {
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
                match other.convert_to_sync(self.unit) {
                    Ok(converted_other) => self.value.partial_cmp(&converted_other.value),
                    Err(_) => None,
                }
            }
        }
   };
}

use units_macro::include_currencies_from_json;
include_currencies_from_json!("data/units/currency");
