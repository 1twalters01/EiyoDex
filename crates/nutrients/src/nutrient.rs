use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashSet},
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};
use uuid::Uuid;

use units::{
    energy::{Energy, EnergyUnit},
    mass::{Mass, MassUnit},
    volume::{Volume, VolumeUnit},
};

use crate::schema::nutrients::NutrientType;

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientAmount {
    value: f64,
    nutrient: Rc<RefCell<Nutrient>>,
}

impl NutrientAmount {
    pub fn new(value: f64, nutrient: Nutrient, unit: Unit) -> Result<Self, String> {
        match nutrient.convert(value, unit, nutrient.main_unit) {
            Ok(conversion_factor) => Ok(Self {
                value: value * conversion_factor,
                nutrient: Rc::new(RefCell::new(nutrient)),
            }),
            Err(err) => Err(err),
        }
    }

    pub fn from_rc_refcell(
        value: f64,
        nutrient: Rc<RefCell<Nutrient>>,
        unit: Unit,
    ) -> Result<Self, String> {
        let conversion_factor = {
            let n = nutrient.borrow();
            n.convert(value, unit, n.main_unit)?
        };
        Ok(Self {
            value: value * conversion_factor,
            nutrient: nutrient,
        })
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_nutrient(&self) -> Rc<RefCell<Nutrient>> {
        self.nutrient.clone()
    }

    pub fn set_nutrient(&mut self, nutrient: Nutrient) {
        self.nutrient = Rc::new(RefCell::new(nutrient));
    }

    pub fn round(&mut self, dp: u8) -> Self {
        let factor = 10f64.powi(dp as i32);
        self.value = (self.value * factor).round() / factor;
        return self.clone();
    }

    pub fn convert(&self, unit: Unit) -> Result<f64, String> {
        let n = self.nutrient.borrow();
        n.convert(self.get_value(), n.get_main_unit(), unit)
    }
}

impl PartialOrd for NutrientAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let n = self.nutrient.borrow();
        self.get_value()
            .partial_cmp(&other.convert(n.get_main_unit()).unwrap())
    }
}

impl Eq for NutrientAmount {}

impl Ord for NutrientAmount {
    fn cmp(&self, other: &Self) -> Ordering {
        let id_cmp = self
            .nutrient
            .borrow()
            .name
            .cmp(&other.nutrient.borrow().name);
        if id_cmp != Ordering::Equal {
            return id_cmp;
        }

        self.value
            .partial_cmp(&other.value)
            .unwrap_or(Ordering::Equal)
    }
}

impl Add for NutrientAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.nutrient != rhs.nutrient {
            panic!(
                "Tried to add different nutrients: {:#?} + {:#?}",
                self.nutrient, rhs.nutrient
            );
        }
        Self {
            value: self.value + rhs.value,
            nutrient: self.nutrient,
        }
    }
}

impl Sub for NutrientAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.nutrient != rhs.nutrient {
            panic!(
                "Tried to add different nutrients: {:#?} + {:#?}",
                self.nutrient, rhs.nutrient
            );
        }
        Self {
            value: self.value - rhs.value,
            nutrient: self.nutrient,
        }
    }
}

impl Mul<f64> for NutrientAmount {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let n = self.nutrient.borrow();
        let main_unit = n.get_main_unit();
        Self::from_rc_refcell(self.value * rhs, self.get_nutrient(), main_unit).unwrap()
    }
}

impl Div<f64> for NutrientAmount {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let n = self.nutrient.borrow();
        let main_unit = n.get_main_unit();
        Self::from_rc_refcell(self.get_value() / rhs, self.get_nutrient(), main_unit).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nutrient {
    id: Uuid,
    name: String,
    description: String,
    categories: HashSet<NutrientType>,
    main_unit: Unit,
    accepted_units: BTreeSet<Unit>,
    unit_conversions: BTreeMap<(Unit, Unit), f64>,
    parent_uuids: Vec<Uuid>,
    child_uuids: Vec<Uuid>,
}

impl Nutrient {
    pub fn new(id: Option<Uuid>, name: String, main_unit: Unit) -> Self {
        let id = id.unwrap_or_else(Uuid::new_v4);

        Nutrient {
            id,
            name,
            description: String::new(),
            categories: HashSet::new(),
            main_unit,
            accepted_units: BTreeSet::from([main_unit]),
            unit_conversions: BTreeMap::new(),
            parent_uuids: Vec::new(),
            child_uuids: Vec::new(),
        }
    }

