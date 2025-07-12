use std::ops::Mul;

use crate::{
    mass::Mass,
    volume::Volume,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Density {
    grams_per_ml: f64
}

impl Density {
    pub fn from_g_per_ml(g_per_ml: f64) -> Self {
        Self { grams_per_ml: g_per_ml }
    }

    pub fn from_g_per_l(g_per_l: f64) -> Self {
        g_per_ml = g_per_l / 1000;
        Self { grams_per_ml: g_per_ml }
    }

    pub fn as_g_per_ml(&self) -> Self {
        self.grams_per_ml
    }

    pub fn as_g_per_l(&self) -> Self{
        self.grams_per_ml * 1000
    }

    pub fn get_mass_for_volume(&self, volume: Volume) -> Mass {
        Mass::from_grams(self.grams_per_ml * volume.as_ml())
    }

    pub fn get_volume_for_mass(&self, mass: Mass) -> Mass {
        Volume::from_ml(mass.as_grams() / self.grams_per_ml)
    }
}

impl Mul<Density> for Volume {
    type Output = Mass;

    fn mul(self, density: Density) -> Mass {
        density.get_mass_for_volume(self)
    }
}