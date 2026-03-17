use std::ops::{Mul};
use units::duration::quantity::DurationQuantity;

struct UVLight {
    uva: Irradiance, // W/m2
    uvb: Irradiance, // W/m2
}

struct UVDose {
    uva_dose: EnergyDensity, // J/m2
    uvb_dose: EnergyDensity, // J/m2
}

impl UVDose {
    pub fn erythema_dose(&self, uva_erythema_weight: f64) -> EnergyDensity {
        self.uvb_dose + self.uva_dose * uva_erythema_weight
    }

    // Need to derive efficiency and pass it in
    pub fn vitamin_d_production(&self, efficiency: f64) -> f64 {
        // Leave implementation for now
    }
}

impl Mul<DurationQuantity> for UVLight {
    type Output = UVDose;

    fn mul(self, rhs: DurationQuantity) -> Self::Output {
        UVDose {
            uva_dose: self.uva * rhs,
            uvb_dose: self.uvb * rhs,
        }
    }
}

struct VisibleLight {
    visible: Irradiance, // W/m2
}

struct VisibleDose {
    visible_dose: EnergyDensity,
}

impl Mul<DurationQuantity> for VisibleLight {
    type Output = VisibleDose;

    fn mul(self, rhs: DurationQuantity) -> Self::Output {
        VisibleDose {
            visible_dose: self.visible * rhs,
        }
    }
}

struct InfraredLight {
    infrared: Irradiance,
}

struct InfraredDose {
    infrared_dose: EnergyDensity,
}

impl Mul<DurationQuantity> for InfraredLight {
    type Output = InfraredDose;

    fn mul(self, rhs: DurationQuantity) -> Self::Output {
        InfraredDose {
            infrared_dose: self.infrared * rhs,
        }
    }
}

struct Sunlight {
    uv: UVLight, // vitamin d and burns
    visible: VisibleLight, // Circadian rhythm
    infrared: InfraredLight, // heat load / sweating
}

struct SunlightDose {
    uv_dose: UVDose,
    visible_dose: VisibleDose,
    infrared_dose: InfraredDose,
}

impl Mul<DurationQuantity> for Sunlight {
    type Output = SunlightDose;

    fn mul(self, rhs: DurationQuantity) -> Self::Output {
        SunlightDose {
            uv_dose: self.uv * rhs,
            visible_dose: self.visible * rhs,
            infrared_dose: self.infrared * rhs,
        }
    }
}

impl SunlightDose {
    // Sunburns
    pub fn erythema_dose(&self, uva_erythema_weight: f64) -> f64 {
        self.uv_dose.erythema_dose(uva_erythema_weight)
    }

    pub fn vitamin_d_production(&self, efficiency: f64) -> EnergyDensity {
        self.uv_dose.vitamin_d_production(efficiency)
    }
}
