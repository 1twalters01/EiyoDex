use units::energy::unit::EnergyUnit;
use crate::units::error::UnitConversionError;

macro_rules! define_energy_units_c {
    ($($variant:ident),+) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub enum EnergyUnitC {
            $($variant),+
        }

        impl From<EnergyUnit> for EnergyUnitC {
            fn from(unit: EnergyUnit) -> Self {
                match unit {
                    $(EnergyUnit::$variant => EnergyUnitC::$variant),+
                }
            }
        }

        // Cannot trust input from FFI even if rust says it is ok
        #[derive(Debug, Copy, Clone)]
        pub enum EnergyUnitConversionError { InvalidValue }

        impl TryFrom<EnergyUnitC> for EnergyUnit {
            type Error = UnitConversionError;

            fn try_from(c_unit: EnergyUnitC) -> Result<Self, Self::Error> {
                #[allow(unreachable_patterns)]
                match c_unit {
                    $(EnergyUnitC::$variant => Ok(EnergyUnit::$variant)),+,
                    _ => Err(UnitConversionError::InvalidEnergyUnitValue),
                }
            }
        }
    }
}

use units_macro::include_energy_units_c_from_json;
include_energy_units_c_from_json!("data/units/energy");
