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
macro_rules! define_currency_units {
    (
        $(
            $variant:ident => {
                symbol: $symbol:expr,
                code: $code:expr,
                unit_type: $unit_type:expr,
                unit_type_plural: $unit_type_plural:expr,
                symbol_lc: $symbol_lc:expr,
                code_lc: $code_lc:expr,
                unit_type_lc: $unit_type_lc:expr,
                unit_type_plural_lc: $unit_type_plural_lc:expr
            }
        ),+ $(,)?
    ) => {
        use sqlx::{Pool, Sqlite};
        use chrono::{NaiveDate, Duration};
        use std::{
            fmt,
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};
        use utils::{
            cache::CacheService,
        };

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum CurrencyUnit {
            $($variant),+
        }

        impl CurrencyUnit {
            pub fn get_enumerations() -> &'static [CurrencyUnit] {
                &[$(CurrencyUnit::$variant),+]
            }

            pub fn get_symbol(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $symbol),+
                }
            }

            pub fn get_code(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $code),+
                }
            }

            pub fn get_unit_type(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $unit_type),+
                }
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                match self {
                    $(CurrencyUnit::$variant => $unit_type_plural),+
                }
            }

            pub async fn to_usd_now_async(&self) -> Result<f64, String> {
                if self == &CurrencyUnit::USD {
                    return Ok(1f64)
                }

                let url = format!(
                    "https://api.frankfurter.app/latest?amount=1&from={}&to={}",
                    self.to_string(),
                    CurrencyUnit::USD.get_code(),
                );
                let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
                let data: ExchangeResponse = resp.json().await.map_err(|e| e.to_string())?;
                Ok(data.rates.usd)
            }

            pub fn to_usd_now_sync(&self) -> Result<f64, String> {
                if self == &CurrencyUnit::USD {
                    return Ok(1f64)
                }

                let url = format!(
                    "https://api.frankfurter.app/latest?amount=1&from={}&to={}",
                    self.to_string(),
                    CurrencyUnit::USD.get_code(),
                );
                let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
                let data: ExchangeResponse = resp.json().map_err(|e| e.to_string())?;
                Ok(data.rates.usd)
            }

            pub async fn to_usd_at_time_async(&self, date: NaiveDate) -> Result<f64, String> {
                if self == &CurrencyUnit::USD {
                    return Ok(1f64)
                }

                let date_str = date.format("%Y-%m-%d").to_string();
                let url = format!(
                    "https://api.frankfurter.app/{}?from={}&to={}",
                    date_str,
                    self.to_string(),
                    CurrencyUnit::USD.to_string(),
                );

                let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
                let data: ExchangeResponse = resp.json().await.map_err(|e| e.to_string())?;
                Ok(data.rates.usd)
            }


            pub fn to_usd_at_time_sync(&self, date: NaiveDate) -> Result<f64, String> {
                if self == &CurrencyUnit::USD {
                    return Ok(1f64)
                }

                let date_str = date.format("%Y-%m-%d").to_string();
                let url = format!(
                    "https://api.frankfurter.app/{}?from={}&to={}",
                    date_str,
                    self.to_string(),
                    CurrencyUnit::USD.to_string(),
                );

                let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
                let data: ExchangeResponse = resp.json().map_err(|e| e.to_string())?;
                Ok(data.rates.usd)
            }

            pub async fn get_current_exchange_rate_async(&self, target_unit: &CurrencyUnit) -> Result<f64, String> {
                if self == target_unit {
                    return Ok(1f64)
                }

                let cache_service = CacheService::new().map_err(|e| e.to_string())?;
                let duration_in_seconds: Option<u64> = Some(Duration::days(1).num_seconds() as u64);

                let current_key = format!("exchange_rate: {} to {}", self, CurrencyUnit::USD);
                let current_value: f64 = match self {
                    CurrencyUnit::USD => 1f64,
                    _ => {
                        match cache_service.get_value(current_key.as_str()) {
                            Ok(Some(current_value)) => current_value,
                            Ok(None) => {
                                let current_value: f64 = match self.to_usd_now_async().await {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
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
                                let target_value: f64 = match target_unit.to_usd_now_async().await {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                                target_value
                            }
                            Err(err) => return Err(err.to_string()),
                        }
                    }
                };

                Ok(current_value / target_value)
            }

            pub fn get_current_exchange_rate_sync(&self, target_unit: &CurrencyUnit) -> Result<f64, String> {
                if self == target_unit {
                    return Ok(1f64)
                }

                let cache_service = CacheService::new().map_err(|e| e.to_string())?;
                let duration_in_seconds: Option<u64> = Some(Duration::days(1).num_seconds() as u64);

                let current_key = format!("exchange_rate: {} to {}", self, CurrencyUnit::USD);
                let current_value: f64 = match self {
                    CurrencyUnit::USD => 1f64,
                    _ => {
                        match cache_service.get_value(current_key.as_str()) {
                            Ok(Some(current_value)) => current_value,
                            Ok(None) => {
                                let current_value: f64 = match self.to_usd_now_sync() {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
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
                                let target_value: f64 = match target_unit.to_usd_now_sync() {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                                target_value
                            }
                            Err(err) => return Err(err.to_string()),
                        }
                    }
                };

                Ok(current_value / target_value)
            }

            pub async fn get_past_exchange_rate_async(&self, target_unit: &CurrencyUnit, datetime: NaiveDate) -> Result<f64, String> {
                if self == target_unit {
                    return Ok(1f64)
                }

                let cache_service = CacheService::new().map_err(|e| e.to_string())?;
                let duration_in_seconds: Option<u64> = Some(Duration::days(1).num_seconds() as u64);
                let date_str = datetime.format("%Y-%m-%d").to_string();

                let current_key = format!("historical_exchange_rate: {} to {} at {}", self, CurrencyUnit::USD, date_str);
                let current_value: f64 = match self {
                    CurrencyUnit::USD => 1f64,
                    _ => {
                        match cache_service.get_value(current_key.as_str()) {
                            Ok(Some(current_value)) => current_value,
                            Ok(None) => {
                                let current_value: f64 = match self.to_usd_at_time_async(datetime).await {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
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
                                let target_value: f64 = match self.to_usd_at_time_async(datetime).await {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                                target_value
                            }
                            Err(err) => return Err(err.to_string()),
                        }
                    }
                };

                Ok(current_value / target_value)
            }

            pub fn get_past_exchange_rate_sync(&self, target_unit: &CurrencyUnit, datetime: NaiveDate) -> Result<f64, String> {
                if self == target_unit {
                    return Ok(1f64)
                }

                let cache_service = CacheService::new().map_err(|e| e.to_string())?;
                let duration_in_seconds: Option<u64> = Some(Duration::days(1).num_seconds() as u64);
                let date_str = datetime.format("%Y-%m-%d").to_string();

                let current_key = format!("historical_exchange_rate: {} to {} at {}", self, CurrencyUnit::USD, date_str);
                let current_value: f64 = match self {
                    CurrencyUnit::USD => 1f64,
                    _ => {
                        match cache_service.get_value(current_key.as_str()) {
                            Ok(Some(current_value)) => current_value,
                            Ok(None) => {
                                let current_value: f64 = match self.to_usd_at_time_sync(datetime) {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(current_key.as_str(), current_value, duration_in_seconds);
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
                                let target_value: f64 = match self.to_usd_at_time_sync(datetime) {
                                    Ok(val) => val,
                                    Err(err) => return Err(err)
                                };
                                let _ = cache_service.store_key_value(target_key.as_str(), target_value, duration_in_seconds);
                                target_value
                            }
                            Err(err) => return Err(err.to_string()),
                        }
                    }
                };

                Ok(current_value / target_value)
            }

            pub async fn save_enumerations_to_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
                let currency_enumerations = CurrencyUnit::get_enumerations();
                for currency in currency_enumerations {
                    let unit_type = currency.get_unit_type();
                    sqlx::query!(
                        r#"
                            INSERT OR IGNORE INTO units_currency_types (unit_type)
                            VALUES (?)
                        "#,
                        unit_type,
                    )
                    .execute(pool)
                    .await?;
                }
                return Ok(())
            }

            pub async fn get_database_id(&self, pool: &Pool<Sqlite>) -> Result<i64, sqlx::Error> {
                let unit_type = self.get_unit_type();
                let row = sqlx::query!(
                    r#"
                        SELECT id 
                        FROM units_currency_types
                        WHERE unit_type = ?
                    "#,
                    unit_type
                )
                .fetch_one(pool)
                .await?;
                Ok(row.id)
            }

            pub async fn from_database_id(id: i64, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
                let row = sqlx::query!(
                    r#"
                        SELECT unit_type
                        FROM units_currency_types
                        WHERE id = ?
                    "#,
                    id
                )
                .fetch_one(pool)
                .await?;

                // FIX THIS
                Ok(Self::from_str(&row.unit_type).unwrap())
            }
        }

        impl FromStr for CurrencyUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();

                match formatted_string.as_str() {
                    $($symbol_lc | $code_lc | $unit_type_lc => return Ok(CurrencyUnit::$variant),)+
                    _ => match formatted_string.as_str() {
                        $($unit_type_plural_lc => return Ok(CurrencyUnit::$variant),)+
                        _ => Err("Unknown currency unit"),
                    }
                }
            }
        }
    }
}

impl fmt::Display for CurrencyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_code())
    }
}

use units_macro::include_currency_units_from_json;
include_currency_units_from_json!("data/units/currency");
