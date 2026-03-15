#[macro_export]
macro_rules! define_energy_units {
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
        use sqlx::{Pool, Sqlite};
        use crate::{
            energy::error::EnergyUnitParseError,
            measurement_system::MeasurementSystem,
        };
        use std::str::FromStr;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum EnergyUnit {
            $($variant),+
        }

        impl EnergyUnit {
            pub fn get_enumerations() -> &'static [EnergyUnit] {
                &[$(EnergyUnit::$variant),+]
            }

            pub fn get_symbol(&self) -> &'static str {
                match self {
                    $(EnergyUnit::$variant => $symbol),+
                }
            }

            pub fn get_unit_type(&self) -> &'static str {
                match self {
                    $(EnergyUnit::$variant => $unit_type),+
                }
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                match self {
                    $(EnergyUnit::$variant => $unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> MeasurementSystem {
                match self {
                    $(EnergyUnit::$variant => MeasurementSystem::$measurement_system),+
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(EnergyUnit::$variant => $si_factor),+
                }
            }

            pub async fn save_enumerations_to_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
                let energy_enumerations = EnergyUnit::get_enumerations();
                for energy in energy_enumerations {
                    let unit_type = energy.get_unit_type();
                    sqlx::query!(
                        r#"
                            INSERT OR IGNORE INTO units_energy_types (unit_type)
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
                        FROM units_energy_types
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
                    FROM units_energy_types
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

        impl FromStr for EnergyUnit {
            type Err = EnergyUnitParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();

                // Warning shows if just one of the user defined types repeats
                #[allow(unreachable_patterns)]
                match formatted_string.as_str() {
                    $($symbol_lc | $unit_type_lc | $unit_type_plural_lc | $identifier_lc => return Ok(EnergyUnit::$variant),)+
                    err => Err(EnergyUnitParseError::UnknownUnit { input: err.to_string() }),
                }
            }
        }
    }
}

use units_macro::include_energy_units_from_json;
include_energy_units_from_json!("data/units/energy");
