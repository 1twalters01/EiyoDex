#[macro_export]
macro_rules! define_masses {
    (
        $(
            $variant:ident => {
                from_fn_name: $from_fn_name:ident,
                as_fn_name: $as_fn_name:ident,
                to_fn_name: $to_fn_name:ident,
                symbol: $symbol:expr,
                unit_type: $unit_type:expr,
                unit_type_plural: $unit_type_plural:expr,
                factor: $factor:expr
            }
        ),+ $(,)?
    ) => {
        use std::{
        cmp::Ordering,
        fmt,
        ops::{Add, Div, Mul, Sub},
        str::FromStr,
        };
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum MassUnit {
            $($variant),+
        }

        impl MassUnit {
            pub fn get_enumerations() -> Vec<MassUnit> {
                Vec::from([$(MassUnit::$variant),+])
            }

            pub fn as_symbol(&self) -> &'static str {
                match self {
                    $(MassUnit::$variant => $symbol),+
                }
            }

            pub fn as_unit_type(&self) -> &'static str {
                match self {
                    $(MassUnit::$variant => $unit_type),+
                }
            }

            pub fn as_unit_type_plural(&self) -> &'static str {
                match self {
                    $(MassUnit::$variant => $unit_type_plural),+
                }
            }

            pub fn grams_factor(&self) -> f64 {
                match self {
                    $(MassUnit::$variant => $factor),+
                }
            }
        }

        impl FromStr for MassUnit {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.trim().to_lowercase().as_str() {
                    $($symbol | $unit_type | $unit_type_plural => Ok(MassUnit::$variant),)+
                    _ => Err("Unknown mass unit"),
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct Mass {
            value: f64,
            unit: MassUnit,
        }

        impl Mass {
            pub fn new(value: f64, unit: MassUnit) -> Self {
                Self { value, unit }
            }

            $(
                pub fn $from_fn_name(value: f64) -> Self {
                    Self::new(value, MassUnit::$variant)
                }
            )+

            $(
                pub fn $as_fn_name(&self) -> f64 {
                    self.value * self.unit.grams_factor() / $factor
                }
            )+

            pub fn to_unit(&self, unit: MassUnit) -> Self {
                let value = match unit {
                    $(MassUnit::$variant => self.$as_fn_name()),+
                };
                Self { value, unit }
            }

            $(
                pub fn $to_fn_name(&self) -> Self {
                    self.to_unit(MassUnit::$variant)
                }
            )+

            pub fn is_zero(&self) -> bool {
                self.value == 0.0
            }

            pub fn is_negative(&self) -> bool {
                self.value < 0.0
            }

            pub fn get_unit(&self) -> MassUnit {
                self.unit
            }

            pub fn get_symbol(&self) -> &'static str {
                self.unit.as_symbol()
            }

            pub fn get_unit_type(&self) -> &'static str {
                self.unit.as_unit_type()
            }

            pub fn get_unit_type_plural(&self) -> &'static str {
                self.unit.as_unit_type_plural()
            }

            pub fn to_string(&self) -> String {
                format!("{} {}", self.value, self.get_symbol())
            }

        }
    };
}

impl fmt::Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Add for Mass {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_g(self.as_g() + rhs.as_g())
    }
}

impl Sub for Mass {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_g(self.as_g() - rhs.as_g())
    }
}

impl Mul<f64> for Mass {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_g(self.as_g() * rhs)
    }
}

impl Div<f64> for Mass {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_g(self.as_g() / rhs)
    }
}

impl PartialOrd for Mass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_g().partial_cmp(&other.as_g())
    }
}

use mass_macro::include_masses_from_json;
include_masses_from_json!("data/mass.json");
