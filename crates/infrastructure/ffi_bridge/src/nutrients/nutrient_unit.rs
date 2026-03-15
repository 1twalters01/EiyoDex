use crate::units::error::UnitConversionError;
use nutrients::nutrient_units::NutrientUnit;
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};

use crate::units::{energy::EnergyUnitC, mass::MassUnitC, volume::VolumeUnitC};

use std::fmt::Debug;

#[macro_export]
macro_rules! define_nutrient_units_c {
    ($($variant:ident),+) => {
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        pub enum NutrientUnitTag {
            Mass,
            Volume,
            Energy,
            $($variant),*
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        pub union NutrientUnitData {
            pub mass: MassUnitC,
            pub volume: VolumeUnitC,
            pub energy: EnergyUnitC,
        }

        #[repr(C)]
        #[derive(Copy, Clone)]
        pub struct NutrientUnitC {
            pub tag: NutrientUnitTag,
            pub data: NutrientUnitData,
        }

        impl From<NutrientUnit> for NutrientUnitC {
            fn from(unit: NutrientUnit) -> Self {
                match unit {
                    NutrientUnit::Mass(mass_type) => NutrientUnitC { tag: NutrientUnitTag::Mass, data: NutrientUnitData { mass: mass_type.into()} },
                    NutrientUnit::Volume(volume_type) => NutrientUnitC { tag: NutrientUnitTag::Volume, data: NutrientUnitData { volume: volume_type.into()} },
                    NutrientUnit::Energy(energy_type) => NutrientUnitC { tag: NutrientUnitTag::Energy, data: NutrientUnitData { energy: energy_type.into()} },
                    $(NutrientUnit::$variant => NutrientUnitC { tag: NutrientUnitTag::$variant, data: unsafe { std::mem::zeroed() } }),+
                }
            }
        }

        impl TryFrom<NutrientUnitC> for NutrientUnit {
            type Error = UnitConversionError;

            fn try_from(c_unit: NutrientUnitC) -> Result<Self, Self::Error> {
                match c_unit.tag {
                    NutrientUnitTag::Mass => {
                        let mass_unit_c = unsafe { c_unit.data.mass };
                        let mass_unit: MassUnit = mass_unit_c.try_into()?;
                        Ok(NutrientUnit::Mass(mass_unit))
                    },
                    NutrientUnitTag::Volume => {
                        let volume_unit_c = unsafe { c_unit.data.volume };
                        let volume_unit: VolumeUnit = volume_unit_c.try_into()?;
                        Ok(NutrientUnit::Volume(volume_unit))
                    },
                    NutrientUnitTag::Energy => {
                        let energy_unit_c = unsafe { c_unit.data.energy };
                        let energy_unit: EnergyUnit = energy_unit_c.try_into()?;
                        Ok(NutrientUnit::Energy(energy_unit))
                    },
                    $(NutrientUnitTag::$variant => Ok(NutrientUnit::$variant)),+
                }
            }
        }

        impl Debug for NutrientUnitC {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.tag {
                    NutrientUnitTag::Mass => unsafe { write!(f, "Mass({:?})", self.data.mass) },
                    NutrientUnitTag::Volume => unsafe { write!(f, "Volume({:?})", self.data.volume) },
                    NutrientUnitTag::Energy => unsafe { write!(f, "Energy({:?})", self.data.energy) },
                    tag => write!(f, "{:?}", tag),
                }
            }
        }
    }
}

use nutrients_macro::include_nutrient_units_c_from_json;
include_nutrient_units_c_from_json!("data/nutrients/nutrient_units");
