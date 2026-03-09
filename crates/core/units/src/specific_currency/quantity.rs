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
                    si_factor: $volume_si_factor: expr
                }
            ),* $(,)?
        },
    ) => {
        use std::{
        cmp::Ordering,
        fmt,
        ops::{Div, Mul},
        };
        use serde::{Deserialize, Serialize};
        use crate::{
            currency::{
                quantity::CurrencyQuantity,
                unit::CurrencyUnit::{self, *},
            },
            measurement_system::MeasurementSystem,
            mass::quantity::MassQuantity,
            volume::quantity::VolumeQuantity,
            density::{
                quantity::DensityQuantity,
                unit::DensityUnit,
            },
            specific_currency::unit::{Denominator, DenominatorType, SpecificCurrencyUnit},
        };

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct SpecificCurrencyQuantity {
            value: f64,
            unit: SpecificCurrencyUnit,
        }

        impl SpecificCurrencyQuantity {
            pub fn from_variants(value: f64, currency_unit: CurrencyUnit, denominator: Denominator) -> Self {
                Self {
                    value,
                    unit: SpecificCurrencyUnit::from_variants(currency_unit, denominator),
                }
            }

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
                        let numerator_factor: f64 = self.unit.get_currency_unit().get_current_exchange_rate_sync(&$all_currency_unit_variant).expect("Unable to get_current_exchange_rate_sync");
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

impl fmt::Display for SpecificCurrencyQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl<T> Mul<T> for SpecificCurrencyQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl Mul<VolumeQuantity> for SpecificCurrencyQuantity {
    type Output = CurrencyQuantity;

    fn mul(self, rhs: VolumeQuantity) -> CurrencyQuantity {
        let currency_unit = self.get_unit().get_currency_unit();
        let volume_unit = match self.get_unit().get_denominator() {
            Denominator::VolumeDenominator(volume_unit) => volume_unit,
            Denominator::MassDenominator(_) => {
                panic!("Cannot multiply MassQuantity based Specific CurrencyQuantity with VolumeQuantity")
            }
        };

        let specific_currency_value = self.get_value() * rhs.to_unit(volume_unit).get_value();

        CurrencyQuantity::new(specific_currency_value, currency_unit)
    }
}

impl Mul<SpecificCurrencyQuantity> for VolumeQuantity {
    type Output = CurrencyQuantity;

    fn mul(self, rhs: SpecificCurrencyQuantity) -> CurrencyQuantity {
        let currency_unit = rhs.get_unit().get_currency_unit();
        let volume_unit = match rhs.get_unit().get_denominator() {
            Denominator::VolumeDenominator(volume_unit) => volume_unit,
            Denominator::MassDenominator(_) => {
                panic!("Cannot multiply MassQuantity based Specific CurrencyQuantity with VolumeQuantity")
            }
        };

        let specific_currency_value = rhs.get_value() * self.to_unit(volume_unit).get_value();

        CurrencyQuantity::new(specific_currency_value, currency_unit)
    }
}

impl Mul<MassQuantity> for SpecificCurrencyQuantity {
    type Output = CurrencyQuantity;

    fn mul(self, rhs: MassQuantity) -> CurrencyQuantity {
        let currency_unit = self.get_unit().get_currency_unit();
        let mass_unit = match self.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply VolumeQuantity based Specific CurrencyQuantity with MassQuantity")
            }
        };

        let specific_currency_value = self.get_value() * rhs.to_unit(mass_unit).get_value();

        CurrencyQuantity::new(specific_currency_value, currency_unit)
    }
}

impl Mul<SpecificCurrencyQuantity> for MassQuantity {
    type Output = CurrencyQuantity;

    fn mul(self, rhs: SpecificCurrencyQuantity) -> CurrencyQuantity {
        let currency_unit = rhs.get_unit().get_currency_unit();
        let mass_unit = match rhs.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply VolumeQuantity based Specific CurrencyQuantity with MassQuantity")
            }
        };

        let specific_currency_value = rhs.get_value() * self.to_unit(mass_unit).get_value();
        CurrencyQuantity::new(specific_currency_value, currency_unit)
    }
}

impl Mul<DensityQuantity> for SpecificCurrencyQuantity {
    type Output = SpecificCurrencyQuantity;

