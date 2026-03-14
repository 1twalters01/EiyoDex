use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};

// Make this a macro instead

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
}

impl NutrientUnit {
    pub fn get_enumerations() -> Vec<Self> {
        let mass_enumerations: Vec<NutrientUnit> = MassUnit::get_enumerations().iter().map(|unit| NutrientUnit::Mass(*unit)).collect();
        let volume_enumerations: Vec<NutrientUnit> = VolumeUnit::get_enumerations().iter().map(|unit| NutrientUnit::Volume(*unit)).collect();
        let energy_enumerations:Vec<NutrientUnit> = EnergyUnit::get_enumerations().iter().map(|unit| NutrientUnit::Energy(*unit)).collect();

        let other_enumerations = Vec::from([
            Self::IU,
            Self::DFE,
            Self::NE,
            Self::RAE,
            Self::PDCAAS,
            Self::DIAAS1,
            Self::DIAAS2,
            Self::DIAAS3,
        ]);
        let mut enumerations = Vec::new();
        enumerations.extend(mass_enumerations);
        enumerations.extend(volume_enumerations);
        enumerations.extend(energy_enumerations);
        enumerations.extend(other_enumerations);
        return enumerations
    }

    pub fn si_factor(&self) -> Option<f64> {
        match self {
            Self::Mass(unit) => Some(unit.si_factor()),
            Self::Volume(unit) => Some(unit.si_factor()),
            Self::Energy(unit) => Some(unit.si_factor()),
            _ => None,
        }
    }
}
