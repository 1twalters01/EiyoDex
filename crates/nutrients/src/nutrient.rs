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
    pub fn add_conversion(&mut self, from: Unit, to: Unit) -> Result((), String) {
    self.unit_conversions.insert((from, to), value);
    if factor != 0.0 {
        self.unit_conversions.insert((to, from), 1.0 / factor);
    }
    if let matches!(from, Mass) {
        self.unit_conversions.insert((from, to), value);
        if factor != 0.0 {
            self.unit_conversions.insert((to, from), 1.0 / factor);
        }
        self.unit_conversions.insert((from, to), value);
        if factor != 0.0 {
            self.unit_conversions.insert((to, from), 1.0 / factor);
        }
    }
    if let matches!(from, Volume) {
        self.unit_conversions.insert((from, to), value);
        if factor != 0.0 {
            self.unit_conversions.insert((to, from), 1.0 / factor);
        }
        self.unit_conversions.insert((from, to), value);
        if factor != 0.0 {
            self.unit_conversions.insert((to, from), 1.0 / factor);
        }
    }
    if let matches!(from, Energy) {
        self.unit_conversions.insert((from, to), value);
        if factor != 0.0 {
            self.unit_conversions.insert((to, from), 1.0 / factor);
        }
        self.unit_conversions.insert((from, to), value);
        if factor != 0.0 {
            self.unit_conversions.insert((to, from), 1.0 / factor);
        }
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