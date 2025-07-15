use std::{
    cmp::Ordering,
    fmt,
    ops::{Div, Mul},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{mass::Mass, volume::Volume};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DensityUnit {
    GramsPerMl,
    GramsPerL,
}

impl DensityUnit {
    pub fn as_symbol(&self) -> &'static str {
        match self {
            DensityUnit::GramsPerMl => "g/ml",
            DensityUnit::GramsPerL => "g/l",
        }
    }

    pub fn as_unit_type(&self) -> &'static str {
        match self {
            DensityUnit::GramsPerMl => "gram per milliliter",
            DensityUnit::GramsPerL => "gram per liter",
        }
    }

    pub fn as_unit_type_plural(&self) -> &'static str {
        match self {
            DensityUnit::GramsPerMl => "grams per milliliter",
            DensityUnit::GramsPerL => "grams per liter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Density {
    value: f64,
    unit: DensityUnit,
}

impl Default for Density {
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

    pub fn as_g_per_ml(&self) -> f64 {
        match self.unit {
            DensityUnit::GramsPerMl => self.value,
            DensityUnit::GramsPerL => self.value / 1000 as f64,
        }
    }

    pub fn as_g_per_l(&self) -> f64 {
        match self.unit {
            DensityUnit::GramsPerMl => self.value * 1000 as f64,
            DensityUnit::GramsPerL => self.value,
        }
    }

    pub fn get_mass_for_volume(&self, volume: Volume) -> Mass {
        Mass::from_g(self.as_g_per_ml() * volume.as_ml())
    }

    pub fn get_volume_for_mass(&self, mass: Mass) -> Volume {
        Volume::from_ml(mass.as_g() / self.as_g_per_ml())
    }

    pub fn to_unit(&self, unit: DensityUnit) -> Self {
        let value = match unit {
            DensityUnit::GramsPerMl => self.as_g_per_ml(),
            DensityUnit::GramsPerL => self.as_g_per_l(),
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

    pub fn to_string(&self) -> String {
        format!("{} {}", self.value, self.get_symbol())
    }
}

impl fmt::Display for Density {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.get_symbol())
    }
}

impl Mul<Volume> for Density {
    type Output = Mass;

    fn mul(self, volume: Volume) -> Mass {
        self.get_mass_for_volume(volume)
    }
}

impl Mul<Density> for Volume {
    type Output = Mass;

    fn mul(self, density: Density) -> Mass {
        density.get_mass_for_volume(self)
    }
}

impl Div<Volume> for Mass {
    type Output = Density;

    fn div(self, volume: Volume) -> Density {
        let grams = self.as_g();
        let milliliters = volume.as_ml();
        let grams_per_milliliter = grams / milliliters;
        Density::from_g_per_ml(grams_per_milliliter)
    }
}

impl PartialOrd for Density {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_g_per_ml().partial_cmp(&other.as_g_per_ml())
    }
}
