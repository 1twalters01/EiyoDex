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
        use crate::currency::unit::CurrencyUnit;
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct CurrencyQuantity {
            value: f64,
            unit: CurrencyUnit,
        }

        impl CurrencyQuantity {
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
                self.unit.get_symbol()
            }

            pub fn get_unit_type(&self) -> &'static str {
                self.unit.get_unit_type()
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                self.unit.get_unit_type_plural()
            }

            pub fn get_code(&self) -> &'static str {
                self.unit.get_code()
            }

            pub async fn convert_to_async(&self, target_unit: CurrencyUnit) -> Result<CurrencyQuantity, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_current_exchange_rate_async(&target_unit).await {
                        Ok(rate) => Ok(CurrencyQuantity::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub fn convert_to_sync(&self, target_unit: CurrencyUnit) -> Result<CurrencyQuantity, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_current_exchange_rate_sync(&target_unit) {
                        Ok(rate) => Ok(CurrencyQuantity::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub async fn convert_to_historic_async(&self, target_unit: CurrencyUnit, date: NaiveDate) -> Result<CurrencyQuantity, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_past_exchange_rate_async(&target_unit, date).await {
                        Ok(rate) => Ok(CurrencyQuantity::new(self.value * rate, target_unit)),
                        Err(err) => Err(err),
                    }
                }
            }

            pub fn convert_to_historic_sync(&self, target_unit: CurrencyUnit, date: NaiveDate) -> Result<CurrencyQuantity, String> {
                if self.unit == target_unit {
                    Ok(*self)
                } else {
                    match self.unit.get_past_exchange_rate_sync(&target_unit, date) {
                        Ok(rate) => Ok(CurrencyQuantity::new(self.value * rate, target_unit)),
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
                pub fn $to_fn_name(&self) -> Result<CurrencyQuantity, String> {
                    self.convert_to_sync(CurrencyUnit::$variant)
                }
            )+
        }
   };
}

impl SaveToDatabase<CurrencyQuantity> for CurrencyQuantity {
    async fn save_to_database(
        &self,
        id: Id<CurrencyQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_uuid();
        let currency_type_id = self.get_unit().get_database_id(pool).await.unwrap();
        let value = self.get_value();
        sqlx::query!(
            r#"
                INSERT INTO units_currency_quantities (id, currency_type_id, value)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    currency_type_id = excluded.currency_type_id,
                    value = excluded.value
            "#,
            uuid,
            currency_type_id,
            value,
        )
        .execute(pool)
        .await?;

        return Ok(());
    }
}

impl GetFromDatabaseUsingId<CurrencyQuantity> for CurrencyQuantity {
    async fn get_from_database_using_id(
        id: Id<CurrencyQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Record<Self>, sqlx::Error> {
        let uuid = id.get_uuid();
        let row = sqlx::query!(
            r#"
                SELECT 
                    cq.id, 
                    ct.unit_type,
                    cq.value
                FROM units_currency_quantities cq
                INNER JOIN units_currency_types ct
                    ON cq.currency_type_id = ct.id
                WHERE cq.id = ?
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        println!(
            "row.unit_type: {}, unit_type: {}",
            row.unit_type,
            CurrencyUnit::GBP.get_unit_type()
        );
        // assert!(&row.unit_type, CurrencyUnit::GBP.as_unit_type());
        let unit = CurrencyUnit::from_str(&row.unit_type).unwrap();
        let value = row.value;

        let inner = Self { unit, value };
        let new_uuid = Uuid::from_slice(&row.id.to_vec()).unwrap();
        let id = Id::from_uuid(new_uuid, inner);
        let distance_record = Record::new_with_id(id, inner);
        Ok(distance_record)
    }
}

impl DeleteFromDatabaseUsingId<CurrencyQuantity> for CurrencyQuantity {
    async fn delete_from_database_using_id(
        id: Id<CurrencyQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_uuid();
        sqlx::query!("DELETE FROM units_currency_quantities WHERE id = ?", uuid)
            .execute(pool)
            .await?;

        return Ok(());
    }
}

impl fmt::Display for CurrencyQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.get_symbol(), self.value)
    }
}

impl Add for CurrencyQuantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match rhs.convert_to_sync(self.unit) {
            Ok(converted_rhs) => Self::new(self.value + converted_rhs.value, self.unit),
            Err(_) => panic!("CurrencyQuantity conversion failed in addition"),
        }
    }
}

impl Sub for CurrencyQuantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match rhs.convert_to_sync(self.unit) {
            Ok(converted_rhs) => Self::new(self.value - converted_rhs.value, self.unit),
            Err(_) => panic!("CurrencyQuantity conversion failed in subtraction"),
        }
    }
}

impl Mul<f64> for CurrencyQuantity {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.value * rhs, self.unit)
    }
}

impl Div<f64> for CurrencyQuantity {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.value / rhs, self.unit)
    }
}

impl PartialOrd for CurrencyQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.convert_to_sync(self.unit) {
            Ok(converted_other) => self.value.partial_cmp(&converted_other.value),
            Err(_) => None,
        }
    }
}

use sqlx::{Pool, Sqlite};
use units_macro::include_currencies_from_json;
use uuid::Uuid;

use crate::record::{
    DeleteFromDatabaseUsingId, GetFromDatabaseUsingId, Id, Record, SaveToDatabase,
};
include_currencies_from_json!("data/units/currency");
