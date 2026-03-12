#[macro_export]
macro_rules! define_power_units {
    (
        all: {
            $(
                $all_variant:ident => {
                    energy_unit_variant: $all_energy_unit_variant: ident,
                    duration_unit_variant: $all_duration_unit_variant:ident,
                    energy_measurement_system: $all_energy_measurement_system: ident,
                    duration_measurement_system: $all_duration_measurement_system: ident,
                    symbol: $all_symbol: expr,
                    symbol_lc: $all_symbol_lc: expr,
                    unit_type: $all_unit_type: expr,
                    unit_type_lc: $all_unit_type_lc: expr,
                    unit_type_plural: $all_unit_type_plural: expr,
                    unit_type_plural_lc: $all_unit_type_plural_lc: expr,
                    identifier_lc: $all_identifier_lc: expr,
                    si_factor: $all_si_factor: expr
                }
            ),* $(,)?
        },
        json: {
            $(
                $json_variant:ident => {
                    energy_unit_variant: $json_energy_unit_variant: ident,
                    duration_unit_variant: $json_duration_unit_variant:ident,
                    energy_measurement_system: $json_energy_measurement_system: ident,
                    duration_measurement_system: $json_duration_measurement_system: ident,
                    symbol: $json_symbol: expr,
                    symbol_lc: $json_symbol_lc: expr,
                    unit_type: $json_unit_type: expr,
                    unit_type_lc: $json_unit_type_lc: expr,
                    unit_type_plural: $json_unit_type_plural: expr,
                    unit_type_plural_lc: $json_unit_type_plural_lc: expr,
                    identifier_lc: $json_identifier_lc: expr,
                    si_factor: $json_si_factor: expr
                }
            ),* $(,)?
        },
    ) => {
        use crate::{
            measurement_system::MeasurementSystem,
            energy::unit::EnergyUnit,
            duration::unit::DurationUnit,
            power::{
                error::PowerUnitParseError,
                measurement_system::PowerMeasurementSystem,
            },
        };
        use std::str::FromStr;

        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum PowerUnit {
            $($all_variant),+
        }

        impl PowerUnit {
            pub fn from_variants(energy_unit: EnergyUnit, duration_unit: DurationUnit) -> PowerUnit {
                match (energy_unit, duration_unit) {
                    $((EnergyUnit::$all_energy_unit_variant, DurationUnit::$all_duration_unit_variant) => PowerUnit::$all_variant,)+
                }
            }

            pub fn get_all_enumerations() -> &'static [Self] {
                &[$(PowerUnit::$all_variant),+]
            }

            pub fn get_selected_enumerations() -> &'static [Self] {
                &[$(PowerUnit::$json_variant),*]
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(PowerUnit::$all_variant => $all_symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(PowerUnit::$all_variant => $all_unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(PowerUnit::$all_variant => $all_unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> PowerMeasurementSystem {
                match self {
                    $(PowerUnit::$all_variant => PowerMeasurementSystem::new(
                        MeasurementSystem::$all_energy_measurement_system,
                        MeasurementSystem::$all_duration_measurement_system,
                    )

                    ),+
                }
            }

            pub fn get_energy_variant(&self) -> EnergyUnit {
                match self {
                    $(PowerUnit::$all_variant => EnergyUnit::$all_energy_unit_variant,)+
                }
            }

            pub fn get_duration_variant(&self) -> DurationUnit {
                match self {
                    $(PowerUnit::$all_variant => DurationUnit::$all_duration_unit_variant,)+
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(PowerUnit::$all_variant => $all_si_factor),+
                }
            }
        }

        impl FromStr for PowerUnit {
            type Err = PowerUnitParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($all_symbol_lc | $all_unit_type_lc | $all_unit_type_plural_lc => return Ok(PowerUnit::$all_variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($all_identifier_lc => Ok(PowerUnit::$all_variant),)+
                            err => Err(PowerUnitParseError::UnknownUnit { input: err.to_string() }),
                        }
                    }
                }
            }
        }
    }
}

use units_macro::include_power_units_from_json;
include_power_units_from_json!(
    EnergyUnit => "data/units/energy",
    PowerUnit => "data/units/power",
    DurationUnit => "data/units/duration",
);
