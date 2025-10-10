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
                    denominator_unit_variant: $json_duration_unit_variant:ident,
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
            mass::Mass,
            volume::Volume,
            density::Density,
        };

        #[derive(PartialEq)]
        pub enum Denominator {
            Mass,
            Volume,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum SpecificCurrencyUnit {
            $($all_variant),+
        }

        impl SpecificCurrencyUnit {
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

            pub fn get_denominator_unit_type(&self) -> Denominator {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => Denominator::$all_denominator_unit_type,)+
                }
            }

            pub fn get_currency_unit(&self) -> CurrencyUnit {
                match self {
                    $(SpecificCurrencyUnit::$all_variant => CurrencyUnit::$all_currency_unit_variant),+
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
                    if self.get_unit().get_denominator_unit_type() == Denominator::$all_denominator_unit_type {
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

            pub fn get_denominator_unit_type(&self) -> Denominator {
                self.unit.get_denominator_unit_type()
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

impl Mul<u64> for SpecificCurrency {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self {
        Self::new(self.get_value() * rhs as f64, self.unit)
    }
}

impl Mul<f64> for SpecificCurrency {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.get_value() * rhs, self.unit)
    }
}

// Make this use their currency, not just usd
impl Mul<Volume> for SpecificCurrency {
    type Output = Currency;

    fn mul(self, rhs: Volume) -> Currency {
        let ml = rhs.as_ml();
        let usd_per_ml = self
            .as_usd_per_ml()
            .expect("Cannot multiply volume based Specific Currency with mass");
        Currency::new(ml * usd_per_ml, CurrencyUnit::USD)
    }
}

impl Mul<Mass> for SpecificCurrency {
    type Output = Currency;

    fn mul(self, rhs: Mass) -> Currency {
        let kg = rhs.as_kg();
        let usd_per_kg = self
            .as_usd_per_kg()
            .expect("Cannot multiply mass based Specific Currency with volume");
        Currency::new(kg * usd_per_kg, CurrencyUnit::USD)
    }
}

impl Mul<Density> for SpecificCurrency {
    type Output = Currency;

    fn mul(self, rhs: Density) -> Currency {
        let kg_per_l = rhs.as_kg_per_l();
        let usd_per_kg = self
            .as_usd_per_kg()
            .expect("Cannot multiply volume based Specific Currency with density");
        Currency::new(kg_per_l * usd_per_kg, CurrencyUnit::USD)
    }
}

impl Mul<SpecificCurrency> for Density {
    type Output = Currency;

    fn mul(self, rhs: SpecificCurrency) -> Currency {
        let kg_per_l = self.as_kg_per_l();
        let usd_per_kg = rhs
            .as_usd_per_kg()
            .expect("Cannot multiply volume based Specific Currency with density");
        Currency::new(kg_per_l * usd_per_kg, CurrencyUnit::USD)
    }
}

impl Mul<SpecificCurrency> for Volume {
    type Output = Currency;

    fn mul(self, rhs: SpecificCurrency) -> Currency {
        let ml = self.as_ml();
        let usd_per_ml = rhs
            .as_usd_per_ml()
            .expect("Cannot multiply volume based Specific Currency with mass");
        Currency::new(ml * usd_per_ml, CurrencyUnit::USD)
    }
}

impl Mul<SpecificCurrency> for Mass {
    type Output = Currency;

    fn mul(self, rhs: SpecificCurrency) -> Currency {
        let kg = self.as_kg();
        let usd_per_kg = rhs
            .as_usd_per_kg()
            .expect("Cannot multiply mass based Specific Currency with volume");
        Currency::new(kg * usd_per_kg, CurrencyUnit::USD)
    }
}

impl Div<i64> for SpecificCurrency {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        Self::new(self.get_value() / rhs as f64, self.unit)
    }
}

impl Div<f64> for SpecificCurrency {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(self.get_value() / rhs, self.unit)
    }
}

impl Div<Volume> for Currency {
    type Output = SpecificCurrency;

    fn div(self, rhs: Volume) -> SpecificCurrency {
        let usd = self
            .convert_to_sync(CurrencyUnit::USD)
            .expect("Could not convert")
            .get_value();
        let ml = rhs.as_ml();
        SpecificCurrency::from_usd_per_ml(usd / ml)
    }
}

impl Div<Mass> for Currency {
    type Output = SpecificCurrency;

    fn div(self, rhs: Mass) -> SpecificCurrency {
        let usd = self
            .convert_to_sync(CurrencyUnit::USD)
            .expect("Could not convert")
            .get_value();
        let kg = rhs.as_kg();
        SpecificCurrency::from_usd_per_ml(usd / kg)
    }
}

impl Div<Density> for SpecificCurrency {
    type Output = SpecificCurrency;

    fn div(self, rhs: Density) -> SpecificCurrency {
        let usd_per_ml = self
            .as_usd_per_ml()
            .expect("Cannot multiply volume based Specific Currency with mass");
        let kg_per_ml = rhs.as_kg_per_ml();
        SpecificCurrency::new(usd_per_ml / kg_per_ml, SpecificCurrencyUnit::USDPerKilogram)
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
include_specific_currencies_from_json!(
    CurrencyUnit => "data/units/currency",
    VolumeUnit => "data/units/volume",
    MassUnit => "data/units/mass",
    SpecificCurrencyUnit => "data/units/specific_currency",
);
