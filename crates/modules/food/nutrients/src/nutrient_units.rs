use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use std::str::FromStr;

#[macro_export]
macro_rules! define_nutrient_units {
    (
        $(
            $variant:ident => {
                symbol: $symbol:expr,
                symbol_lc: $symbol_lc:expr,
                unit_type: $unit_type:expr,
                unit_type_lc: $unit_type_lc:expr,
                unit_type_plural: $unit_type_plural:expr,
                unit_type_plural_lc: $unit_type_plural_lc:expr,
                identifier_lc: $identifier_lc:expr,
            }
        ),+ $(,)?
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub enum NutrientUnit {
            Mass(MassUnit),
            Volume(VolumeUnit),
            Energy(EnergyUnit),
            $($variant),*
        }

        impl NutrientUnit {
            pub fn get_enumerations() -> Vec<Self> {
                let mass_enumerations: Vec<NutrientUnit> = MassUnit::get_enumerations().iter().map(|unit| NutrientUnit::Mass(*unit)).collect();
                let volume_enumerations: Vec<NutrientUnit> = VolumeUnit::get_enumerations().iter().map(|unit| NutrientUnit::Volume(*unit)).collect();
                let energy_enumerations:Vec<NutrientUnit> = EnergyUnit::get_enumerations().iter().map(|unit| NutrientUnit::Energy(*unit)).collect();
                
                let other_enumerations = Vec::from([$(NutrientUnit::$variant),+]);

                let mut enumerations = Vec::new();
                enumerations.extend(mass_enumerations);
                enumerations.extend(volume_enumerations);
                enumerations.extend(energy_enumerations);
                enumerations.extend(other_enumerations);
                return enumerations
            }

            pub fn get_unit_type(&self) -> &'static str {
                match self {
                    NutrientUnit::Mass(unit) => unit.get_unit_type(),
                    NutrientUnit::Volume(unit) => unit.get_unit_type(),
                    NutrientUnit::Energy(unit) => unit.get_unit_type(),
                    $(NutrientUnit::$variant => $unit_type),+
                }
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                match self {
                    NutrientUnit::Mass(unit) => unit.get_unit_type_plural(),
                    NutrientUnit::Volume(unit) => unit.get_unit_type_plural(),
                    NutrientUnit::Energy(unit) => unit.get_unit_type_plural(),
                    $(NutrientUnit::$variant => $unit_type_plural),+
                }
            }

            pub fn si_factor(&self) -> Option<f64> {
                match self {
                    Self::Mass(unit) => Some(unit.si_factor()),
                    Self::Volume(unit) => Some(unit.si_factor()),
                    Self::Energy(unit) => Some(unit.si_factor()),
                    _ => None,
                }
            }
        }

        impl FromStr for NutrientUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, &'static str> {
                let formatted_string = s.trim().to_lowercase();

                // Warning shows if just one of the user defined types repeats
                #[allow(unreachable_patterns)]
                match formatted_string.as_str() {
                    $($symbol_lc | $unit_type_lc | $unit_type_plural_lc | $identifier_lc => return Ok(NutrientUnit::$variant),)+
                    _ => Err("Unknown unit"),
                }
            }
        }
    }
}

use nutrients_macro::include_nutrient_units_from_json;
include_nutrient_units_from_json!("data/nutrients/nutrient_units");

