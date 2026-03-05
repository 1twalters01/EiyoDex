use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NutrientUnit {
    Mass(MassUnit),
    Volume(VolumeUnit),
    Energy(EnergyUnit),
    IU,     // International Unit
    DFE,    // Dietary Folate Equivalent
    NE,     // Niacin Equivalent
    RAE,    // Retinol Activity Equivalent
    PDCAAS, // Protein Digestibility Corrected Amino Acid Score
    DIAAS1, // Digestible Indispensable Amino Acid Score 0 to 6 months
    DIAAS2, // Digestible Indispensable Amino Acid Score 6 months to 3 years
    DIAAS3, // Digestible Indispensable Amino Acid Score Over 3 years
    None,   // None
}

impl NutrientUnit {
    pub fn si_factor(&self) -> Option<f64> {
        match self {
            Self::Mass(unit) => Some(unit.si_factor()),
            Self::Volume(unit) => Some(unit.si_factor()),
            Self::Energy(unit) => Some(unit.si_factor()),
            _ => None,
        }
    }
}
