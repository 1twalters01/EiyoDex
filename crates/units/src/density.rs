#[macro_export]
macro_rules! define_densities {
    (
        all: {
            $(
                $all_variant:ident => {
                    from_fn_name: $all_from_fn_name:ident,
                    as_fn_name: $all_as_fn_name:ident,
                    to_fn_name: $all_to_fn_name:ident,
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
                    from_fn_name: $json_from_fn_name: ident,
                    as_fn_name: $json_as_fn_name: ident,
                    to_fn_name: $json_to_fn_name: ident,
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
            mass::{Mass, MassUnit},
            measurement_system::MeasurementSystem,
            volume::{Volume, VolumeUnit},
        };
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            iter::Sum,
            str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Deserialize, PartialEq)]
        pub struct DensityMeasurementSystem {
            mass_measurement_system: MeasurementSystem,
            volume_measurement_system: MeasurementSystem,
        }

        impl DensityMeasurementSystem {
            pub fn new(mass_measurement_system: MeasurementSystem, volume_measurement_system: MeasurementSystem) -> DensityMeasurementSystem {
                Self { mass_measurement_system, volume_measurement_system }
            }

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
            pub fn from_variants(value: f64, mass_unit: MassUnit, volume_unit: VolumeUnit) -> Self {
                Self {
                    value,
                    unit: DensityUnit::from_variants(mass_unit, volume_unit),
                }
            }

            pub fn new(value: f64, unit: DensityUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $all_from_fn_name(value: f64) -> Self {
                    Self::new(value, DensityUnit::$all_variant)
                }
            )+

            pub fn round(&mut self, dp: u8) -> Self {
                let factor = 10f64.powi(dp as i32);
                self.value = (self.value * factor).round()/factor;
                return *self
            }

            $(
                pub fn $all_as_fn_name(&self) -> f64 {
                    self.value * self.unit.si_factor() / $all_si_factor
                }
            )+

            pub fn to_unit(&self, unit: DensityUnit) -> Self {
                let value = match unit {
                    $(DensityUnit::$all_variant => self.$all_as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $all_to_fn_name(&self) -> Self {
                    self.to_unit(DensityUnit::$all_variant)
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

            pub fn get_unit(&self) -> DensityUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: DensityUnit) {
                self.unit = unit;
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.as_symbol()
            }

            pub fn get_measurement_system(&self) -> DensityMeasurementSystem {
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

impl fmt::Display for Density {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Density {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for Density {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl<T> Mul<T> for Density
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl Mul<Volume> for Density {
    type Output = Mass;

    fn mul(self, rhs: Volume) -> Mass {
        let density_volume_variant = self.get_unit().get_volume_variant();
        let density_mass_variant = self.get_unit().get_mass_variant();
        let volume = rhs.to_unit(density_volume_variant).get_value();
        let density = self.get_value();
        Mass::new(density * volume, density_mass_variant)
    }
}

impl Mul<Density> for Volume {
    type Output = Mass;

    fn mul(self, rhs: Density) -> Mass {
        let density_volume_variant = rhs.get_unit().get_volume_variant();
        let density_mass_variant = rhs.get_unit().get_mass_variant();
        let volume = self.to_unit(density_volume_variant).get_value();
        let density = rhs.get_value();
        Mass::new(density * volume, density_mass_variant)
    }
}

impl<T> Div<T> for Density
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Div<Volume> for Mass {
    type Output = Density;

    fn div(self, rhs: Volume) -> Density {
        let value = self.get_value() / rhs.get_value();
        let mass_unit = self.get_unit();
        let volume_unit = rhs.get_unit();
        Density::from_variants(value, mass_unit, volume_unit)
    }
}

impl Sum for Density {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Density::new(0f64, DensityUnit::KilogramPerLiter), |a, b| {
            b + a
        })
    }
}

impl PartialOrd for Density {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use density_macro::include_densities_from_json;
include_densities_from_json!(
    DensityUnit => "data/units/density",
    MassUnit => "data/units/mass",
    VolumeUnit => "data/units/volume"
);
