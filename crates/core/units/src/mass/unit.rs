#[macro_export]
macro_rules! define_mass_units {
    (
        $(
            $variant:ident => {
                measurement_system: $measurement_system:ident,
                symbol: $symbol:expr,
                symbol_lc: $symbol_lc:expr,
                unit_type: $unit_type:expr,
                unit_type_lc: $unit_type_lc:expr,
                unit_type_plural: $unit_type_plural:expr,
                unit_type_plural_lc: $unit_type_plural_lc:expr,
                identifier_lc: $identifier_lc:expr,
                si_factor: $si_factor:expr
            }
        ),+ $(,)?
    ) => {
        use crate::{
            mass::error::MassUnitParseError,
            measurement_system::MeasurementSystem,
        };
        use utils::database::DatabaseService;
        use std::{
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum MassUnit {
            $($variant),+
        }

        impl MassUnit {
            pub fn get_enumerations() -> &'static [Self] {
                &[$(MassUnit::$variant),+]
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(MassUnit::$variant => $symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(MassUnit::$variant => $unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(MassUnit::$variant => $unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> MeasurementSystem {
                match self {
                    $(MassUnit::$variant => MeasurementSystem::$measurement_system),+
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(MassUnit::$variant => $si_factor),+
                }
            }

            pub async fn save_to_database() -> Result<(), sqlx::Error> {
                let database_service = DatabaseService::new().await?;
                let mass_enumerations = MassUnit::get_enumerations();
                for mass in mass_enumerations {
                    let unit_type = mass.as_unit_type();
                    sqlx::query!(
                        r#"
                            INSERT OR IGNORE INTO units_mass_types (unit_type)
                            VALUES (?)
                        "#,
                        unit_type,
                    )
                    .execute(&database_service.pool)
                    .await?;
                }
                return Ok(())
            }

            pub async fn get_database_id(&self) -> Result<i64, sqlx::Error> {
                let database_service = DatabaseService::new().await?;
                let unit_type = self.as_unit_type();
                let row = sqlx::query!(
                    r#"
                        SELECT id 
                        FROM units_mass_types
                        WHERE unit_type = ?
                    "#,
                    unit_type
                )
                .fetch_one(&database_service.pool)
                .await?;
                Ok(row.id)
            }

            pub async fn from_database_id(id: i64) -> Result<Self, sqlx::Error> {
                let database_service = DatabaseService::new().await?;
                let row = sqlx::query!(
                    r#"
                        SELECT unit_type
                        FROM units_mass_types
                        WHERE id = ?
                    "#,
                    id
                )
                .fetch_one(&database_service.pool)
                .await?;

                // FIX THIS
                Ok(Self::from_str(&row.unit_type).unwrap())
            }
        }

        impl FromStr for MassUnit {
            type Err = MassUnitParseError;

            fn from_str(s: &str) -> Result<Self, MassUnitParseError> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($symbol_lc | $unit_type_lc | $unit_type_plural_lc => return Ok(MassUnit::$variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($identifier_lc => Ok(MassUnit::$variant),)+
                            err => Err(MassUnitParseError::UnknownUnit { input: err.to_string() }),
                        }
                    }
                }
            }
        }
    }
}

use units_macro::include_mass_units_from_json;
include_mass_units_from_json!("data/units/mass");
