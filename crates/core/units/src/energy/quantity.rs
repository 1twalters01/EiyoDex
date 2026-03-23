// #[allow(unreachable_patterns)]

#[macro_export]
macro_rules! define_energies {
    (
        $(
            $variant:ident => {
                from_fn_name: $from_fn_name:ident,
                as_fn_name: $as_fn_name:ident,
                to_fn_name: $to_fn_name:ident,
                si_factor: $si_factor:expr
            }
        ),+ $(,)?
    ) => {
        use crate::{
            energy::unit::EnergyUnit,
            measurement_system::MeasurementSystem,
            into_f64::IntoF64Safe,
        };
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            iter::Sum,
            str::FromStr,
        };
        use sqlx::{Pool, Sqlite};
        use serde::{Deserialize, Serialize};
        use uuid::Uuid;

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct EnergyQuantity {
            value: f64,
            unit: EnergyUnit,
        }

        impl EnergyQuantity {
            pub fn new(value: f64, unit: EnergyUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $from_fn_name(value: f64) -> Self {
                    Self::new(value, EnergyUnit::$variant)
                }
            )+

            pub fn round(&mut self, dp: u8) -> Self {
                let factor = 10f64.powi(dp as i32);
                self.value = (self.value * factor).round()/factor;
                return *self
            }

            $(
                pub fn $as_fn_name(&self) -> f64 {
                    self.value * self.unit.si_factor() / $si_factor
                }
            )+

            pub fn to_unit(&self, unit: EnergyUnit) -> Self {
                let value = match unit {
                    $(EnergyUnit::$variant => self.$as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $to_fn_name(&self) -> Self {
                    self.to_unit(EnergyUnit::$variant)
                }
            )+

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

            pub fn get_unit(&self) -> EnergyUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: EnergyUnit) {
                self.unit = unit;
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.get_symbol()
            }

            pub fn get_measurement_system(&self) -> MeasurementSystem {
                self.unit.get_measurement_system()
            }

            pub fn get_unit_type(&self) -> &'static str {
                self.unit.get_unit_type()
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                self.unit.get_unit_type_plural()
            }

            pub fn to_string(&self) -> String {
                format!("{}{}", self.value.to_string().trim(), self.get_symbol().trim())
            }
        }
    };
}

impl SaveToDatabase<EnergyQuantity> for EnergyQuantity {
    async fn save_to_database(
        &self,
        id: Id<EnergyQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let energy_type_id = self.get_unit().get_database_id(pool).await.unwrap();
        let value = self.get_value();
        sqlx::query!(
            r#"
                INSERT INTO units_energy_quantities (id, energy_type_id, value)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    energy_type_id = excluded.energy_type_id,
                    value = excluded.value
            "#,
            uuid,
            energy_type_id,
            value,
        )
        .execute(pool)
        .await?;

        return Ok(());
    }
}

impl GetFromDatabaseUsingId<EnergyQuantity> for EnergyQuantity {
    async fn get_from_database_using_id(
        id: Id<EnergyQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Record<Self>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let row = sqlx::query!(
            r#"
                SELECT 
                    mq.id, 
                    mt.unit_type,
                    mq.value
                FROM units_energy_quantities mq
                INNER JOIN units_energy_types mt
                    ON mq.energy_type_id = mt.id
                WHERE mq.id = ?
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        let unit = EnergyUnit::from_str(&row.unit_type).unwrap();
        let value = row.value;

        let inner = Self { unit, value };
        let new_uuid = Uuid::from_slice(&row.id.to_vec()).unwrap();
        let id = Id::from_inner(InnerId::Uuid(new_uuid));
        let energy_record = Record::new_with_id(id, inner);
        Ok(energy_record)
    }
}

impl DeleteFromDatabaseUsingId<EnergyQuantity> for EnergyQuantity {
    async fn delete_from_database_using_id(
        id: Id<EnergyQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        sqlx::query!("DELETE FROM units_energy_quantities WHERE id = ?", uuid,)
            .execute(pool)
            .await?;

        return Ok(());
    }
}

impl fmt::Display for EnergyQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for EnergyQuantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for EnergyQuantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl<T> Mul<T> for EnergyQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl<T> Div<T> for EnergyQuantity
where
    T: Into<f64> + IntoF64Safe + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        println!("yoooo");
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Sum for EnergyQuantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            EnergyQuantity::new(0f64, EnergyUnit::Kilocalorie),
            |a, b| b + a,
        )
    }
}

impl PartialOrd for EnergyQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use units_macro::include_energies_from_json;

use crate::record::{
    DeleteFromDatabaseUsingId, GetFromDatabaseUsingId, Record, SaveToDatabase,
};

use identity::{Id, InnerId};

include_energies_from_json!("data/units/energy");
