// #[allow(unreachable_patterns)]

#[macro_export]
macro_rules! define_energies {
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
            // str::FromStr,
        };
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum EnergyUnit {
            $($variant),+
        }

        impl EnergyUnit {
            pub fn get_enumerations() -> Vec<EnergyUnit> {
                Vec::from([$(EnergyUnit::$variant),+])
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

            pub fn from_str(s: &str) -> Result<Self, &str> {
                match s.trim().to_lowercase().as_str() {
                    $($symbol_lc | $unit_type_lc | $unit_type_plural_lc | $identifier_lc => Ok(EnergyUnit::$variant),)+
                    _ => Err("Unknown energy unit"),
                }
            }

        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
        pub struct Energy {
            value: f64,
            unit: EnergyUnit,
        }

        impl Energy {
            pub fn new(value: f64, unit: EnergyUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $from_fn_name(value: f64) -> Self {
                    Self::new(value, EnergyUnit::$variant)
                }
            )+

            pub fn round(&mut self, dp: u8) -> Self {
                let factor = 10f64.powi(dp as i32);
                self.value = (self.value * factor).round()/factor;
                return *self
            }

            $(
                pub fn $as_fn_name(&self) -> f64 {
                    self.value * self.unit.si_factor() / $si_factor
                }
            )+

            pub fn to_unit(&self, unit: EnergyUnit) -> Self {
                let value = match unit {
                    $(EnergyUnit::$variant => self.$as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $to_fn_name(&self) -> Self {
                    self.to_unit(EnergyUnit::$variant)
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

            pub fn get_unit(&self) -> EnergyUnit {
                self.unit
            }

            pub fn set_unit(&mut self, unit: EnergyUnit) {
                self.unit = unit;
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

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Energy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() + rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Sub for Energy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.get_value() - rhs.to_unit(self.unit).get_value(),
            self.unit,
        )
    }
}

impl Mul<u64> for Energy {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self {
        Self::new(self.get_value() * rhs as f64, self.unit)
    }
}

impl Mul<f64> for Energy {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.get_value() * rhs, self.unit)
    }
}

impl Div<i64> for Energy {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        Self::new(self.get_value() / rhs as f64, self.unit)
    }
}

impl Div<f64> for Energy {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(self.get_value() / rhs, self.unit)
    }
}

impl PartialOrd for Energy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_value()
            .partial_cmp(&other.to_unit(self.unit).get_value())
    }
}

use energy_macro::include_energies_from_json;
include_energies_from_json!("data/energy.json", "data/fake_energy.json",);

