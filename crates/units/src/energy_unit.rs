#[macro_export]
macro_rules! define_energy_units {
    (
        $(
            $variant:ident => {
                from_fn_name: $from_fn_name:ident,
                as_fn_name: $as_fn_name:ident,
                to_fn_name: $to_fn_name:ident,
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
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            iter::Sum,
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};
        use crate::into_f64::IntoF64Safe;

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum EnergyUnit {
            $($variant),+
        }

        impl EnergyUnit {
            pub fn get_enumerations() -> &'static [EnergyUnit] {
                &[$(EnergyUnit::$variant),+]
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(EnergyUnit::$variant => $symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(EnergyUnit::$variant => $unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
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
        }

        impl FromStr for EnergyUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($symbol_lc | $unit_type_lc | $unit_type_plural_lc => return Ok(EnergyUnit::$variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($identifier_lc => Ok(EnergyUnit::$variant),)+
                            _ => Err("Unknown mass unit"),
                        }
                    }
                }
            }
        }
    }
}

use units_macro::include_energy_units_from_json;
include_energy_units_from_json!("data/units/energy");
