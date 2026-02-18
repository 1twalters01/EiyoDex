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
        use crate::measurement_system::MeasurementSystem;
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
        }

        impl FromStr for MassUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($symbol_lc | $unit_type_lc | $unit_type_plural_lc => return Ok(MassUnit::$variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($identifier_lc => Ok(MassUnit::$variant),)+
                            _ => Err("Unknown mass unit"),
                        }
                    }
                }
            }
        }
    }
}

use units_macro::include_mass_units_from_json;
include_mass_units_from_json!("data/units/mass");
