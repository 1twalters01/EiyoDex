#[macro_export]
macro_rules! define_duration_units {
    (
        $(
            $variant:ident => {
                chrono_name: $chrono_name: ident,
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
            duration::error::DurationUnitParseError,
            measurement_system::MeasurementSystem,
        };
        use std::str::FromStr;
        use serde::{Deserialize, Serialize};
        use utils::database::DatabaseService;

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum DurationUnit {
            $($variant),+
        }

        impl DurationUnit {
            pub fn get_enumerations() -> &'static [Self] {
                &[$(DurationUnit::$variant),+]
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(DurationUnit::$variant => $symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(DurationUnit::$variant => $unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(DurationUnit::$variant => $unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> MeasurementSystem {
                match self {
                    $(DurationUnit::$variant => MeasurementSystem::$measurement_system),+
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(DurationUnit::$variant => $si_factor),+
                }
            }

            pub async fn save_to_database() -> Result<(), sqlx::Error> {
                let database_service = DatabaseService::new().await?;
                let duration_enumerations = DurationUnit::get_enumerations();
                for duration in duration_enumerations {
                    let unit_type = duration.as_unit_type();
                    sqlx::query!(
                        r#"
                            INSERT OR IGNORE INTO units_duration_types (unit_type)
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
                        FROM units_duration_types
                        WHERE unit_type = ?
                    "#,
                    unit_type
                )
                .fetch_one(&database_service.pool)
                .await?;
                Ok(row.id)
            }
        }

        impl FromStr for DurationUnit {
            type Err = DurationUnitParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($symbol_lc | $unit_type_plural_lc => return Ok(DurationUnit::$variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($unit_type_lc => Ok(DurationUnit::$variant),)+
                            _ => {
                                match formatted_string.as_str() {
                                    $($identifier_lc => Ok(DurationUnit::$variant),)+
                                    err => Err(DurationUnitParseError::UnknownUnit { input: err.to_string() }),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use units_macro::include_duration_units_from_json;
include_duration_units_from_json!("data/units/duration");