    pub fn new_rc_refcell(id: Option<Uuid>, name: String, main_unit: Unit) -> Rc<RefCell<Self>> {
        let id = id.unwrap_or_else(Uuid::new_v4);

        Rc::new(RefCell::new(Nutrient {
            id,
            name,
            description: String::new(),
            categories: HashSet::new(),
            main_unit,
            accepted_units: BTreeSet::from([main_unit]),
            unit_conversions: BTreeMap::new(),
            parent_uuids: Vec::new(),
            child_uuids: Vec::new(),
        }))
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_categories(&self) -> HashSet<NutrientType> {
        self.categories.clone()
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
            return Ok(());
        }

        match self.main_unit {
            Unit::Mass(_) => {
                if let Unit::Mass(_) = main_unit {
                    self.accepted_units.remove(&self.main_unit);
                    self.accepted_units.insert(main_unit);
                    self.main_unit = main_unit;
                    return Ok(());
                }
            }
            Unit::Volume(_) => {
                if let Unit::Volume(_) = main_unit {
                    self.accepted_units.remove(&self.main_unit);
                    self.accepted_units.insert(main_unit);
                    self.main_unit = main_unit;
                    return Ok(());
                }
            }
            Unit::Energy(_) => {
                if let Unit::Energy(_) = main_unit {
                    self.accepted_units.remove(&self.main_unit);
                    self.accepted_units.insert(main_unit);
                    self.main_unit = main_unit;
                    return Ok(());
                }
            }
            _ => return Err(String::from("New main unit not in accepted units")),
        }

        Err(String::from("New main unit not in accepted units"))
    }

    pub fn get_accepted_units(&self) -> BTreeSet<Unit> {
        let mut accepted_units = self.accepted_units.clone();
        let mut mass_added = false;
        let mut volume_added = false;
        let mut energy_added = false;
        for unit in accepted_units.clone() {
            match unit {
                Unit::Mass(_) => {
                    if !mass_added {
                        accepted_units.extend(
                            MassUnit::get_enumerations()
                                .iter()
                                .map(|unit| Unit::Mass(*unit)),
                        );
                        mass_added = true;
                    }
                }
                Unit::Volume(_) => {
                    if !volume_added {
                        accepted_units.extend(
                            VolumeUnit::get_enumerations()
                                .iter()
                                .map(|unit| Unit::Volume(*unit)),
                        );
                        volume_added = true;
                    }
                }
                Unit::Energy(_) => {
                    if !energy_added {
                        accepted_units.extend(
                            EnergyUnit::get_enumerations()
                                .iter()
                                .map(|unit| Unit::Energy(*unit)),
                        );
                        energy_added = true;
                    }
                }
                _ => {}
            }
        }
        accepted_units
    }

    pub fn convert(&self, value: f64, from: Unit, to: Unit) -> Result<f64, String> {
        if from == to {
            return Ok(1f64);
        }

        match from {
            Unit::Mass(from_mass) => {
                if let Unit::Mass(to_mass) = to {
                    println!("{}", from_mass.si_factor() / to_mass.si_factor());
                    return Ok(from_mass.si_factor() / to_mass.si_factor());
                }
            }
            Unit::Volume(from_volume) => {
                if let Unit::Volume(to_volume) = to {
                    return Ok(from_volume.si_factor() / to_volume.si_factor());
                }
            }
            Unit::Energy(from_energy) => {
                if let Unit::Energy(to_energy) = to {
                    return Ok(from_energy.si_factor() / to_energy.si_factor());
                }
            }
            _ => {}
        }

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

    pub fn add_parent(&self, parent_uuid: Uuid) {
        todo!()
    }

    pub fn remove_parent(&self, parent_uuid: Uuid) {
        todo!()
    }

    pub fn add_child(&self, parent_uuid: Uuid) {
        todo!()
    }

    pub fn remove_child(&self, parent_uuid: Uuid) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
