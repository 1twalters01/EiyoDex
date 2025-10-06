#[macro_export]
macro_rules! define_densities {
    (
        all: {
            $(
                $all_variant:ident => {
                    from_fn_name: $all_from_fn_name:ident,
                    as_fn_name: $all_as_fn_name:ident,
                    to_fn_name: $all_to_fn_name:ident,
                    mass_unit_varient: $all_mass_unit_varient: ident,
                    volume_unit_varient: $all_volume_unit_varient:ident,
                    mass_measurement_system: $all_mass_measurement_system: ident,
                    volume_measurement_system: $all_volume_measurement_system: ident,
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
                    from_fn_name: $json_from_fn_name: ident,
                    as_fn_name: $json_as_fn_name: ident,
                    to_fn_name: $json_to_fn_name: ident,
                    mass_unit_varient: $json_mass_unit_variant: ident,
                    volume_unit_varient: $json_volume_unit_varient:ident,
                    mass_measurement_system: $json_mass_measurement_system: ident,
                    volume_measurement_system: $json_volume_measurement_system: ident,
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
        use crate::measurement_system::MeasurementSystem;
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize)]
        pub struct DensityMeasurementSystem {
            mass_measurement_system: MeasurementSystem,
            volume_measurement_system: MeasurementSystem,
        }

        impl DensityMeasurementSystem {
            pub fn get_mass_measurement_system(&self) -> MeasurementSystem {
                self.mass_measurement_system
            }

            pub fn get_volume_measurement_system(&self) -> MeasurementSystem {
                self.volume_measurement_system
            }
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum DensityUnit {
            $($all_variant),+
        }

        impl DensityUnit {
            pub fn get_all_enumerations() -> &'static [Self] {
                &[$(DensityUnit::$all_variant),+]
            }

            pub fn get_selected_enumerations() -> &'static [Self] {
                &[$(DensityUnit::$json_variant),+]
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(DensityUnit::$all_variant => $all_symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(DensityUnit::$all_variant => $all_unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(DensityUnit::$all_variant => $all_unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> DensityMeasurementSystem {
                match self {
                    $(DensityUnit::$all_variant => DensityMeasurementSystem {
                        mass_measurement_system: MeasurementSystem::$all_mass_measurement_system,
                        volume_measurement_system: MeasurementSystem::$all_volume_measurement_system,
                    }),+
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(DensityUnit::$all_variant => $all_si_factor),+
                }
            }
        }

        impl FromStr for DensityUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($all_symbol_lc | $all_unit_type_lc | $all_unit_type_plural_lc => return Ok(DensityUnit::$all_variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($all_identifier_lc => Ok(DensityUnit::$all_variant),)+
                            _ => Err("Unknown density unit"),
                        }
                    }
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct Density {
            value: f64,
            unit: DensityUnit,
        }

        impl Density {
        }
    };
}

use density_macro::include_densities_from_json;
include_densities_from_json!(
    DensityUnit => "data/density.json",
    MassUnit => "data/mass.json",
    VolumeUnit => "data/volume.json"
);
