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
        use chrono::{NaiveDateTime, Duration};
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};
        use utils::redis::CacheService;

        pub struct CurrencyMetadata {
            pub symbol: &'static str,
            pub code: &'static str,
            pub unit_type: &'static str,
            pub unit_type_plural: &'static str,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum CurrencyUnit {
            $($variant),+
        }

        impl CurrencyUnit {
            pub fn get_enumerations() -> &'static [CurrencyUnit] {
                &[$(CurrencyUnit::$variant),+]
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
                match self {
                    $(CurrencyUnit::$variant => $symbol),+
                }
            }

            pub fn as_code(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $code),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $unit_type_plural),+
                }
            }
        }

        impl FromStr for CurrencyUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_s = s.trim().to_uppercase();
                for variant in Self::get_enumerations() {
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
                    match get_current_exchange_rate(self.unit, target_unit).await {
                        Ok(rate) => Ok(Currency::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub fn convert_to_sync(&self, target_unit: CurrencyUnit) -> Result<Currency, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match get_current_exchange_rate_sync(self.unit, target_unit) {
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

        pub async fn get_current_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
            if current_unit == target_unit {
                return Ok(1f64)
            }
            let datetime = None;

            let mut cache_service = CacheService::new();
            let duration_in_seconds: Option<i64> = Some(Duration::days(1).num_seconds());

            let current_key = format!("exchange_rate: {} to {}", current_unit, CurrencyUnit::USD);
            let current_value: f64 = match current_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(current_key.as_str()) {
                        Ok(Some(current_value)) => current_value,
                        Ok(None) => {
                            let current_value: f64 = match fetch_unit_to_usd_exchange_rate(current_unit, datetime).await {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
                            current_value
                        },
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            let target_key = format!("exchange_rate: {} to {}", target_unit, CurrencyUnit::USD);
            let target_value: f64 = match target_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(target_key.as_str()) {
                        Ok(Some(target_value)) => target_value,
                        Ok(None) => {
                            let target_value: f64 = match fetch_unit_to_usd_exchange_rate(target_unit, datetime).await {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                            target_value
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            Ok(current_value / target_value)
        }

        pub fn get_current_exchange_rate_sync(current_unit: CurrencyUnit, target_unit: CurrencyUnit) -> Result<f64, String> {
            if current_unit == target_unit {
                return Ok(1f64)
            }

            let datetime = None;

            let mut cache_service = CacheService::new();
            let duration_in_seconds: Option<i64> = Some(Duration::days(1).num_seconds());

            let current_key = format!("exchange_rate: {} to {}", current_unit, CurrencyUnit::USD);
            let current_value: f64 = match current_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(current_key.as_str()) {
                        Ok(Some(current_value)) => current_value,
                        Ok(None) => {
                            let current_value: f64 = match fetch_unit_to_usd_exchange_rate_sync(current_unit, datetime) {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
                            current_value
                        },
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            let target_key = format!("exchange_rate: {} to {}", target_unit, CurrencyUnit::USD);
            let target_value: f64 = match target_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(target_key.as_str()) {
                        Ok(Some(target_value)) => target_value,
                        Ok(None) => {
                            let target_value: f64 = match fetch_unit_to_usd_exchange_rate_sync(target_unit, datetime) {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                            target_value
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            Ok(current_value / target_value)
        }

        // example: {rates":{"EUR":0.92}}
        #[derive(Deserialize)]
        struct HistoricalResponse {
            rates: std::collections::HashMap<String, f64>,
        }

        pub async fn get_past_exchange_rate(current_unit: CurrencyUnit, target_unit: CurrencyUnit, datetime: NaiveDateTime) -> Result<f64, String> {
            if current_unit == target_unit {
                return Ok(1f64)
            }

            let mut cache_service = CacheService::new();
            let duration_in_seconds: Option<i64> = Some(Duration::days(1).num_seconds());
            let date_str = datetime.format("%Y-%m-%d").to_string();

            let current_key = format!("historical_exchange_rate: {} to {} at {}", current_unit, CurrencyUnit::USD, date_str);
            let current_value: f64 = match current_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(current_key.as_str()) {
                        Ok(Some(current_value)) => current_value,
                        Ok(None) => {
                            let current_value: f64 = match fetch_unit_to_usd_exchange_rate_sync(current_unit, Some(datetime)) {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
                            current_value
                        },
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            let target_key = format!("exchange_rate: {} to {}", target_unit, CurrencyUnit::USD);
            let target_value: f64 = match target_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(target_key.as_str()) {
                        Ok(Some(target_value)) => target_value,
                        Ok(None) => {
                            let target_value: f64 = match fetch_unit_to_usd_exchange_rate(target_unit, Some(datetime)).await {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                            target_value
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            Ok(current_value / target_value)
        }

        pub fn get_past_exchange_rate_sync(current_unit: CurrencyUnit, target_unit: CurrencyUnit, datetime: NaiveDateTime) -> Result<f64, String> {
            if current_unit == target_unit {
                return Ok(1f64)
            }

            let mut cache_service = CacheService::new();
            let duration_in_seconds: Option<i64> = Some(Duration::days(1).num_seconds());
            let date_str = datetime.format("%Y-%m-%d").to_string();

            let current_key = format!("historical_exchange_rate: {} to {} at {}", current_unit, CurrencyUnit::USD, date_str);
            let current_value: f64 = match current_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(current_key.as_str()) {
                        Ok(Some(current_value)) => current_value,
                        Ok(None) => {
                            let current_value: f64 = match fetch_unit_to_usd_exchange_rate_sync(current_unit, Some(datetime)) {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
                            current_value
                        },
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            let target_key = format!("exchange_rate: {} to {}", target_unit, CurrencyUnit::USD);
            let target_value: f64 = match target_unit {
                CurrencyUnit::USD => 1f64,
                _ => {
                    match cache_service.get_value(target_key.as_str()) {
                        Ok(Some(target_value)) => target_value,
                        Ok(None) => {
                            let target_value: f64 = match fetch_unit_to_usd_exchange_rate_sync(target_unit, Some(datetime)) {
                                Ok(val) => val,
                                Err(err) => return Err(err)
                            };
                            cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                            target_value
                        }
                        Err(err) => return Err(err.to_string()),
                    }
                }
            };

            Ok(current_value / target_value)
        }
   };
}

pub async fn fetch_unit_to_usd_exchange_rate(
    unit: CurrencyUnit,
    datetime: Option<NaiveDateTime>,
) -> Result<f64, String> {
    match datetime {
        None => {
            let url = format!(
                "https://api.exchangerate.host/convert?from={}&to={}",
                unit.to_string(),
                CurrencyUnit::USD.to_string(),
            );
            let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
            let data: ExchangeResponse = resp.json().await.map_err(|e| e.to_string())?;
            Ok(data.result)
        }
        Some(datetime) => {
            let date_str = datetime.format("%Y-%m-%d").to_string();
            let url = format!(
                "https://api.exchangerate.host/{}?base={}&symbols={}",
                date_str,
                unit.to_string(),
                CurrencyUnit::USD.to_string(),
            );

            let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;

            let data: HistoricalResponse = resp.json().await.map_err(|e| e.to_string())?;
            let value = data
                .rates
                .get(&unit.to_string())
                .cloned()
                .ok_or_else(|| "Rate not found".to_string())
                .ok();

            match value {
                Some(val) => Ok(val),
                None => return Err(String::from("No rate found")),
            }
        }
    }
}

pub fn fetch_unit_to_usd_exchange_rate_sync(
    unit: CurrencyUnit,
    datetime: Option<NaiveDateTime>,
) -> Result<f64, String> {
    match datetime {
        None => {
            let url = format!(
                "https://api.exchangerate.host/convert?from={}&to={}",
                unit.to_string(),
                CurrencyUnit::USD.to_string(),
            );
            let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
            let data: ExchangeResponse = resp.json().map_err(|e| e.to_string())?;
            Ok(data.result)
        }
        Some(datetime) => {
            let date_str = datetime.format("%Y-%m-%d").to_string();
            let url = format!(
                "https://api.exchangerate.host/{}?base={}&symbols={}",
                date_str,
                unit.to_string(),
                CurrencyUnit::USD.to_string(),
            );

            let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;

            let data: HistoricalResponse = resp.json().map_err(|e| e.to_string())?;
            let value = data
                .rates
                .get(&unit.to_string())
                .cloned()
                .ok_or_else(|| "Rate not found".to_string())
                .ok();

            match value {
                Some(val) => Ok(val),
                None => return Err(String::from("No rate found")),
            }
        }
    }
}

use currency_macro::include_currencies_from_json;
include_currencies_from_json!("data/currencies.json");
