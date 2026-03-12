#[macro_export]
macro_rules! define_specific_currency_units {
    (
        all: {
            $(
                $all_variant:ident => {
                    currency_unit_variant: $all_currency_unit_variant: ident,
                    denominator_unit_variant: $all_denominator_unit_variant:ident,
                    denominator_unit_type: $all_denominator_unit_type:ident,
                    denominator_measurement_system: $all_denominator_measurement_system: ident,
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
                    currency_unit_variant: $json_currency_unit_variant: ident,
                    denominator_unit_variant: $json_denominator_unit_variant:ident,
                    denominator_unit_type: $json_denominator_unit_type:ident,
                    denominator_measurement_system: $json_denominator_measurement_system: ident,
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
        mass: {
            $(
                $mass_variant:ident => {
                    currency_unit_variant: $mass_currency_unit_variant: ident,
                    denominator_unit_variant: $mass_denominator_unit_variant:ident,
                    denominator_unit_type: $mass_denominator_unit_type:ident,
                    denominator_measurement_system: $mass_denominator_measurement_system: ident,
                    symbol: $mass_symbol: expr,
                    symbol_lc: $mass_symbol_lc: expr,
                    unit_type: $mass_unit_type: expr,
                    unit_type_lc: $mass_unit_type_lc: expr,
                    unit_type_plural: $mass_unit_type_plural: expr,
                    unit_type_plural_lc: $mass_unit_type_plural_lc: expr,
                    identifier_lc: $mass_identifier_lc: expr,
                    si_factor: $mass_si_factor: expr
                }
            ),* $(,)?
        },
        volume: {
            $(
                $volume_variant:ident => {
                    currency_unit_variant: $volume_currency_unit_variant: ident,
                    denominator_unit_variant: $volume_denominator_unit_variant:ident,
                    denominator_unit_type: $volume_denominator_unit_type:ident,
                    denominator_measurement_system: $volume_denominator_measurement_system: ident,
                    symbol: $volume_symbol: expr,
                    symbol_lc: $volume_symbol_lc: expr,
                    unit_type: $volume_unit_type: expr,
                    unit_type_lc: $volume_unit_type_lc: expr,
                    unit_type_plural: $volume_unit_type_plural: expr,
                    unit_type_plural_lc: $volume_unit_type_plural_lc: expr,
                    identifier_lc: $volume_identifier_lc: expr,
                    si_factor: $volume_si_factor: expr
                }
            ),* $(,)?
        },
    ) => {
        use std::{
        str::FromStr,
        };
        use serde::{Deserialize, Serialize};
        use crate::{
            currency::unit::CurrencyUnit::self,
            measurement_system::MeasurementSystem,
            mass::unit::MassUnit,
            volume::unit::VolumeUnit,
            specific_currency::error::SpecificCurrencyUnitParseError,
        };

        #[derive(Debug, PartialEq)]
        pub enum Denominator {
            MassDenominator(MassUnit),
            VolumeDenominator(VolumeUnit),
        }

        impl Denominator {
            pub fn from_mass_unit(mass_unit: MassUnit) -> Self {
                Self::MassDenominator(mass_unit)
            }

            pub fn from_volume_unit(volume_unit: VolumeUnit) -> Self {
                Self::VolumeDenominator(volume_unit)
            }

            pub fn get_mass_variant(&self) -> Option<MassUnit> {
                match self {
                    Denominator::MassDenominator(mass_unit) => Some(*mass_unit),
                    Denominator::VolumeDenominator(_) => None,
                }
            }

            pub fn get_volume_variant(&self) -> Option<VolumeUnit> {
                match self {
                    Denominator::MassDenominator(_) => None,
                    Denominator::VolumeDenominator(volume_unit) => Some(*volume_unit),
                }
            }
        }

        #[derive(Debug, PartialEq)]
        pub enum DenominatorType {
            MassDenominator,
            VolumeDenominator,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum SpecificCurrencyUnit {
            $($all_variant),+
        }

        impl SpecificCurrencyUnit {
            pub fn from_variants(currency_unit: CurrencyUnit, denominator: Denominator) -> SpecificCurrencyUnit {
                match denominator {
                    Denominator::MassDenominator(mass_unit) => {
                        match (currency_unit, mass_unit) {
                            $((CurrencyUnit::$mass_currency_unit_variant, MassUnit::$mass_denominator_unit_variant) => SpecificCurrencyUnit::$mass_variant,)+
                        }
                    },
                    Denominator::VolumeDenominator(volume_unit) => {
                        match (currency_unit, volume_unit) {
                            $((CurrencyUnit::$volume_currency_unit_variant, VolumeUnit::$volume_denominator_unit_variant) => SpecificCurrencyUnit::$volume_variant,)+
                        }
                    }
                }
            }

            pub fn get_all_enumerations() -> &'static [Self] {
                &[$(SpecificCurrencyUnit::$all_variant),+]
            }

            pub fn get_selected_enumerations() -> &'static [Self] {
                &[$(SpecificCurrencyUnit::$json_variant),*]
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => $all_symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => $all_unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => $all_unit_type_plural),+
                }
            }

            pub fn get_measurement_system(&self) -> MeasurementSystem {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => MeasurementSystem::$all_denominator_measurement_system,)+
                }
            }

            pub fn get_denominator_type(&self) -> DenominatorType {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => DenominatorType::$all_denominator_unit_type,)+
                }
            }

            pub fn get_currency_unit(&self) -> CurrencyUnit {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => CurrencyUnit::$all_currency_unit_variant),+
                }
            }

            pub fn get_denominator(&self) -> Denominator {
                match self.get_denominator_type() {
                    DenominatorType::MassDenominator => {
                        match self {
                            $(SpecificCurrencyUnit::$mass_variant => Denominator::MassDenominator(MassUnit::$mass_denominator_unit_variant),)+
                            _ => panic!("Volume based specific currency can not have a mass denominator"),
                        }
                    },
                    DenominatorType::VolumeDenominator => {
                        match self {
                            $(SpecificCurrencyUnit::$volume_variant => Denominator::VolumeDenominator(VolumeUnit::$volume_denominator_unit_variant),)+
                            _ => panic!("Mass based specific currency can not have a mass denominator"),
                        }
                    },
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => $all_si_factor),+
                }
            }
        }

        impl FromStr for SpecificCurrencyUnit {
            type Err = SpecificCurrencyUnitParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($all_symbol_lc | $all_unit_type_lc => return Ok(SpecificCurrencyUnit::$all_variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($all_identifier_lc | $all_unit_type_plural_lc=> Ok(SpecificCurrencyUnit::$all_variant),)+
                            _ => {
                                let (currency_str, denominator_str) = formatted_string.split_once("/").ok_or(SpecificCurrencyUnitParseError::InvalidFormat { input: formatted_string.to_string() })?;
                                let currency_unit = CurrencyUnit::from_str(currency_str);
                                let mass_unit = MassUnit::from_str(denominator_str);
                                let volume_unit = VolumeUnit::from_str(denominator_str);

                                if let Some(err) = currency_unit.err() {
                                    return Err(SpecificCurrencyUnitParseError::UnknownCurrencyUnit { input: err.to_string() })
                                }
                                if mass_unit.is_err() && volume_unit.is_err() {
                                    return Err(SpecificCurrencyUnitParseError::UnknownDenominatorUnit { input: denominator_str.to_string() })
                                }
                                if mass_unit.is_ok() {
                                    let denominator = Denominator::MassDenominator(mass_unit.unwrap());
                                    return Ok(SpecificCurrencyUnit::from_variants(currency_unit.unwrap(), denominator))
                                }
                                if volume_unit.is_ok() {
                                    let denominator = Denominator::VolumeDenominator(volume_unit.unwrap());
                                    return Ok(SpecificCurrencyUnit::from_variants(currency_unit.unwrap(), denominator))
                                }

                                return Err(SpecificCurrencyUnitParseError::UnknownUnit { input: formatted_string.to_string() })
                            }
                        }
                    }
                }
            }
        }
    }
}

use units_macro::include_specific_currency_units_from_json;
include_specific_currency_units_from_json!(
    CurrencyUnit => "data/units/currency",
    VolumeUnit => "data/units/volume",
    MassUnit => "data/units/mass",
    SpecificCurrencyUnit => "data/units/specific_currency",
);
