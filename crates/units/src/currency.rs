#[macro_export]
macro_rules! define_currencies {
    (
        $(
            $variant:ident => {
                symbol: $symbol:expr,
                code: $code:expr,
                unit_type: $unit_type:expr,
                unit_type_plural: $unit_type_plural:expr
            }
        ),+ $(,)?
    ) => {
        use chrono::NaiveDateTime;
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        pub struct CurrencyMetadata {
            pub symbol: &'static str,
            pub code: &'static str,
            pub unit_type: &'static str,
            pub unit_type_plural: &'static str,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
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
                        unit_type: $unit_type,
                        unit_type_plural: $unit_type_plural,
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

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
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

            pub async fn convert_to(&self, target_unit: CurrencyUnit) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match fetch_current_exchange_rate(self.unit, target_unit).await {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub fn convert_to_sync(&self, target_unit: CurrencyUnit) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match fetch_current_exchange_rate_sync(self.unit, target_unit) {
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


        #[derive(Deserialize)]
        struct ExchangeResponse {
            result: f64,
        }

        pub async fn fetch_current_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
            let url = format!(
                "https://api.exchangerate.host/convert?from={}&to={}",
                current_unit.to_string(),
                target_unit.to_string()
            );
            let resp = reqwest::get(&url)
                .await
                .map_err(|e| e.to_string())?;
    
            // Save to postgres and/or redis
            todo!();

            let data: ExchangeResponse = resp.json().await.map_err(|e| e.to_string())?;
            Ok(data.result)
        }

        pub fn fetch_current_exchange_rate_sync(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
        todo!()
        }

        // example: {rates":{"EUR":0.92}}
        #[derive(Deserialize)]
        struct HistoricalResponse {
        rates: std::collections::HashMap<String, f64>,
        }

        pub async fn fetch_past_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit, datetime: NaiveDateTime) -> Result<f64, String> {
            let date_str = datetime.format("%Y-%m-%d").to_string();

            let url = format!(
                "https://api.exchangerate.host/{}?base={}&symbols={}",
                date_str,
                current_unit.to_string(),
                target_unit.to_string(),
            );

            let resp = reqwest::get(&url)
                .await
                .map_err(|e| e.to_string())?;

            // Save to postgres and/or redis
            todo!();

            let data: HistoricalResponse = resp.json().await.map_err(|e| e.to_string())?;
            data.rates
                .get(&target_unit.to_string())
                .cloned()
                .ok_or_else(|| "Rate not found".to_string())
        }

        pub fn fetch_past_exchange_rate_sync(current_unit: CurrencyUnit, target_unit: CurrencyUnit, datetime: NaiveDateTime) -> Result<f64, String> {
            todo!()
        }
   };
}

use currency_macro::include_currencies_from_json;
include_currencies_from_json!("data/currencies.json");
