use crate::units::error::UnitConversionError;
use units::volume::unit::VolumeUnit;

macro_rules! define_volume_units_c {
    ($($variant:ident),+) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        pub enum VolumeUnitC {
            $($variant),+
        }

        impl From<VolumeUnit> for VolumeUnitC {
            fn from(unit: VolumeUnit) -> Self {
                match unit {
                    $(VolumeUnit::$variant => VolumeUnitC::$variant),+
                }
            }
        }

        // Cannot trust input from FFI even if rust says it is ok
        impl TryFrom<VolumeUnitC> for VolumeUnit {
            type Error = UnitConversionError;

            fn try_from(c_unit: VolumeUnitC) -> Result<Self, Self::Error> {
                #[allow(unreachable_patterns)]
                match c_unit {
                    $(VolumeUnitC::$variant => Ok(VolumeUnit::$variant)),+,
                    _ => Err(UnitConversionError::InvalidVolumeUnitValue),
                }
            }
        }
    }
}

use units_macro::include_volume_units_c_from_json;
include_volume_units_c_from_json!("data/units/volume");
