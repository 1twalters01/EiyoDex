use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::{Rc, Weak},
};
use uuid::Uuid;

use units::{energy_unit::EnergyUnit, mass_unit::MassUnit, volume_unit::VolumeUnit};

use crate::{schema::nutrient_type::NutrientType, units::NutrientUnit};

#[derive(Debug, Clone)]
pub struct Nutrient {
    id: Uuid,
    name: String,
    description: String,
    nutrient_type: NutrientType,
    main_unit: NutrientUnit,
    unit_conversions: BTreeMap<NutrientUnit, f64>, // 1 unit = factor * main_unit
    parents: Vec<Weak<RefCell<Nutrient>>>,
    children: Vec<Rc<RefCell<Nutrient>>>,
}

impl PartialEq for Nutrient {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.description == other.description
            && self.nutrient_type == other.nutrient_type
            && self.main_unit == other.main_unit
            && self.unit_conversions == other.unit_conversions
    }
}

impl Nutrient {
    pub fn new(
        id: Option<Uuid>,
        name: String,
        nutrient_type: NutrientType,
        main_unit: NutrientUnit,
    ) -> Self {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let unit_conversions: BTreeMap<NutrientUnit, f64> = match main_unit {
            NutrientUnit::Mass(mass_unit) => {
                let mass_enums = MassUnit::get_enumerations();
                let unit_conversions: BTreeMap<NutrientUnit, f64> = mass_enums
                    .iter()
                    .map(|mass_enum| {
                        (
                            NutrientUnit::Mass(*mass_enum),
                            mass_enum.si_factor() / mass_unit.si_factor(),
                        )
                    })
                    .collect();
                unit_conversions
            }
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeMap<NutrientUnit, f64> = volume_enums
                    .iter()
                    .map(|volume_enum| {
                        (
                            NutrientUnit::Volume(*volume_enum),
                            volume_enum.si_factor() / volume_unit.si_factor(),
                        )
                    })
                    .collect();
                unit_conversions
            }
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeMap<NutrientUnit, f64> = energy_enums
                    .iter()
                    .map(|energy_enum| {
                        (
                            NutrientUnit::Energy(*energy_enum),
                            energy_enum.si_factor() / energy_unit.si_factor(),
                        )
                    })
                    .collect();
                unit_conversions
            }
            _ => {
                let unit_conversions = BTreeMap::from([(main_unit, 1f64)]);
                unit_conversions
            }
        };

        Nutrient {
            id,
            name,
            description: String::new(),
            nutrient_type: nutrient_type,
            main_unit,
            unit_conversions: unit_conversions,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn new_rc_refcell(
        id: Option<Uuid>,
        name: String,
        nutrient_type: NutrientType,
        main_unit: NutrientUnit,
    ) -> Rc<RefCell<Self>> {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let unit_conversions: BTreeMap<NutrientUnit, f64> = match main_unit {
            NutrientUnit::Mass(mass_unit) => {
                let mass_enums = MassUnit::get_enumerations();
                let unit_conversions: BTreeMap<NutrientUnit, f64> = mass_enums
                    .iter()
                    .map(|mass_enum| {
                        (
                            NutrientUnit::Mass(*mass_enum),
                            mass_enum.si_factor() / mass_unit.si_factor(),
                        )
                    })
                    .collect();
                unit_conversions
            }
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeMap<NutrientUnit, f64> = volume_enums
                    .iter()
                    .map(|volume_enum| {
                        (
                            NutrientUnit::Volume(*volume_enum),
                            volume_enum.si_factor() / volume_unit.si_factor(),
                        )
                    })
                    .collect();
                unit_conversions
            }
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeMap<NutrientUnit, f64> = energy_enums
                    .iter()
                    .map(|energy_enum| {
                        (
                            NutrientUnit::Energy(*energy_enum),
                            energy_enum.si_factor() / energy_unit.si_factor(),
                        )
                    })
                    .collect();
                unit_conversions
            }
            _ => {
                let unit_conversions = BTreeMap::from([(main_unit, 1f64)]);
                unit_conversions
            }
        };

        Rc::new(RefCell::new(Nutrient {
            id,
            name,
            description: String::new(),
            nutrient_type: nutrient_type,
            main_unit,
            unit_conversions: unit_conversions,
            parents: Vec::new(),
            children: Vec::new(),
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

    pub fn get_nutrient_type(&self) -> NutrientType {
        self.nutrient_type.clone()
    }

    pub fn set_nutrient_type(&mut self, nutrient_type: NutrientType) {
        self.nutrient_type = nutrient_type;
    }

    pub fn get_main_unit(&self) -> NutrientUnit {
        self.main_unit
    }

    pub fn set_main_unit(&mut self, main_unit: NutrientUnit) -> Result<(), &'static str> {
        if self.main_unit == main_unit {
            return Ok(());
        }

        self.update_accepted_units(main_unit)?;
        let accepted_units = self.get_accepted_units();
        if accepted_units.contains(&main_unit) {
            self.main_unit = main_unit;
            return Ok(());
        }

        Err("Cannot convert to selected unit")
    }

    pub fn get_accepted_units(&self) -> BTreeSet<NutrientUnit> {
        // self.update_accepted_units(self.main_unit);
        self.unit_conversions.keys().cloned().collect()
    }

    pub fn update_accepted_units(
        &mut self,
        new_main_unit: NutrientUnit,
    ) -> Result<(), &'static str> {
        let new_factor = if self.main_unit == new_main_unit {
            1f64
        } else {
            match self.unit_conversions.get(&new_main_unit) {
                Some(factor) => 1f64 / *factor,
                None => return Err("factor not found"),
            }
        };

        let accepted_units = self.get_accepted_units();
        let mut mass_units = Vec::new();
        let mut volume_units = Vec::new();
        let mut energy_units = Vec::new();
        let mut other_units = Vec::new();

        for unit in &accepted_units {
            match unit {
                NutrientUnit::Mass(mass_unit) => mass_units.push(NutrientUnit::Mass(*mass_unit)),
                NutrientUnit::Volume(volume_unit) => {
                    volume_units.push(NutrientUnit::Volume(*volume_unit))
                }
                NutrientUnit::Energy(energy_unit) => {
                    energy_units.push(NutrientUnit::Energy(*energy_unit))
                }
                other => other_units.push(other),
            }
        }

        if !mass_units.is_empty() {
            let all_mass_units = MassUnit::get_enumerations()
                .iter()
                .map(|mass_enum| NutrientUnit::Mass(*mass_enum))
                .collect::<BTreeSet<NutrientUnit>>();

            let (included_mass_units, unincluded_mass_units): (
                Vec<NutrientUnit>,
                Vec<NutrientUnit>,
            ) = all_mass_units
                .into_iter()
                .partition(|unit| accepted_units.contains(unit));

            let initial_mass_unit = match included_mass_units.first() {
                Some(unit) => unit,
                None => return Err("No mass unit despite containing mass"),
            };

            for unit in unincluded_mass_units {
                let initial_conversion_factor: f64 =
                    match self.unit_conversions.get(initial_mass_unit) {
                        Some(factor) => *factor,
                        None => panic!("factor not found"),
                    };

                let factor = match unit.si_factor().zip(initial_mass_unit.si_factor()).map(
                    |(unit, initial)| (unit / initial) * initial_conversion_factor * new_factor,
                ) {
                    Some(factor) => factor,
                    None => return Err("factor not found"),
                };
                self.unit_conversions.insert(unit, factor);
            }
        }

        if !volume_units.is_empty() {
            let all_volume_units = VolumeUnit::get_enumerations()
                .iter()
                .map(|volume_enum| NutrientUnit::Volume(*volume_enum))
                .collect::<BTreeSet<NutrientUnit>>();

            let (included_volume_units, unincluded_volume_units): (
                Vec<NutrientUnit>,
                Vec<NutrientUnit>,
            ) = all_volume_units
                .into_iter()
                .partition(|unit| accepted_units.contains(unit));

            let initial_volume_unit = match included_volume_units.first() {
                Some(unit) => unit,
                None => return Err("No volume unit despite containing volume"),
            };

            for unit in unincluded_volume_units {
                let initial_conversion_factor: f64 =
                    match self.unit_conversions.get(initial_volume_unit) {
                        Some(factor) => *factor,
                        None => return Err("factor not found"),
                    };

                let factor = match unit.si_factor().zip(initial_volume_unit.si_factor()).map(
                    |(unit, initial)| (unit / initial) * initial_conversion_factor * new_factor,
                ) {
                    Some(factor) => factor,
                    None => return Err("factor not found"),
                };
                self.unit_conversions.insert(unit, factor);
            }
        }

        if !energy_units.is_empty() {
            let all_energy_units = EnergyUnit::get_enumerations()
                .iter()
                .map(|energy_enum| NutrientUnit::Energy(*energy_enum))
                .collect::<BTreeSet<NutrientUnit>>();

            let (included_energy_units, unincluded_mass_units): (
                Vec<NutrientUnit>,
                Vec<NutrientUnit>,
            ) = all_energy_units
                .into_iter()
                .partition(|unit| accepted_units.contains(unit));

            let initial_energy_unit = match included_energy_units.first() {
                Some(unit) => unit,
                None => return Err("No mass unit despite containing mass"),
            };

            for unit in unincluded_mass_units {
                let initial_conversion_factor: f64 =
                    match self.unit_conversions.get(initial_energy_unit) {
                        Some(factor) => *factor,
                        None => return Err("factor not found"),
                    };

                let factor = match unit.si_factor().zip(initial_energy_unit.si_factor()).map(
                    |(unit, initial)| (unit / initial) * initial_conversion_factor * new_factor,
                ) {
                    Some(factor) => factor,
                    None => return Err("factor not found"),
                };
                self.unit_conversions.insert(unit, factor);
            }
        }

        for unit in accepted_units.clone() {
            if self.main_unit != new_main_unit {
                let conversion_factor: f64 = match self.unit_conversions.get(&unit) {
                    Some(factor) => *factor,
                    None => return Err("factor not found"),
                };
                let new_conversion_factor = conversion_factor * new_factor;
                self.unit_conversions.insert(unit, new_conversion_factor);
            }
        }

        return Ok(());
    }

    pub fn get_conversions(&self) -> BTreeMap<NutrientUnit, f64> {
        self.unit_conversions.clone()
    }

    pub fn convert(&self, from: NutrientUnit, to: NutrientUnit) -> Result<f64, &'static str> {
        if from == to {
            return Ok(1f64);
        }

        match from {
            NutrientUnit::Mass(from_mass) => {
                if let NutrientUnit::Mass(to_mass) = to {
                    println!("{}", from_mass.si_factor() / to_mass.si_factor());
                    return Ok(from_mass.si_factor() / to_mass.si_factor());
                }
            }
            NutrientUnit::Volume(from_volume) => {
                if let NutrientUnit::Volume(to_volume) = to {
                    return Ok(from_volume.si_factor() / to_volume.si_factor());
                }
            }
            NutrientUnit::Energy(from_energy) => {
                if let NutrientUnit::Energy(to_energy) = to {
                    return Ok(from_energy.si_factor() / to_energy.si_factor());
                }
            }
            _ => {}
        }

        let accepted_units = self.get_accepted_units();
        if accepted_units.contains(&to) && accepted_units.contains(&from) {
            let to_factor = match self.unit_conversions.get(&to) {
                Some(factor) => *factor,
                None => return Err("To conversion factor not found"),
            };
            let from_factor = match self.unit_conversions.get(&from) {
                Some(factor) => *factor,
                None => return Err("From conversion factor not found"),
            };
            let factor = to_factor * from_factor;
            return Ok(factor);
        } else {
            return Err("Conversion must use values accepted by nutrient");
        }
    }

    pub fn remove_conversion(&mut self, unit: NutrientUnit) -> Result<(), &'static str> {
        if unit == self.main_unit {
            return Err("Cannot remove a conversion if it is the main unit");
        }

        self.unit_conversions.remove(&unit);

        match unit {
            NutrientUnit::Mass(_) => {
                let all_masses = MassUnit::get_enumerations();
                all_masses.iter().for_each(|mass_unit| {
                    self.unit_conversions
                        .remove(&NutrientUnit::Mass(*mass_unit));
                });
            }
            NutrientUnit::Volume(_) => {
                let all_volumes = VolumeUnit::get_enumerations();
                all_volumes.iter().for_each(|volume_unit| {
                    self.unit_conversions
                        .remove(&NutrientUnit::Volume(*volume_unit));
                });
            }
            NutrientUnit::Energy(_) => {
                let all_energies = EnergyUnit::get_enumerations();
                all_energies.iter().for_each(|energy_unit| {
                    self.unit_conversions
                        .remove(&NutrientUnit::Energy(*energy_unit));
                });
            }
            other_unit => {
                self.unit_conversions.remove(&other_unit);
            }
        }

        return Ok(());
    }

    pub fn add_conversion(
        &mut self,
        from_unit: NutrientUnit,
        to_unit: NutrientUnit,
        factor: f64,
    ) -> Result<(), &'static str> {
        let accepted_units = self.get_accepted_units();

        if !accepted_units.contains(&to_unit) && !accepted_units.contains(&from_unit) {
            return Err("At least one unit must be an accepted unit");
        }

        if accepted_units.contains(&to_unit) && accepted_units.contains(&from_unit) {
            return Err("At least one unit must be a new unit");
        }

        let new_unit = if !accepted_units.contains(&to_unit) {
            to_unit
        } else {
            from_unit
        };

        let to_conversion_factor = match self.convert(to_unit, self.main_unit) {
            Ok(factor) => Some(factor),
            Err(_) => None,
        };

        let from_conversion_factor = match self.convert(self.main_unit, from_unit) {
            Ok(factor) => Some(factor),
            Err(_) => None,
        };

        if to_conversion_factor.is_none() && from_conversion_factor.is_none() {
            return Err("Both conversions are none");
        }

        let conversion_factor =
            to_conversion_factor.unwrap_or(1f64) * from_conversion_factor.unwrap_or(1f64);

        self.unit_conversions
            .insert(new_unit, factor * conversion_factor);

        self.update_accepted_units(self.main_unit)?;

        Ok(())
    }

    pub fn get_parents(&self) -> Vec<Weak<RefCell<Nutrient>>> {
        self.parents.clone()
    }

    pub fn get_ancestors(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        let mut result = Vec::new();

        for parent_weak in self.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                result.push(parent.clone());
                Self::collect_ancestors(&parent, &mut result);
            }
        }

        result
    }

    pub fn collect_ancestors(node: &Rc<RefCell<Self>>, out: &mut Vec<Rc<RefCell<Self>>>) {
        let parents = {
            let n = node.borrow();
            n.parents.clone()
        };

        for parent_weak in parents {
            if let Some(parent) = parent_weak.upgrade() {
                out.push(parent.clone());
                Self::collect_ancestors(&parent, out);
            }
        }
    }

    pub fn set_parents(&mut self, parents: Vec<Weak<RefCell<Nutrient>>>) {
        self.parents = parents;
    }

    pub fn add_parent(&mut self, parent: Rc<RefCell<Nutrient>>) {
        let parent_weak = Rc::downgrade(&parent);
        self.parents.push(parent_weak);
    }

    pub fn add_parent_weak(&mut self, parent: Weak<RefCell<Nutrient>>) {
        self.parents.push(parent);
    }

    pub fn remove_parent(&mut self, target: Weak<RefCell<Nutrient>>) {
        let Some(target_rc) = target.upgrade() else {
            return;
        };

        self.parents.retain(|parent| {
            parent
                .upgrade()
                .map(|p| !Rc::ptr_eq(&p, &target_rc))
                .unwrap_or(false) // remove dead parents
        });
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        self.children.clone()
    }

    pub fn get_descendants(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        let mut result = Vec::new();

        for child in self.get_children() {
            result.push(child.clone());
            Self::collect_descendants(&child, &mut result);
        }

        result
    }

    pub fn collect_descendants(node: &Rc<RefCell<Self>>, out: &mut Vec<Rc<RefCell<Self>>>) {
        let children = {
            let n = node.borrow();
            n.children.clone()
        };

        for child in children {
            out.push(child.clone());
            Self::collect_descendants(&child, out);
        }
    }

    pub fn set_children(&mut self, children: Vec<Rc<RefCell<Nutrient>>>) {
        self.children = children;
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Nutrient>>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, target: Rc<RefCell<Nutrient>>) {
        self.children.retain(|child| !Rc::ptr_eq(child, &target));
    }
}

pub fn link_parent_child(parent: &Rc<RefCell<Nutrient>>, child: &Rc<RefCell<Nutrient>>) {
    // Add child to parent's children (strong reference)
    if !parent
        .borrow()
        .children
        .iter()
        .any(|c| Rc::ptr_eq(c, child))
    {
        parent.borrow_mut().children.push(Rc::clone(child));
    }

    // Add parent to child's parents (weak reference)
    if !child.borrow().parents.iter().any(|p| {
        p.upgrade()
            .map(|p_rc| Rc::ptr_eq(&p_rc, parent))
            .unwrap_or(false)
    }) {
        child.borrow_mut().parents.push(Rc::downgrade(parent));
    }
}
