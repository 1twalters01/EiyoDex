#[macro_export]
macro_rules! define_specific_currencies {
    (
        all: {
            $(
                $all_variant:ident => {
                    from_fn_name: $all_from_fn_name:ident,
                    as_fn_name: $all_as_fn_name:ident,
                    to_fn_name: $all_to_fn_name:ident,
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
                    from_fn_name: $json_from_fn_name: ident,
                    as_fn_name: $json_as_fn_name: ident,
                    to_fn_name: $json_to_fn_name: ident,
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
                    from_fn_name: $mass_from_fn_name: ident,
                    as_fn_name: $mass_as_fn_name: ident,
                    to_fn_name: $mass_to_fn_name: ident,
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
                    from_fn_name: $volume_from_fn_name: ident,
                    as_fn_name: $volume_as_fn_name: ident,
                    to_fn_name: $volume_to_fn_name: ident,
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
        cmp::Ordering,
        fmt,
        ops::{Div, Mul},
        str::FromStr,
        };
        use serde::{Deserialize, Serialize};
        use crate::{
            currency::{
                get_current_exchange_rate_sync,
                Currency,
                CurrencyUnit::{self, *},
            },
            measurement_system::MeasurementSystem,
            mass::{ Mass, MassUnit },
            volume::{ Volume, VolumeUnit },
            density::Density,
        };

        #[derive(PartialEq)]
        pub enum Denominator {
            MassDenominator(MassUnit),
            VolumeDenominator(VolumeUnit),
        }

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
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($all_symbol_lc | $all_unit_type_lc => return Ok(SpecificCurrencyUnit::$all_variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($all_identifier_lc | $all_unit_type_plural_lc=> Ok(SpecificCurrencyUnit::$all_variant),)+
                            _ => Err("Unknown density unit"),
                        }
                    }
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct SpecificCurrency {
            value: f64,
            unit: SpecificCurrencyUnit,
        }

        impl SpecificCurrency {
            pub fn new(value: f64, unit: SpecificCurrencyUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $all_from_fn_name(value: f64) -> Self {
                    Self::new(value, SpecificCurrencyUnit::$all_variant)
                }
            )+

            pub fn round(&mut self, dp: u8) -> Self {
                let factor = 10f64.powi(dp as i32);
                self.value = (self.value * factor).round()/factor;
                return *self
            }

            $(
                pub fn $all_as_fn_name(&self) -> Result<f64, &'static str> {
                    if let DenominatorType::$all_denominator_unit_type = self.get_unit().get_denominator_type() {
                        let numerator_factor: f64 = get_current_exchange_rate_sync(self.unit.get_currency_unit(), $all_currency_unit_variant).expect("Unable to get_current_exchange_rate_sync");
                        let denominator_factor: f64 = self.unit.si_factor() / $all_si_factor;
                        return Ok(self.value * numerator_factor * denominator_factor)
                    } else {
                        return Err("Cannot convert mass to volume")
                    }
                }
            )+

            pub fn to_unit(&self, unit: SpecificCurrencyUnit) -> Result<Self, &'static str> {
                let value = match unit {
                    $(SpecificCurrencyUnit::$all_variant => self.$all_as_fn_name()),+
                };
                match value {
                    Ok(value) => Ok(Self { value, unit }),
                    Err(err) => Err(err),
                }
            }

            $(
                pub fn $all_to_fn_name(&self) -> Result<Self, &'static str> {
                    self.to_unit(SpecificCurrencyUnit::$all_variant)
                }
            )+

            pub fn is_zero(&self) -> bool {
                self.value == 0.0
            }

            pub fn is_negative(&self) -> bool {
                self.value < 0.0
            }

            pub fn get_value(&self) -> f64 {
                self.value
            }

            pub fn set_value(&mut self, value: f64) {
                self.value = value;
            }

            pub fn get_unit(&self) -> SpecificCurrencyUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: SpecificCurrencyUnit) {
                self.unit = unit;
            }

            pub fn get_denominator_type(&self) -> DenominatorType {
                self.unit.get_denominator_type()
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.as_symbol()
            }

            pub fn get_measurement_system(&self) -> MeasurementSystem {
                self.unit.get_measurement_system()
            }

            pub fn get_unit_type(&self) -> &'static str {
                self.unit.as_unit_type()
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                self.unit.as_unit_type_plural()
            }

            pub fn to_string(&self) -> String {
                format!("{}{}", self.value.to_string().trim(), self.get_symbol().trim())
            }
        }
    };
}

impl fmt::Display for SpecificCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl<T> Mul<T> for SpecificCurrency
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl Mul<Volume> for SpecificCurrency {
    type Output = Currency;

    fn mul(self, rhs: Volume) -> Currency {
        let currency_unit = self.get_unit().get_currency_unit();
        let volume_unit = match self.get_unit().get_denominator() {
            Denominator::VolumeDenominator(volume_unit) => volume_unit,
            Denominator::MassDenominator(_) => {
                panic!("Cannot multiply Mass based Specific Currency with Volume")
            }
        };

        let specific_currency_value = self.get_value() * rhs.to_unit(volume_unit).get_value();

        Currency::new(specific_currency_value, currency_unit)
    }
}

impl Mul<SpecificCurrency> for Volume {
    type Output = Currency;

    fn mul(self, rhs: SpecificCurrency) -> Currency {
        let currency_unit = rhs.get_unit().get_currency_unit();
        let volume_unit = match rhs.get_unit().get_denominator() {
            Denominator::VolumeDenominator(volume_unit) => volume_unit,
            Denominator::MassDenominator(_) => {
                panic!("Cannot multiply Mass based Specific Currency with Volume")
            }
        };

        let specific_currency_value = rhs.get_value() * self.to_unit(volume_unit).get_value();

        Currency::new(specific_currency_value, currency_unit)
    }
}

