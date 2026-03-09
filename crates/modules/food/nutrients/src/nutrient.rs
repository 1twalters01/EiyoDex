use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashSet},
    rc::{Rc, Weak},
};
use uuid::Uuid;

use units::{energy::{quantity::EnergyQuantity, unit::EnergyUnit}, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::dsa::{dfs::DFSTrait, node::GraphNode};

use crate::{
    nutrient_units::NutrientUnit,
    schema::{
        energy::EnergyYieldingNutrients, nutrient_classes::{ChemicalType, QuantityType}, nutrient_type::NutrientType
    },
};

#[derive(Debug, Clone)]
pub struct Nutrient {
    id: Uuid,
    name: String,
    description: String,
    nutrient_type: NutrientType,
    unit_conversions: BTreeMap<NutrientUnit, f64>, // 1 unit = factor * main_unit
    main_unit: NutrientUnit,

    parents: Vec<Weak<RefCell<Nutrient>>>,
    children: Vec<Rc<RefCell<Nutrient>>>,
}

impl GraphNode for Nutrient {
    fn get_parents(&self) -> Vec<Weak<RefCell<Nutrient>>> {
        self.parents.clone()
    }

    fn get_children(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        self.children.clone()
    }
}

impl DFSTrait for Nutrient {}

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
    pub fn default() -> Self {
        let chemical_type = ChemicalType::Other;
        let quantity_type = QuantityType::NonNutrient;
        let essentiality_type = None;

        let nutrient_type = NutrientType::new(chemical_type, quantity_type, essentiality_type);
        Nutrient {
            id: Uuid::nil(),
            name: String::new(),
            description: String::new(),
            nutrient_type: nutrient_type,
            unit_conversions: BTreeMap::new(),
            main_unit: NutrientUnit::None,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

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

    pub fn get_unit_conversions(&self) -> BTreeMap<NutrientUnit, f64> {
        self.unit_conversions.clone()
    }

    pub fn set_unit_conversions(&mut self, unit_conversions: BTreeMap<NutrientUnit, f64>) {
        self.unit_conversions = unit_conversions;
    }

    pub fn insert_unit_conversion(&mut self, unit: NutrientUnit, value: f64) {
        // let value = 1f64 / value;
        println!("value {:?}", value);
        match unit {
            NutrientUnit::Mass(input_mass) => {
                let all_masses = MassUnit::get_enumerations();
                all_masses.iter().for_each(|mass_unit| {
                    self.unit_conversions.insert(
                        NutrientUnit::Mass(*mass_unit),
                        value * mass_unit.si_factor() / input_mass.si_factor(),
                    );
                });
            }
            NutrientUnit::Volume(input_volume) => {
                let all_volumes = VolumeUnit::get_enumerations();
                all_volumes.iter().for_each(|volume_unit| {
                    println!(
                        "v: {:#?} {:#?}",
                        value * input_volume.si_factor() / volume_unit.si_factor(),
                        volume_unit
                    );
                    self.unit_conversions.insert(
                        NutrientUnit::Volume(*volume_unit),
                        value * volume_unit.si_factor() / input_volume.si_factor(),
                    );
                });
            }
            NutrientUnit::Energy(input_energy) => {
                let all_energies = EnergyUnit::get_enumerations();
                all_energies.iter().for_each(|energy_unit| {
                    self.unit_conversions.insert(
                        NutrientUnit::Energy(*energy_unit),
                        value * energy_unit.si_factor() / input_energy.si_factor(),
                    );
                });
            }
            _ => {
                self.unit_conversions.insert(unit, value);
            }
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

    pub fn get_conversion_factor(
        &self,
        from_unit: NutrientUnit,
        to_unit: NutrientUnit,
    ) -> Result<f64, &'static str> {
        let unit_conversions = self.get_unit_conversions();
        let to_factor = match unit_conversions.get(&to_unit) {
            Some(factor) => *factor,
            None => return Err("To conversion factor not found"),
        };
        let from_factor = match unit_conversions.get(&from_unit) {
            Some(factor) => *factor,
            None => return Err("From conversion factor not found"),
        };
        return Ok(from_factor / to_factor);
    }

    pub fn get_accepted_units(&self) -> BTreeSet<NutrientUnit> {
        self.unit_conversions.keys().cloned().collect()
    }

    pub fn get_main_unit(&self) -> NutrientUnit {
        self.main_unit
    }

    pub fn set_main_unit(&mut self, new_main_unit: NutrientUnit) -> Result<(), &'static str> {
        let unit_conversions = self.get_unit_conversions();
        let conversion_factor = match unit_conversions.get(&new_main_unit) {
            Some(factor) => *factor,
            None => return Err("Conversion factor for new main unit was not found"),
        };

        for value in self.unit_conversions.values_mut() {
            *value /= conversion_factor;
        }

        self.main_unit = new_main_unit;
        Ok(())
    }

    pub fn get_ancestors(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        for parent_weak in self.get_parents() {
            if let Some(parent) = parent_weak.upgrade() {
                Self::dfs_up(&parent, &mut result, &mut visited);
            }
        }

        result
    }

    pub fn get_descendants(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        for child in self.get_children() {
            Self::dfs_down(&child, &mut result, &mut visited);
        }

        result
    }

    pub fn get_calories_per_gram(&self) -> EnergyQuantity {
        match self.get_nutrient_type().chemical_type {
            ChemicalType::EnergyYieldingNutrients(energy_yielding_nutrient) => match energy_yielding_nutrient {
                EnergyYieldingNutrients::Carbohydrate(_) => EnergyQuantity::new(4f64, EnergyUnit::Kilocalorie),
                EnergyYieldingNutrients::Protein(_) => EnergyQuantity::new(4f64, EnergyUnit::Kilocalorie),
                EnergyYieldingNutrients::Lipid(_) => EnergyQuantity::new(9f64, EnergyUnit::Kilocalorie),
                EnergyYieldingNutrients::Alcohol => EnergyQuantity::new(7f64, EnergyUnit::Kilocalorie),
            },
            _ => EnergyQuantity::new(0f64, EnergyUnit::Kilocalorie)
        }
    }
}

pub fn link_parent_child(
    parent: &Rc<RefCell<Nutrient>>,
    child: &Rc<RefCell<Nutrient>>,
) -> Result<(), &'static str> {
    // Prevent self-linkage
    if Rc::ptr_eq(parent, child) {
        return Err("Cannot link node to itself");
    }

    // Prevent circular links
    if parent.borrow().get_ancestors().contains(&child) {
        return Err("Child is an ancestor of the parent");
    }

    {
        // Add child to parent's children (strong reference)
        if !parent
            .borrow()
            .children
            .iter()
            .any(|c| Rc::ptr_eq(c, child))
        {
            parent.borrow_mut().children.push(Rc::clone(child));
        }
    }

    {
        // Add parent to child's parents (weak reference)
        let mut child_mut = child.borrow_mut();
        if !child_mut.parents.iter().any(|p| {
            p.upgrade()
                .map(|p_rc| Rc::ptr_eq(&p_rc, parent))
                .unwrap_or(false)
        }) {
            child_mut.parents.push(Rc::downgrade(parent));
        }
    }

    Ok(())
}

pub fn unlink_parent_child(parent: &Rc<RefCell<Nutrient>>, child: &Rc<RefCell<Nutrient>>) {
    parent
        .borrow_mut()
        .children
        .retain(|c| !Rc::ptr_eq(c, child));

    child.borrow_mut().parents.retain(|p| {
        if let Some(p_rc) = p.upgrade() {
            !Rc::ptr_eq(&p_rc, parent)
        } else {
            // Remove dropped weak references
            false
        }
    })
}
