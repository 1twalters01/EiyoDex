#[macro_export]
macro_rules! define_density_units {
    (
        all: {
            $(
                $all_variant:ident => {
                    mass_unit_variant: $all_mass_unit_variant: ident,
                    volume_unit_variant: $all_volume_unit_variant:ident,
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
                    mass_unit_variant: $json_mass_unit_variant: ident,
                    volume_unit_variant: $json_volume_unit_variant:ident,
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
        use crate::{
            density::measurement_system::DensityMeasurementSystem,
            mass::unit::MassUnit,
            measurement_system::MeasurementSystem,
            volume::unit::VolumeUnit,
        };
        use std::{
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum DensityUnit {
            $($all_variant),+
        }

        impl DensityUnit {
            pub fn from_variants(mass_unit: MassUnit, volume_unit: VolumeUnit) -> DensityUnit {
                match (mass_unit, volume_unit) {
                    $((MassUnit::$all_mass_unit_variant, VolumeUnit::$all_volume_unit_variant) => DensityUnit::$all_variant,)+
                }
            }

            pub fn get_all_enumerations() -> &'static [Self] {
                &[$(DensityUnit::$all_variant),+]
            }

            pub fn get_selected_enumerations() -> &'static [Self] {
                &[$(DensityUnit::$json_variant),*]
            }

            pub fn get_symbol(&self) -> &'static str {
                match self {
                    $(DensityUnit::$all_variant => $all_symbol),+
                }
            }

            pub fn get_unit_type(&self) -> &'static str {
                match self {
                    $(DensityUnit::$all_variant => $all_unit_type),+
                }
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                match self {
                    $(DensityUnit::$all_variant => $all_unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> DensityMeasurementSystem {
                match self {
                    $(DensityUnit::$all_variant => {
                        let mass_measurement_system = MeasurementSystem::$all_mass_measurement_system;
                        let volume_measurement_system = MeasurementSystem::$all_volume_measurement_system;

                        DensityMeasurementSystem::new(
                            mass_measurement_system,
                            volume_measurement_system,
                        )
                    }),+
                }
            }

            pub fn get_mass_variant(&self) -> MassUnit {
                match self {
                    $(DensityUnit::$all_variant => MassUnit::$all_mass_unit_variant,)+
                }
            }

            pub fn get_volume_variant(&self) -> VolumeUnit {
                match self {
                    $(DensityUnit::$all_variant => VolumeUnit::$all_volume_unit_variant,)+
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

                // Warning shows if just one of the user defined types repeats
                #[allow(unreachable_patterns)]
                match formatted_string.as_str() {
                    $($all_symbol_lc | $all_unit_type_lc | $all_unit_type_plural_lc |$all_identifier_lc => return Ok(DensityUnit::$all_variant),)+
                    _ => Err("Unknown density unit"),
                }
            }
        }
    }
}

use units_macro::include_density_units_from_json;
include_density_units_from_json!(
    DensityUnit => "data/units/density",
    MassUnit => "data/units/mass",
    VolumeUnit => "data/units/volume"
);
