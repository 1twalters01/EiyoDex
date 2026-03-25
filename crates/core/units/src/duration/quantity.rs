// Exists to make power not use Energy/second and instead be Energy/Duration

#[macro_export]
macro_rules! define_durations {
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
            duration::unit::DurationUnit,
            measurement_system::MeasurementSystem,
        };
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            iter::Sum,
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};
        use chrono::Duration;

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct DurationQuantity {
            value: Duration,
            unit: DurationUnit,
        }

        impl DurationQuantity {
            pub fn new(value: f64, unit: DurationUnit) -> Self {
                match unit {
                    $(DurationUnit::$variant => {
                        let nanoseconds = value * unit.si_factor() * 1e9;
                        let approximate_duration = nanoseconds.round() as i64;
                        let duration = Duration::nanoseconds(approximate_duration);
                        return Self { value: duration, unit }
                    })+
                }
            }

            pub fn from_duration(value: Duration, unit: DurationUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $from_fn_name(value: f64) -> Self {
                    Self::new(value, DurationUnit::$variant)
                }
            )+

            $(
                pub fn $as_fn_name(&self) -> f64 {
                    self.value.as_seconds_f64() / $si_factor
                }
            )+

            pub fn to_unit(&self, unit: DurationUnit) -> Self {
                Self { value: self.value, unit }
            }

            $(
                pub fn $to_fn_name(&self) -> Self {
                    self.to_unit(DurationUnit::$variant)
                }
            )+

            pub fn get_duration(&self) -> f64 {
                let duration = self.value.as_seconds_f64() / self.unit.si_factor();
                return duration
            }

            pub fn is_zero(&self) -> bool {
                self.value.num_seconds() == 0
            }

            pub fn get_value(&self) -> Duration {
                self.value
            }

            pub fn set_value(&mut self, value: Duration) {
                self.value = value;
            }

            pub fn get_unit(&self) -> DurationUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: DurationUnit) {
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

            pub fn to_string(&self, precision: Option<usize>) -> String {
                let duration = self.get_duration();
                let unit_type_plural = self.get_unit_type_plural();
                match precision {
                    None => format!("{} {}", duration, unit_type_plural),
                    Some(precision) => {
                        let factor = 10f64.powi(precision as i32);
                        println!("just duration: {}", self.get_duration());
                        println!("after round: {}", (self.get_duration() * factor).round());
                        let rounded_duration = (self.get_duration() * factor).round() / factor;
                        format!("{:.*} {}", precision, rounded_duration, unit_type_plural)
                    },
                }
            }
        }
    };
}

impl SaveToDatabase<DurationQuantity> for DurationQuantity {
    async fn save_to_database(
        &self,
        id: Id<DurationQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let duration_type_id = self.get_unit().get_database_id(pool).await.unwrap();
        let duration = self.get_duration();
        sqlx::query!(
            r#"
                INSERT INTO units_duration_quantities (id, duration_type_id, value)
                VALUES (?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    duration_type_id = excluded.duration_type_id,
                    value = excluded.value
            "#,
            uuid,
            duration_type_id,
            duration,
        )
        .execute(pool)
        .await?;

        return Ok(());
    }
}

impl GetFromDatabaseUsingId<DurationQuantity> for DurationQuantity {
    async fn get_from_database_using_id(
        id: Id<DurationQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Entity<Self>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let row = sqlx::query!(
            r#"
                SELECT 
                    dq.id, 
                    dt.unit_type,
                    dq.value
                FROM units_duration_quantities dq
                INNER JOIN units_duration_types dt
                    ON dq.duration_type_id = dt.id
                WHERE dq.id = ?
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        let unit = DurationUnit::from_str(&row.unit_type).unwrap();
        let value = row.value;

        let inner = Self::new(value, unit);
        let new_uuid = Uuid::from_slice(&row.id.to_vec()).unwrap();
        let id = Id::from_inner(InnerId::Uuid(new_uuid));
        let distance_record = Entity::new_with_id(id, inner);
        Ok(distance_record)
    }
}

impl DeleteFromDatabaseUsingId<DurationQuantity> for DurationQuantity {
    async fn delete_from_database_using_id(
        id: Id<DurationQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        sqlx::query!("DELETE FROM units_duration_quantities WHERE id = ?", uuid)
            .execute(pool)
            .await?;

        return Ok(());
    }
}

impl fmt::Display for DurationQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = Some(2);
        write!(f, "{}", self.to_string(precision))
    }
}

impl Add for DurationQuantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_duration(self.get_value() + rhs.get_value(), self.unit)
    }
}

impl Sub for DurationQuantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_duration(self.get_value() - rhs.get_value(), self.unit)
    }
}

impl<T> Mul<T> for DurationQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_duration() * rhs.into(), self.unit)
    }
}

impl<T> Div<T> for DurationQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_duration() / rhs.into(), self.unit)
    }
}

impl Sum for DurationQuantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DurationQuantity::new(0f64, DurationUnit::Second), |a, b| {
            b + a
        })
    }
}

impl PartialOrd for DurationQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use sqlx::{Pool, Sqlite};
use units_macro::include_durations_from_json;
use uuid::Uuid;

use crate::entity::{
    DeleteFromDatabaseUsingId, GetFromDatabaseUsingId, Entity, SaveToDatabase,
};

use identity::{Id, InnerId};

include_durations_from_json!("data/units/duration");
