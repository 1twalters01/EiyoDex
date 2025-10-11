#[macro_export]
macro_rules! define_powers {
    (
        all: {
            $(
                $all_variant:ident => {
                    from_fn_name: $all_from_fn_name:ident,
                    as_fn_name: $all_as_fn_name:ident,
                    to_fn_name: $all_to_fn_name:ident,
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
                    from_fn_name: $json_from_fn_name: ident,
                    as_fn_name: $json_as_fn_name: ident,
                    to_fn_name: $json_to_fn_name: ident,
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
            energy::{Energy, EnergyUnit},
            into_f64::IntoF64Safe,
        };
        use chrono::Duration;
        use std::{
            cmp::Ordering,
            fmt,
            ops::{Add, Div, Mul, Sub},
            str::FromStr,
        };

        use serde::{Deserialize, Serialize};

        #[derive(Deserialize)]
        pub struct PowerMeasurementSystem {
            energy_measurement_system: MeasurementSystem,
            duration_measurement_system: MeasurementSystem,
        }

        impl PowerMeasurementSystem {
            pub fn get_energy_measurement_system(&self) -> MeasurementSystem {
                self.energy_measurement_system
            }

            pub fn get_duration_measurement_system(&self) -> MeasurementSystem {
                self.duration_measurement_system
            }
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum PowerUnit {
            $($all_variant),+
        }

        impl PowerUnit {
            pub fn from_variants(energy_unit: EnergyUnit) -> PowerUnit {
                match (energy_unit, "Second") {
                    $((EnergyUnit::$all_energy_unit_variant, stringify!($all_duration_unit_variant)) => PowerUnit::$all_variant,)+
                    _ => panic!("Cannot find unit"),
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
                    $(PowerUnit::$all_variant => PowerMeasurementSystem {
                        energy_measurement_system: MeasurementSystem::$all_energy_measurement_system,
                        duration_measurement_system: MeasurementSystem::$all_duration_measurement_system,
                    }),+
                }
            }

            pub fn get_energy_variant(&self) -> EnergyUnit {
                match self {
                    $(PowerUnit::$all_variant => EnergyUnit::$all_energy_unit_variant,)+
                }
            }

            pub fn si_factor(&self) -> f64 {
                match self {
                    $(PowerUnit::$all_variant => $all_si_factor),+
                }
            }
        }

        impl FromStr for PowerUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let formatted_string = s.trim().to_lowercase();
                match formatted_string.as_str() {
                    $($all_symbol_lc | $all_unit_type_lc | $all_unit_type_plural_lc => return Ok(PowerUnit::$all_variant),)+
                    _ => {
                        match formatted_string.as_str() {
                            $($all_identifier_lc => Ok(PowerUnit::$all_variant),)+
                            _ => Err("Unknown density unit"),
                        }
                    }
                }
            }
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct Power {
            value: f64,
            unit: PowerUnit,
        }

        impl Power {
            pub fn new(value: f64, unit: PowerUnit) -> Self {
                Self { value, unit }
            }

            pub fn from_variants(value: f64, energy_unit: EnergyUnit) -> Self {
                Self {
                    value,
                    unit: PowerUnit::from_variants(energy_unit),
                }
            }

            $(
                pub fn $all_from_fn_name(value: f64) -> Self {
                    Self::new(value, PowerUnit::$all_variant)
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

            pub fn to_unit(&self, unit: PowerUnit) -> Self {
                let value = match unit {
                    $(PowerUnit::$all_variant => self.$all_as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $all_to_fn_name(&self) -> Self {
                    self.to_unit(PowerUnit::$all_variant)
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

            pub fn get_unit(&self) -> PowerUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: PowerUnit) {
                self.unit = unit;
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.as_symbol()
            }

            pub fn get_measurement_system(&self) -> PowerMeasurementSystem {
                self.unit.get_measurement_system()
            }

            pub fn get_unit_type(&self) -> &'static str {
                self.unit.as_unit_type()
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                self.unit.as_unit_type_plural()
            }

            pub fn to_string(&self) -> String {
                format!("{}{}", self.value, self.get_symbol())
            }
        }
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Power {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for Power {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl<T> Div<T> for Power
where
    T: Into<f64> + IntoF64Safe + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.get_value() / rhs.into(), self.unit)
    }
}

impl Div<Duration> for Energy {
    type Output = Power;

    fn div(self, rhs: Duration) -> Power {
        let value = self.get_value() / rhs.num_seconds() as f64;
        let energy_unit = self.get_unit();
        Power::from_variants(value, energy_unit)
    }
}

impl<T> Mul<T> for Power
where
    T: Into<f64> + IntoF64Safe + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.get_value() * rhs.into(), self.unit)
    }
}

impl Mul<Duration> for Power {
    type Output = Energy;

    fn mul(self, rhs: Duration) -> Energy {
        let power_energy_variant = self.get_unit().get_energy_variant();
        let seconds: f64 = rhs.num_seconds() as f64;
        Energy::from_kcal(seconds as f64 * self.as_kcal_per_s()).to_unit(power_energy_variant)
    }
}

impl Mul<Power> for Duration {
    type Output = Energy;

    fn mul(self, rhs: Power) -> Energy {
        let power_energy_variant = rhs.get_unit().get_energy_variant();
        let seconds: f64 = self.num_seconds() as f64;
        Energy::from_kcal(seconds as f64 * rhs.as_kcal_per_s()).to_unit(power_energy_variant)
    }
}

impl PartialOrd for Power {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use power_macro::include_powers_from_json;
include_powers_from_json!(
    EnergyUnit => "data/units/energy",
    PowerUnit => "data/units/power",
    DurationUnit => "data/units/duration",
);
