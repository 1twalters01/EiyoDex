#[macro_export]
macro_rules! define_distances {
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
            distance::unit::DistanceUnit,
            measurement_system::MeasurementSystem,
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

        #[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
        pub struct DistanceQuantity {
            value: f64,
            unit: DistanceUnit,
        }

        impl DistanceQuantity {
            pub fn new(value: f64, unit: DistanceUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $from_fn_name(value: f64) -> Self {
                    Self::new(value, DistanceUnit::$variant)
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

            pub fn to_unit(&self, unit: DistanceUnit) -> Self {
                let value = match unit {
                    $(DistanceUnit::$variant => self.$as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $to_fn_name(&self) -> Self {
                    self.to_unit(DistanceUnit::$variant)
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

            pub fn get_unit(&self) -> DistanceUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: DistanceUnit) {
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

impl SaveToDatabase<DistanceQuantity> for DistanceQuantity {
    async fn save_to_database(
        &self,
        id: Id<DistanceQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Vec<u8>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let distance_type_id = self.get_unit().get_database_id(pool).await.unwrap();
        let value = self.get_value();
        let row = sqlx::query!(
            r#"
                INSERT INTO units_distance_quantities (id, distance_type_id, value)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    distance_type_id = excluded.distance_type_id,
                    value = excluded.value
                RETURNING id
            "#,
            uuid,
            distance_type_id,
            value,
        )
        .fetch_one(pool)
        .await?;

        return Ok(row.id);
    }
}

impl GetFromDatabaseUsingId<DistanceQuantity> for DistanceQuantity {
    async fn get_from_database_using_id(
        id: Id<DistanceQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Entity<Self>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let row = sqlx::query!(
            r#"
                SELECT 
                    dq.id, 
                    dt.unit_type,
                    dq.value
                FROM units_distance_quantities dq
                INNER JOIN units_distance_types dt
                    ON dq.distance_type_id = dt.id
                WHERE dq.id = ?
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        let unit = DistanceUnit::from_str(&row.unit_type).unwrap();
        let value = row.value;

        let inner = Self { unit, value };
        let new_uuid = Uuid::from_slice(&row.id.to_vec()).unwrap();
        let id = Id::from_inner(InnerId::Uuid(new_uuid));
        let distance_record = Entity::new_with_id(id, inner);
        Ok(distance_record)
    }
}

impl DeleteFromDatabaseUsingId<DistanceQuantity> for DistanceQuantity {
    async fn delete_from_database_using_id(
        id: Id<DistanceQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        sqlx::query!("DELETE FROM units_distance_quantities WHERE id = ?", uuid)
            .execute(pool)
            .await?;

        return Ok(());
    }
}

impl fmt::Display for DistanceQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for DistanceQuantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for DistanceQuantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl<T> Mul<T> for DistanceQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl<T> Div<T> for DistanceQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Sum for DistanceQuantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DistanceQuantity::new(0f64, DistanceUnit::Meter), |a, b| {
            b + a
        })
    }
}

impl PartialOrd for DistanceQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use units_macro::include_distances_from_json;

use identity::{
    entity::{DeleteFromDatabaseUsingId, Entity, GetFromDatabaseUsingId, SaveToDatabase},
    Id,
    InnerId
};

include_distances_from_json!("data/units/distance",);
