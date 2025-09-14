use uuid::Uuid;
use std::collections::BTreeMap;

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
    pub fn add_conversion(&mut self, from: Unit, to: Unit, factor: f64) -> Result((), String) {
        if factor == 0 {
            return Err(String::from("Conversion factor may not be 0"))
        }

        if let matches!(from, MassUnit) {
            let mass = Mass::new(1, from);
            let mass_g = mass.as_g();
            let mass_kg = mass.as_kg;
            let mass_mg = mass.as_mg;
            let mass_ug = mass.as_ug;
            
            let g_factor = factor * mass_g.unit;
            self.unit_conversions.insert((MassUnit::Gram, to), g_factor);
            self.unit_conversions.insert((to, from), 1.0 / g_factor);

            let kg_factor = factor * mass_kg.unit;
            self.unit_conversions.insert((from, to), kg_factor);
            self.unit_conversions.insert((to, from), 1.0 / kg_factor);
            
            let mg_factor = factor * mass_mg.unit;
            self.unit_conversions.insert((from, to), mg_factor);
            self.unit_conversions.insert((to, from), 1.0 / mg_factor);

            let ug_factor = factor * mass_ug.unit;
            self.unit_conversions.insert((from, to), ug_factor);
            self.unit_conversions.insert((to, from), 1.0 / ug_factor);
        }
        else if let matches!(from, VolumeUnit) {
            self.unit_conversions.insert((from, to), value);
            if factor != 0.0 {
                self.unit_conversions.insert((to, from), 1.0 / factor);
            }
            self.unit_conversions.insert((from, to), value);
            if factor != 0.0 {
                self.unit_conversions.insert((to, from), 1.0 / factor);
            }
        }
        else if let matches!(from, EnergyUnit) {
            self.unit_conversions.insert((from, to), value);
            if factor != 0.0 {
                self.unit_conversions.insert((to, from), 1.0 / factor);
            }
            self.unit_conversions.insert((from, to), value);
            if factor != 0.0 {
                self.unit_conversions.insert((to, from), 1.0 / factor);
            }
        }
        else {
            self.unit_conversions.insert((from, to), factor)
            self.unit_conversions.insert((to, from), 1.0 / factor)
        }
    }

    pub fn convert(&self, value: f64, from: Unit, to: Unit) -> Option<f64> {
        self.unit_conversions
            .get(&(from, to))
            .map(|factor| value * factor)
    }
}

pub enum Unit { // Make this just the unit rather than the 
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

pub enum MassUnit {
    Gram,
    Milligram,
    Kilogram,
    Microgram,
    Ounce,
}

pub enum VolumeUnit {
    Liter,
    Milliliter,
}

pub enum EnergyUnit {
    Kcal,
    KJ,
}