#[macro_export]
macro_rules! define_masses_test {
    (
        $(
            $variant:ident => {
                from_fn_name: $from_fn_name:ident,
                as_fn_name: $as_fn_name:ident,
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

            // as_<symbol>
            paste::paste! {
                $(
                    pub fn as_fn_name(&self) -> f64 {
                        match self.unit {
                            // re-expand the whole unit list with fresh metavars
                            define_masses!(@arms [ $($variant => $factor),+ ] ; $factor)
                        }
                    }
                )+
            }
        }
    };

    // helper: expand all match arms against a given target factor
    (@arms [ $($v:ident => $f:expr),+ ] ; $target:expr) => {
        $(
            MassUnit::$v => self.value * $f / $target,
        )+
    };
}

