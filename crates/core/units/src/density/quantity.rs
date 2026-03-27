#[macro_export]
macro_rules! define_densities {
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
            density::{
                unit::DensityUnit,
                measurement_system::DensityMeasurementSystem,
            },
            mass::quantity::MassQuantity,
            mass::unit::MassUnit,
            volume::quantity::VolumeQuantity,
            volume::unit::VolumeUnit,
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
        pub struct DensityQuantity {
            value: f64,
            unit: DensityUnit,
        }

        impl DensityQuantity {
            pub fn from_variants(value: f64, mass_unit: MassUnit, volume_unit: VolumeUnit) -> Self {
                Self {
                    value,
                    unit: DensityUnit::from_variants(mass_unit, volume_unit),
                }
            }

            pub fn new(value: f64, unit: DensityUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $all_from_fn_name(value: f64) -> Self {
                    Self::new(value, DensityUnit::$all_variant)
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

            pub fn to_unit(&self, unit: DensityUnit) -> Self {
                let value = match unit {
                    $(DensityUnit::$all_variant => self.$all_as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $all_to_fn_name(&self) -> Self {
                    self.to_unit(DensityUnit::$all_variant)
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

            pub fn get_unit(&self) -> DensityUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: DensityUnit) {
                self.unit = unit;
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.get_symbol()
            }

            pub fn get_measurement_system(&self) -> DensityMeasurementSystem {
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

impl SaveToDatabase<DensityQuantity> for DensityQuantity {
    async fn save_to_database(
        &self,
        id: Id<DensityQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let mass_type_id = self
            .get_unit()
            .get_mass_variant()
            .get_database_id(pool)
            .await
            .unwrap();
        let volume_type_id = self
            .get_unit()
            .get_volume_variant()
            .get_database_id(pool)
            .await
            .unwrap();
        let value = self.get_value();
        sqlx::query!(
            r#"
                INSERT INTO units_density_quantities (id, mass_type_id, volume_type_id, value)
                VALUES (?, ?, ?, ?)
                ON CONFLICT (id) DO UPDATE SET
                    mass_type_id = excluded.mass_type_id,
                    volume_type_id = excluded.volume_type_id,
                    value = excluded.value
            "#,
            uuid,
            mass_type_id,
            volume_type_id,
            value,
        )
        .execute(pool)
        .await?;

        return Ok(());
    }
}

impl GetFromDatabaseUsingId<DensityQuantity> for DensityQuantity {
    async fn get_from_database_using_id(
        id: Id<DensityQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<Entity<Self>, sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        let row = sqlx::query!(
            r#"
                SELECT 
                    dq.id, 
                    mt.unit_type as mass_unit_type,
                    vt.unit_type as volume_unit_type,
                    dq.value
                FROM units_density_quantities dq
                INNER JOIN units_mass_types mt
                    ON dq.mass_type_id = mt.id
                INNER JOIN units_volume_types vt
                    ON dq.volume_type_id = vt.id
                WHERE dq.id = ?
            "#,
            uuid
        )
        .fetch_one(pool)
        .await?;

        let mass_unit = MassUnit::from_str(&row.mass_unit_type).unwrap();
        let volume_unit = VolumeUnit::from_str(&row.volume_unit_type).unwrap();

        let unit = DensityUnit::from_variants(mass_unit, volume_unit);
        let value = row.value;

        let inner = Self { unit, value };
        let new_uuid = Uuid::from_slice(&row.id.to_vec()).unwrap();
        let id = Id::from_inner(InnerId::Uuid(new_uuid));
        let density_record = Entity::new_with_id(id, inner);
        Ok(density_record)
    }
}

impl DeleteFromDatabaseUsingId<DensityQuantity> for DensityQuantity {
    async fn delete_from_database_using_id(
        id: Id<DensityQuantity>,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = id.get_inner().to_bytes().to_vec();
        sqlx::query!("DELETE FROM units_density_quantities WHERE id = ?", uuid)
            .execute(pool)
            .await?;

        return Ok(());
    }
}

impl fmt::Display for DensityQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for DensityQuantity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for DensityQuantity {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl<T> Mul<T> for DensityQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl Mul<VolumeQuantity> for DensityQuantity {
    type Output = MassQuantity;

    fn mul(self, rhs: VolumeQuantity) -> MassQuantity {
        let density_volume_variant = self.get_unit().get_volume_variant();
        let density_mass_variant = self.get_unit().get_mass_variant();
        let volume = rhs.to_unit(density_volume_variant).get_value();
        let density = self.get_value();
        MassQuantity::new(density * volume, density_mass_variant)
    }
}

impl Mul<DensityQuantity> for VolumeQuantity {
    type Output = MassQuantity;

    fn mul(self, rhs: DensityQuantity) -> MassQuantity {
        let density_volume_variant = rhs.get_unit().get_volume_variant();
        let density_mass_variant = rhs.get_unit().get_mass_variant();
        let volume = self.to_unit(density_volume_variant).get_value();
        let density = rhs.get_value();
        MassQuantity::new(density * volume, density_mass_variant)
    }
}

impl<T> Div<T> for DensityQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Div<VolumeQuantity> for MassQuantity {
    type Output = DensityQuantity;

    fn div(self, rhs: VolumeQuantity) -> DensityQuantity {
        let value = self.get_value() / rhs.get_value();
        let mass_unit = self.get_unit();
        let volume_unit = rhs.get_unit();
        DensityQuantity::from_variants(value, mass_unit, volume_unit)
    }
}

impl Sum for DensityQuantity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(
            DensityQuantity::new(0f64, DensityUnit::KilogramPerLiter),
            |a, b| b + a,
        )
    }
}

impl PartialOrd for DensityQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use sqlx::{Pool, Sqlite};
use units_macro::include_densities_from_json;
use uuid::Uuid;

use crate::entity::{DeleteFromDatabaseUsingId, Entity, GetFromDatabaseUsingId, SaveToDatabase};

use identity::{Id, InnerId};

include_densities_from_json!(
    DensityUnit => "data/units/density",
    MassUnit => "data/units/mass",
    VolumeUnit => "data/units/volume"
);
