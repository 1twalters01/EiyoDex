#[macro_export]
macro_rules! define_powers {
    (
        all: {
            $(
                $all_variant:ident => {
                    from_fn_name: $all_from_fn_name:ident,
                    as_fn_name: $all_as_fn_name:ident,
                    to_fn_name: $all_to_fn_name:ident,
                    si_factor: $all_si_factor: expr
                }
            ),* $(,)?
        },
        json: {
            $(
                $json_variant:ident => {
                    from_fn_name: $json_from_fn_name: ident,
                    as_fn_name: $json_as_fn_name: ident,
                    to_fn_name: $json_to_fn_name: ident,
                    si_factor: $json_si_factor: expr
                }
            ),* $(,)?
        },
    ) => {
        use crate::{
            energy::{
                quantity::EnergyQuantity,
                unit::EnergyUnit,
            },
            duration::{
                quantity::DurationQuantity,
                unit::DurationUnit,
            },
            into_f64::IntoF64Safe,
            power::{
                unit::PowerUnit,
                measurement_system::PowerMeasurementSystem,
            },
        };
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            iter::Sum,
            str::FromStr,
        };

        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct PowerQuantity {
            value: f64,
            unit: PowerUnit,
        }

        impl PowerQuantity {
            pub fn new(value: f64, unit: PowerUnit) -> Self {
                Self { value, unit }
            }

            pub fn from_variants(value: f64, energy_unit: EnergyUnit, duration_unit: DurationUnit) -> Self {
                Self {
                    value,
                    unit: PowerUnit::from_variants(energy_unit, duration_unit),
                }
            }

            $(
                pub fn $all_from_fn_name(value: f64) -> Self {
                    Self::new(value, PowerUnit::$all_variant)
                }
            )+

            pub fn round(&mut self, dp: u8) -> Self {
                let factor = 10f64.powi(dp as i32);
                self.value = (self.value * factor).round()/factor;
                return *self
            }

            $(
                pub fn $all_as_fn_name(&self) -> f64 {
                    self.value * self.unit.si_factor() / $all_si_factor
                }
            )+

            pub fn to_unit(&self, unit: PowerUnit) -> Self {
                let value = match unit {
                    $(PowerUnit::$all_variant => self.$all_as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $all_to_fn_name(&self) -> Self {
                    self.to_unit(PowerUnit::$all_variant)
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

            pub fn get_unit(&self) -> PowerUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: PowerUnit) {
                self.unit = unit;
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.get_symbol()
            }

            pub fn get_measurement_system(&self) -> PowerMeasurementSystem {
                self.unit.get_measurement_system()
            }

            pub fn get_unit_type(&self) -> &'static str {
                self.unit.get_unit_type()
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                self.unit.get_unit_type_plural()
            }

            pub fn to_string(&self) -> String {
                format!("{}{}", self.value, self.get_symbol())
            }
        }
    }
}

impl SaveToDatabase<PowerQuantity> for PowerQuantity {
    async fn save_to_database(
        &self,
        id: Id<PowerQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Vec<u8>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let energy_type_id = self
            .get_unit()
            .get_energy_variant()
            .get_database_id(pool)
            .await
            .unwrap();
        let duration_type_id = self
            .get_unit()
            .get_duration_variant()
            .get_database_id(pool)
            .await
            .unwrap();
        let value = self.get_value();
        let row = sqlx::query!(
            r#"
                INSERT INTO units_power_quantities (id, energy_type_id, duration_type_id, value)
                VALUES (?, ?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    energy_type_id = excluded.energy_type_id,
                    duration_type_id = excluded.duration_type_id,
                    value = excluded.value
                RETURNING id
            "#,
            uuid,
            energy_type_id,
            duration_type_id,
            value,
        )
        .fetch_one(pool)
        .await?;

        return Ok(row.id);
    }
}

impl GetFromDatabaseUsingId<PowerQuantity> for PowerQuantity {
    async fn get_from_database_using_id(
        id: Id<PowerQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Entity<Self>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let row = sqlx::query!(
            r#"
                SELECT 
                    pq.id, 
                    et.unit_type as energy_unit_type,
                    dt.unit_type as duration_unit_type,
                    pq.value
                FROM units_power_quantities pq
                INNER JOIN units_energy_types et
                    ON pq.energy_type_id = et.id
                INNER JOIN units_duration_types dt
                    ON pq.duration_type_id = dt.id
                WHERE pq.id = ?
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        let energy_unit = EnergyUnit::from_str(&row.energy_unit_type).unwrap();
        let duration_unit = DurationUnit::from_str(&row.duration_unit_type).unwrap();

        let unit = PowerUnit::from_variants(energy_unit, duration_unit);
        let value = row.value;

        let inner = Self { unit, value };
        let new_uuid = Uuid::from_slice(&row.id.to_vec()).unwrap();
        let id = Id::from_inner(InnerId::Uuid(new_uuid));
        let density_record = Entity::new_with_id(id, inner);
        Ok(density_record)
    }
}

impl DeleteFromDatabaseUsingId<PowerQuantity> for PowerQuantity {
    async fn delete_from_database_using_id(
        id: Id<PowerQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        sqlx::query!("DELETE FROM units_power_quantities WHERE id = ?", uuid)
            .execute(pool)
            .await?;

        return Ok(());
    }
}

impl fmt::Display for PowerQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for PowerQuantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for PowerQuantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl<T> Div<T> for PowerQuantity
where
    T: Into<f64> + IntoF64Safe + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Div<DurationQuantity> for EnergyQuantity {
    type Output = PowerQuantity;

    fn div(self, rhs: DurationQuantity) -> PowerQuantity {
        let value = self.get_value() / rhs.get_duration();
        let energy_unit = self.get_unit();
        let duration_unit = rhs.get_unit();
        PowerQuantity::from_variants(value, energy_unit, duration_unit)
    }
}

impl<T> Mul<T> for PowerQuantity
where
    T: Into<f64> + IntoF64Safe + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl Mul<DurationQuantity> for PowerQuantity {
    type Output = EnergyQuantity;

    fn mul(self, rhs: DurationQuantity) -> EnergyQuantity {
        let power_energy_variant = self.get_unit().get_energy_variant();
        let power_duration_variant = self.get_unit().get_duration_variant();
        let duration = rhs.to_unit(power_duration_variant).get_duration();
        let power = self.get_value();
        EnergyQuantity::new(power * duration, power_energy_variant)
    }
}

impl Mul<PowerQuantity> for DurationQuantity {
    type Output = EnergyQuantity;

    fn mul(self, rhs: PowerQuantity) -> EnergyQuantity {
        let power_energy_variant = rhs.get_unit().get_energy_variant();
        let power_duration_variant = rhs.get_unit().get_duration_variant();
        let duration: f64 = self.to_unit(power_duration_variant).get_duration();
        let power = rhs.get_value();
        EnergyQuantity::new(power * duration, power_energy_variant)
    }
}

impl Sum for PowerQuantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            PowerQuantity::new(0f64, PowerUnit::KilojoulePerSecond),
            |a, b| b + a,
        )
    }
}

impl PartialOrd for PowerQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use sqlx::{Pool, Sqlite};
use units_macro::include_powers_from_json;
use uuid::Uuid;

use crate::entity::{DeleteFromDatabaseUsingId, Entity, GetFromDatabaseUsingId, SaveToDatabase};

use identity::{Id, InnerId};

include_powers_from_json!(
    EnergyUnit => "data/units/energy",
    PowerUnit => "data/units/power",
    DurationUnit => "data/units/duration",
);
