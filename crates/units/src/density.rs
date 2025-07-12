use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt,
    ops::{Div, Mul},
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    mass::Mass,
    volume::Volume,
};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DensityUnit {
    GramsPerMl,
    GramsPerL,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Density {
    value: f64,
    unit: DensityUnit,
}

impl Default for Energy {
    fn default() -> Self {
        Density::from_g_per_ml(0.0)
    }
}

impl Density {
    pub fn new(value: f64, unit: DensityUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_g_per_ml(g_per_ml: f64) -> Self {
        Self::new(g_per_ml, DensityUnit::GramsPerMl)
    }

    pub fn from_g_per_l(g_per_l: f64) -> Self {
        Self::new(g_per_l, DensityUnit::GramsPerL)
    }

    pub fn as_g_per_ml(&self) -> Self {
        match self.unit {
            DensityUnit::GramsPerMl => self.value,
            DensityUnit::GramsPerL => self.value / 1000
        }
    }

    pub fn as_g_per_l(&self) -> f64 {
        match self.unit {
            DensityUnit::GramsPerMl => self.value * 1000,
            DensityUnit::GramsPerL => self.value,
        }
    }

    pub fn get_mass_for_volume(&self, volume: Volume) -> Mass {
        Mass::from_grams(self.grams_per_ml * volume.as_ml())
    }

    pub fn get_volume_for_mass(&self, mass: Mass) -> Mass {
        Volume::from_ml(mass.as_grams() / self.grams_per_ml)
    }

    pub fn to_unit(&self, unit: VolumeUnit) -> Self {
        let value = match unit {
            DensityUnit::GramsPerMl => self.as_l(),
            DensityUnit::GramsPerL => self.as_ml(),
        };
        Self { value, unit }
    }

    pub fn to_g_per_ml(&self) -> Self {
        self.to_unit(DensityUnit::GramsPerMl)
    }

    pub fn to_g_per_l(&self) -> Self {
        self.to_unit(DensityUnit::GramsPerL)
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0.0
    }

    pub fn is_negative(&self) -> bool {
        self.value < 0.0
    }

    pub fn get_unit(&self) -> DensityUnit {
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

    pub fn to_string(&self) -> Strint {
        format!("{} {}", self.value, self.get_symbol())
    }
}

impl Mul<Density> for Volume {
    type Output = Mass;

    fn mul(self, density: Density) -> Mass {
        density.get_mass_for_volume(self)
    }
}

impl Div<Density> for Mass {
    type Output = Density;

    fn div(self, volume: Volume) -> Density {
        grams = self.as_g();
        milliliters = volume.as_ml();
        grams_per_milliliter = grams / milliliters;
        density.from_g_per_ml(grams_per_milliliter)
    }
}

impl PartialOrd for Density {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_g_per_ml().partial_cmp(&other.as_g_per_ml())
    }
}

impl Ord for Energy {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}