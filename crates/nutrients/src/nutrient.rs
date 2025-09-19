use uuid::Uuid;
use std::collections::{
    BTreeMap,
    HashSet,
};

use units::{
    mass::{Mass, MassUnit},
    volume::{Volume, VolumeUnit},
    energy::{Energy, EnergyUnit},
};

use crate::schema::nutrients::NutrientTypes;

pub struct Nutrient {
    id: Uuid,
    name: String,
    categories: HashSet<NutrientTypes>,
    parent: Vec<Uuid>,
    main_unit: Option<Unit>,
    accepted_units: Unit,
    unit_conversions: BTreeMap<(Unit, Unit), f64>
}

impl Nutrient {
    pub fn add_conversion(&mut self, from: Unit, to: Unit, factor: f64) -> Result<(), String> {
        if factor == 0 as f64 {
            return Err(String::from("Conversion factor may not be 0"))
        }

        let (from_unit, from_factor): (Unit, f64) = match from {
            Unit::Mass(mass_unit) => {
                let mass = Mass::new(1 as f64, mass_unit);
                (Unit::Mass(MassUnit::Gram), mass.as_g())
            },
            Unit::Volume(volume_unit) => {
                let volume = Volume::new(1 as f64, volume_unit);
                (Unit::Volume(VolumeUnit::Milliliter), volume.as_ml())
            },
            Unit::Energy(energy_unit) => {
                let energy = Energy::new(1 as f64, energy_unit);
                (Unit::Energy(EnergyUnit::Kcal), energy.as_kcal())
            },
            _ => (to, 1 as f64)
        };

        let (to_unit, to_factor): (Unit, f64) = match to {
            Unit::Mass(mass_unit) => {
                let mass = Mass::new(1 as f64, mass_unit);
                (Unit::Mass(MassUnit::Gram), mass.as_g())
            },
            Unit::Volume(volume_unit) => {
                let volume = Volume::new(1 as f64, volume_unit);
                (Unit::Volume(VolumeUnit::Milliliter), volume.as_ml())
            },
            Unit::Energy(energy_unit) => {
                let energy = Energy::new(1 as f64, energy_unit);
                (Unit::Energy(EnergyUnit::Kcal), energy.as_kcal())
            },
            _ => (to, 1 as f64)
        };

        let new_factor = factor * from_factor * to_factor;
        self.unit_conversions.insert((from_unit, to_unit), new_factor);
        self.unit_conversions.insert((to_unit, from_unit), 1.0 / new_factor);

        Ok(())
    }

    pub fn convert(&self, value: f64, from: Unit, to: Unit) -> Option<f64> {
        self.unit_conversions
            .get(&(from, to))
            .map(|factor| value * factor)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Unit { 
    Mass(MassUnit),
    Volume(VolumeUnit),
    Energy(EnergyUnit),
    IU, // International Unit
    DFE, // Dietary Folate Equivalent
    NE, // Niacin Equivalent
    RAE, // Retinol Activity Equivalent
    PDCAAS, // Protein Digestibility Corrected Amino Acid Score
    DIAAS1, // Digestible Indispensable Amino Acid Score 0 to 6 months
    DIAAS2, // Digestible Indispensable Amino Acid Score 6 months to 3 years
    DIAAS3, // Digestible Indispensable Amino Acid Score Over 3 years
}

