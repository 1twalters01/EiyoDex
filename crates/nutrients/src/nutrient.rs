use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    ops::{Div, Mul},
};
use uuid::Uuid;

use units::{
    energy::{Energy, EnergyUnit},
    mass::{Mass, MassUnit},
    volume::{Volume, VolumeUnit},
};

use crate::schema::nutrients::NutrientType;

#[derive(Clone)]
pub struct NutrientAmount {
    value: f64, // value for main unit is saved
    nutrient: Nutrient,
}

impl NutrientAmount {
    pub fn new(value: f64, nutrient: Nutrient, unit: Unit) -> Result<Self, String> {
        match nutrient.convert(value, unit, nutrient.main_unit) {
            Ok(value) => Ok(Self { value, nutrient }),
            Err(err) => Err(err),
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_nutrient(&self) -> &Nutrient {
        &self.nutrient
    }

    pub fn round(&self) -> Self {
        Self::new(
            self.get_value().round(),
            self.get_nutrient().clone(),
            self.get_nutrient().get_main_unit(),
        )
        .unwrap()
    }
}

impl Mul<f64> for NutrientAmount {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::new(
            self.get_value() * rhs,
            self.get_nutrient().clone(),
            self.get_nutrient().get_main_unit(),
        )
        .unwrap()
    }
}

impl Div<f64> for NutrientAmount {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::new(
            self.get_value() / rhs,
            self.get_nutrient().clone(),
            self.get_nutrient().get_main_unit(),
        )
        .unwrap()
    }
}

#[derive(Clone)]
pub struct Nutrient {
    id: Uuid,
    name: String,
    description: String,
    categories: HashSet<NutrientType>,
    parent: Vec<Uuid>,
    main_unit: Unit,
    accepted_units: BTreeSet<Unit>,
    unit_conversions: BTreeMap<(Unit, Unit), f64>,
}

impl Nutrient {
    pub fn new(id: Option<Uuid>, name: String, main_unit: Unit) -> Self {
        let id = id.unwrap_or_else(Uuid::new_v4);

        Nutrient {
            id,
            name: name,
            description: String::new(),
            categories: HashSet::new(),
            parent: Vec::new(),
            main_unit: main_unit,
            accepted_units: BTreeSet::from([main_unit]),
            unit_conversions: BTreeMap::new(),
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn remove_category(&mut self, category: NutrientType) {
        self.categories.remove(&category);
    }

    pub fn insert_category(&mut self, category: NutrientType) {
        self.categories.insert(category);
    }

    pub fn get_main_unit(&self) -> Unit {
        self.main_unit
    }

    pub fn set_main_unit(&mut self, main_unit: Unit) -> Result<(), String> {
        if self.accepted_units.contains(&main_unit) {
            self.main_unit = main_unit;
            Ok(())
        } else {
            Err(String::from("New main unit not in accepted units"))
        }
    }

    pub fn convert(&self, value: f64, from: Unit, to: Unit) -> Result<f64, String> {
        if self.accepted_units.contains(&to) && self.accepted_units.contains(&from) {
            match self.unit_conversions.get(&(from, to)) {
                Some(factor) => Ok(value * factor),
                None => Err(String::from("Conversion factor not found")),
            }
        } else {
            Err(String::from(
                "Conversion must use values accepted by nutrient",
            ))
        }
    }

    pub fn remove_conversion(&mut self, from: Unit, to: Unit) {
        self.unit_conversions.remove(&(from, to));

        let (mut found_from, mut found_to) = (false, false);
        for (key, _) in &self.unit_conversions {
            if key.0 == from || key.1 == from {
                found_from = true;
            }
            if key.0 == to || key.1 == to {
                found_to = true;
            }
        }

        if found_from == false {
            self.accepted_units.remove(&from);
        }
        if found_to == false {
            self.accepted_units.remove(&to);
        }
    }

    pub fn add_conversion(&mut self, from: Unit, to: Unit, factor: f64) -> Result<(), String> {
        if factor == 0 as f64 {
            return Err(String::from("Conversion factor may not equal 0"));
        }

        let (from_unit, from_factor): (Unit, f64) = match from {
            Unit::Mass(mass_unit) => {
                let _ = MassUnit::get_enumerations()
                    .iter()
                    .map(|mass_unit| self.accepted_units.insert(Unit::Mass(*mass_unit)));

                let mass = Mass::new(1 as f64, mass_unit);
                (Unit::Mass(MassUnit::Gram), mass.as_g())
            }
            Unit::Volume(volume_unit) => {
                let _ = VolumeUnit::get_enumerations()
                    .iter()
                    .map(|volume_unit| self.accepted_units.insert(Unit::Volume(*volume_unit)));

                let volume = Volume::new(1 as f64, volume_unit);
                (Unit::Volume(VolumeUnit::Milliliter), volume.as_ml())
            }
            Unit::Energy(energy_unit) => {
                let _ = EnergyUnit::get_enumerations()
                    .iter()
                    .map(|energy_unit| self.accepted_units.insert(Unit::Energy(*energy_unit)));

                let energy = Energy::new(1 as f64, energy_unit);
                (Unit::Energy(EnergyUnit::Kilocalorie), energy.as_kcal())
            }
            _ => {
                self.accepted_units.insert(from);
                (to, 1 as f64)
            }
        };

        let (to_unit, to_factor): (Unit, f64) = match to {
            Unit::Mass(mass_unit) => {
                let _ = MassUnit::get_enumerations()
                    .iter()
                    .map(|mass_unit| self.accepted_units.insert(Unit::Mass(*mass_unit)));

                let mass = Mass::new(1 as f64, mass_unit);
                (Unit::Mass(MassUnit::Gram), mass.as_g())
            }
            Unit::Volume(volume_unit) => {
                let _ = VolumeUnit::get_enumerations()
                    .iter()
                    .map(|volume_unit| self.accepted_units.insert(Unit::Volume(*volume_unit)));

                let volume = Volume::new(1 as f64, volume_unit);
                (Unit::Volume(VolumeUnit::Milliliter), volume.as_ml())
            }
            Unit::Energy(energy_unit) => {
                let _ = EnergyUnit::get_enumerations()
                    .iter()
                    .map(|energy_unit| self.accepted_units.insert(Unit::Energy(*energy_unit)));

                let energy = Energy::new(1 as f64, energy_unit);
                (Unit::Energy(EnergyUnit::Kilocalorie), energy.as_kcal())
            }
            _ => {
                self.accepted_units.insert(from);
                (to, 1 as f64)
            }
        };

        let new_factor = factor * from_factor * to_factor;
        self.unit_conversions
            .insert((from_unit, to_unit), new_factor);
        self.unit_conversions
            .insert((to_unit, from_unit), 1.0 / new_factor);

        Ok(())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Unit {
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