    fn mul(self, rhs: DensityQuantity) -> SpecificCurrencyQuantity {
        let sc_currency_unit = self.get_unit().get_currency_unit();
        let sc_mass_unit = match self.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply VolumeQuantity based Specific CurrencyQuantity with DensityQuantity")
            }
        };

        let d_volume_unit = rhs.get_unit().get_volume_variant();
        let d_density_unit = DensityUnit::from_variants(sc_mass_unit, d_volume_unit);

        let new_denominator = Denominator::VolumeDenominator(d_volume_unit);
        let new_specific_currency_unit =
            SpecificCurrencyUnit::from_variants(sc_currency_unit, new_denominator);

        let density = rhs.to_unit(d_density_unit).get_value();
        let specific_currency = self.get_value();
        SpecificCurrencyQuantity::new(density * specific_currency, new_specific_currency_unit)
    }
}

impl Mul<SpecificCurrencyQuantity> for DensityQuantity {
    type Output = SpecificCurrencyQuantity;

    fn mul(self, rhs: SpecificCurrencyQuantity) -> SpecificCurrencyQuantity {
        let sc_currency_unit = rhs.get_unit().get_currency_unit();
        let sc_mass_unit = match rhs.get_unit().get_denominator() {
            Denominator::MassDenominator(mass_unit) => mass_unit,
            Denominator::VolumeDenominator(_) => {
                panic!("Cannot multiply VolumeQuantity based Specific CurrencyQuantity with DensityQuantity")
            }
        };

        let d_volume_unit = self.get_unit().get_volume_variant();
        let d_density_unit = DensityUnit::from_variants(sc_mass_unit, d_volume_unit);

        let new_denominator = Denominator::VolumeDenominator(d_volume_unit);
        let new_specific_currency_unit =
            SpecificCurrencyUnit::from_variants(sc_currency_unit, new_denominator);

        let density = self.to_unit(d_density_unit).get_value();
        let specific_currency = rhs.get_value();
        SpecificCurrencyQuantity::new(density * specific_currency, new_specific_currency_unit)
    }
}

impl<T> Div<T> for SpecificCurrencyQuantity
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Div<VolumeQuantity> for CurrencyQuantity {
    type Output = SpecificCurrencyQuantity;

    fn div(self, rhs: VolumeQuantity) -> SpecificCurrencyQuantity {
        let currency_unit = self.get_unit();
        let volume_unit = rhs.get_unit();
        let denominator = Denominator::VolumeDenominator(volume_unit);

        let specific_currency_value = self.get_value() / rhs.get_value();
        let specific_currency_unit =
            SpecificCurrencyUnit::from_variants(currency_unit, denominator);

        SpecificCurrencyQuantity::new(specific_currency_value, specific_currency_unit)
    }
}

impl Div<MassQuantity> for CurrencyQuantity {
    type Output = SpecificCurrencyQuantity;

    fn div(self, rhs: MassQuantity) -> SpecificCurrencyQuantity {
        let currency_unit = self.get_unit();
        let mass_unit = rhs.get_unit();
        let denominator = Denominator::MassDenominator(mass_unit);

        let specific_currency_value = self.get_value() / rhs.get_value();
        let specific_currency_unit =
            SpecificCurrencyUnit::from_variants(currency_unit, denominator);

        SpecificCurrencyQuantity::new(specific_currency_value, specific_currency_unit)
    }
}

impl Div<DensityQuantity> for SpecificCurrencyQuantity {
    type Output = SpecificCurrencyQuantity;

    fn div(self, rhs: DensityQuantity) -> SpecificCurrencyQuantity {
        let sc_currency_unit = self.get_unit().get_currency_unit();
        let sc_volume_unit = match self.get_unit().get_denominator() {
            Denominator::VolumeDenominator(volume_unit) => volume_unit,
            Denominator::MassDenominator(_) => {
                panic!("Cannot divide MassQuantity based Specific CurrencyQuantity with DensityQuantity")
            }
        };

        let d_mass_unit = rhs.get_unit().get_mass_variant();
        let d_density_unit = DensityUnit::from_variants(d_mass_unit, sc_volume_unit);

        let new_denominator = Denominator::MassDenominator(d_mass_unit);
        let new_specific_currency_unit =
            SpecificCurrencyUnit::from_variants(sc_currency_unit, new_denominator);

        let density = rhs.to_unit(d_density_unit).get_value();
        let specific_currency = self.get_value();
        SpecificCurrencyQuantity::new(specific_currency / density, new_specific_currency_unit)
    }
}

impl PartialOrd for SpecificCurrencyQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value().partial_cmp(
            &other
                .to_unit(self.unit)
                .expect("Cannot compare mass based specific currencies to volume based ones")
                .get_value(),
        )
    }
}

use units_macro::include_specific_currencies_from_json;
include_specific_currencies_from_json!(
    CurrencyUnit => "data/units/currency",
    VolumeUnit => "data/units/volume",
    MassUnit => "data/units/mass",
    SpecificCurrencyUnit => "data/units/specific_currency",
);
