use units::mass::unit::MassUnit;
use crate::units::error::UnitConversionError;

macro_rules! define_mass_units_c {
    ($($variant:ident),+) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub enum MassUnitC {
            $($variant),+
        }

        impl From<MassUnit> for MassUnitC {
            fn from(unit: MassUnit) -> Self {
                match unit {
                    $(MassUnit::$variant => MassUnitC::$variant),+
                }
            }
        }

        // Cannot trust input from FFI even if rust says it is ok
        impl TryFrom<MassUnitC> for MassUnit {
            type Error = UnitConversionError;

            fn try_from(c_unit: MassUnitC) -> Result<Self, Self::Error> {
                #[allow(unreachable_patterns)]
                match c_unit {
                    $(MassUnitC::$variant => Ok(MassUnit::$variant)),+,
                    _ => Err(UnitConversionError::InvalidMassUnitValue),
                }
            }
        }
    }
}

use units_macro::include_mass_units_c_from_json;
include_mass_units_c_from_json!("data/units/mass");