impl Mul<Mass> for SpecificCurrency {
    type Output = Currency;

    fn mul(self, rhs: Mass) -> Currency {
        let currency_unit = self.get_unit().get_currency_unit();
        let mass_unit = match self.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply Volume based Specific Currency with Mass")
            }
        };

        let specific_currency_value = self.get_value() * rhs.to_unit(mass_unit).get_value();

        Currency::new(specific_currency_value, currency_unit)
    }
}

impl Mul<SpecificCurrency> for Mass {
    type Output = Currency;

    fn mul(self, rhs: SpecificCurrency) -> Currency {
        let currency_unit = rhs.get_unit().get_currency_unit();
        let mass_unit = match rhs.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply Volume based Specific Currency with Mass")
            }
        };

        let specific_currency_value = rhs.get_value() * self.to_unit(mass_unit).get_value();
        Currency::new(specific_currency_value, currency_unit)
    }
}

impl Mul<Density> for SpecificCurrency {
    type Output = SpecificCurrency;

    fn mul(self, rhs: Density) -> SpecificCurrency {
        let sc_currency_unit = self.get_unit().get_currency_unit();
        let sc_mass_unit = match self.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply Volume based Specific Currency with Density")
            }
        };

        let d_volume_unit = rhs.get_unit().get_volume_variant();
        let d_density_unit = DensityUnit::from_variants(sc_mass_unit, d_volume_unit);

        let new_denominator = Denominator::VolumeDenominator(d_volume_unit);
        let new_specific_currency_unit =
            SpecificCurrencyUnit::from_variants(sc_currency_unit, new_denominator);

        let density = rhs.to_unit(d_density_unit).get_value();
        let specific_currency = self.get_value();
        SpecificCurrency::new(density * specific_currency, new_specific_currency_unit)
    }
}

impl Mul<SpecificCurrency> for Density {
    type Output = SpecificCurrency;

    fn mul(self, rhs: SpecificCurrency) -> SpecificCurrency {
        let sc_currency_unit = rhs.get_unit().get_currency_unit();
        let sc_mass_unit = match rhs.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply Volume based Specific Currency with Density")
            }
        };

        let d_volume_unit = self.get_unit().get_volume_variant();
        let d_density_unit = DensityUnit::from_variants(sc_mass_unit, d_volume_unit);

        let new_denominator = Denominator::VolumeDenominator(d_volume_unit);
        let new_specific_currency_unit =
            SpecificCurrencyUnit::from_variants(sc_currency_unit, new_denominator);

        let density = self.to_unit(d_density_unit).get_value();
        let specific_currency = rhs.get_value();
        SpecificCurrency::new(density * specific_currency, new_specific_currency_unit)
    }
}

impl<T> Div<T> for SpecificCurrency
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Div<Volume> for Currency {
    type Output = SpecificCurrency;

    fn div(self, rhs: Volume) -> SpecificCurrency {
        let currency_unit = self.get_unit();
        let volume_unit = rhs.get_unit();
        let denominator = Denominator::VolumeDenominator(volume_unit);

        let specific_currency_value = self.get_value() / rhs.get_value();
        let specific_currency_unit =
            SpecificCurrencyUnit::from_variants(currency_unit, denominator);

        SpecificCurrency::new(specific_currency_value, specific_currency_unit)
    }
}

impl Div<Mass> for Currency {
    type Output = SpecificCurrency;

    fn div(self, rhs: Mass) -> SpecificCurrency {
        let currency_unit = self.get_unit();
        let mass_unit = rhs.get_unit();
        let denominator = Denominator::MassDenominator(mass_unit);

        let specific_currency_value = self.get_value() / rhs.get_value();
        let specific_currency_unit =
            SpecificCurrencyUnit::from_variants(currency_unit, denominator);

        SpecificCurrency::new(specific_currency_value, specific_currency_unit)
    }
}

impl Div<Density> for SpecificCurrency {
    type Output = SpecificCurrency;

    fn div(self, rhs: Density) -> SpecificCurrency {
        let sc_currency_unit = self.get_unit().get_currency_unit();
        let sc_volume_unit = match self.get_unit().get_denominator() {
            Denominator::VolumeDenominator(volume_unit) => volume_unit,
            Denominator::MassDenominator(_) => {
                panic!("Cannot divide Mass based Specific Currency with Density")
            }
        };

        let d_mass_unit = rhs.get_unit().get_mass_variant();
        let d_density_unit = DensityUnit::from_variants(d_mass_unit, sc_volume_unit);

        let new_denominator = Denominator::MassDenominator(d_mass_unit);
        let new_specific_currency_unit =
            SpecificCurrencyUnit::from_variants(sc_currency_unit, new_denominator);

        let density = rhs.to_unit(d_density_unit).get_value();
        let specific_currency = self.get_value();
        SpecificCurrency::new(specific_currency / density, new_specific_currency_unit)
    }
}

impl PartialOrd for SpecificCurrency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(
            &other
                .to_unit(self.unit)
                .expect("Cannot compare mass based specific currencies to volume based ones")
                .get_value(),
        )
    }
}

use specific_currency_macro::include_specific_currencies_from_json;

use crate::density::DensityUnit;
include_specific_currencies_from_json!(
    CurrencyUnit => "data/units/currency",
    VolumeUnit => "data/units/volume",
    MassUnit => "data/units/mass",
    SpecificCurrencyUnit => "data/units/specific_currency",
);
