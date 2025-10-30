 use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashSet},
    rc::{Rc, Weak},
};
use uuid::Uuid;

use units::{
    energy::{Energy, EnergyUnit},
    mass::{Mass, MassUnit},
    volume::{Volume, VolumeUnit},
};

use crate::{schema::nutrients::NutrientType, units::NutrientUnit};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitConversion {
    unit: NutrientUnit,
    main_unit: NutrientUnit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nutrient {
    id: Uuid,
    name: String,
    description: String,
    categories: HashSet<NutrientType>,
    main_unit: NutrientUnit,
    unit_conversions: BTreeMap<UnitConversion, f64>, // f64 = unit_to_main_unit_factor
    parent: Vec<Rc<RefCell<Nutrient>>>,
    child: Vec<Rc<RefCell<Nutrient>>>,
}

impl Nutrient {
    pub fn new(id: Option<Uuid>, name: String, main_unit: NutrientUnit) -> Self {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let unit_conversions: BTreeMap<UnitConversion, f64> = match main_unit {
            NutrientUnit::Mass(mass_unit) => {
                let mass_enums = MassUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = mass_enums
                    .iter()
                    .map(|mass_enum| (
                        UnitConversion {
                        unit: NutrientUnit::Mass(*mass_enum),
                        main_unit: main_unit,
                        },
                        mass_enum.si_factor() / mass_unit.si_factor()
                    ))
                    .collect();
                unit_conversions
            },
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = volume_enums
                    .iter()
                    .map(|volume_enum| (
                        UnitConversion {
                            unit: NutrientUnit::Volume(*volume_enum),
                            main_unit: main_unit,
                        },
                        volume_enum.si_factor() / volume_unit.si_factor()
                    ))
                    .collect();
                unit_conversions
            },
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = energy_enums
                    .iter()
                    .map(|energy_enum| (
                        UnitConversion {
                            unit: NutrientUnit::Energy(*energy_enum),
                            main_unit: main_unit,
                        },
                        energy_enum.si_factor() / energy_unit.si_factor(),
                    ))
                    .collect();
                unit_conversions
            },
            _ => {
                let unit_conversions = BTreeMap::from([(
                    UnitConversion {
                        unit: main_unit,
                        main_unit: main_unit,
                    },
                    1f64
                )
                ]);
                unit_conversions
            }
        };

        Nutrient {
            id,
            name,
            description: String::new(),
            categories: HashSet::new(),
            main_unit,
            unit_conversions: unit_conversions,
            parent: Vec::new(),
            child: Vec::new(),
        }
    }

    pub fn new_rc_refcell(
        id: Option<Uuid>,
        name: String,
        main_unit: NutrientUnit,
    ) -> Rc<RefCell<Self>> {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let unit_conversions: BTreeMap<UnitConversion, f64> = match main_unit {
            NutrientUnit::Mass(mass_unit) => {
                let mass_enums = MassUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = mass_enums
                    .iter()
                    .map(|mass_enum| (
                        UnitConversion {
                        unit: NutrientUnit::Mass(*mass_enum),
                        main_unit: main_unit,
                        },
                        mass_enum.si_factor() / mass_unit.si_factor()
                    ))
                    .collect();
                unit_conversions
            },
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = volume_enums
                    .iter()
                    .map(|volume_enum| (
                        UnitConversion {
                            unit: NutrientUnit::Volume(*volume_enum),
                            main_unit: main_unit,
                        },
                        volume_enum.si_factor() / volume_unit.si_factor()
                    ))
                    .collect();
                unit_conversions
            },
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = energy_enums
                    .iter()
                    .map(|energy_enum| (
                        UnitConversion {
                            unit: NutrientUnit::Energy(*energy_enum),
                            main_unit: main_unit,
                        },
                        energy_enum.si_factor() / energy_unit.si_factor(),
                    ))
                    .collect();
                unit_conversions
            },
            _ => {
                let unit_conversions = BTreeMap::from([(
                    UnitConversion {
                        unit: main_unit,
                        main_unit: main_unit,
                    },
                    1f64
                )
                ]);
                unit_conversions
            }
        };
        
        Rc::new(RefCell::new(Nutrient {
            id,
            name,
            description: String::new(),
            categories: HashSet::new(),
            main_unit,
            unit_conversions: unit_conversions,
            parent: Vec::new(),
            child: Vec::new(),
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

    pub fn extend_category(&mut self, categories: Vec<NutrientType>) {
        self.categories.extend(categories);
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
        self.unit_conversions.iter().map(|conversion| conversion.0.unit).collect()
    }

    pub fn update_accepted_units(&mut self, new_main_unit: NutrientUnit) -> Result<(), &'static str> {
        let new_factor = if self.main_unit == new_main_unit {
            1f64
        } else {
            let conversion = UnitConversion {
                unit: new_main_unit,
                main_unit: self.main_unit,
            };
            match self.unit_conversions.get(&conversion) {
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
                NutrientUnit::Volume(volume_unit) => volume_units.push(NutrientUnit::Volume(*volume_unit)),
                NutrientUnit::Energy(energy_unit) => energy_units.push(NutrientUnit::Energy(*energy_unit)),
                other => other_units.push(other)
            }
        }

        if !mass_units.is_empty() {
            let all_mass_units = MassUnit::get_enumerations().iter()
                .map(|mass_enum| NutrientUnit::Mass(*mass_enum)
            ).collect::<BTreeSet<NutrientUnit>>();

            let (included_mass_units, unincluded_mass_units): (Vec<NutrientUnit>, Vec<NutrientUnit>) = 
                all_mass_units
                    .into_iter()
                    .partition(|unit| accepted_units.contains(unit));

            let initial_mass_unit = match included_mass_units.first() {
                 Some(unit) => unit,
                 None => return Err("No mass unit despite containing mass"),
            };

            for unit in unincluded_mass_units {
                let conversion = UnitConversion {
                    unit: unit,
                    main_unit: self.main_unit,
                };

                let initial_conversion_factor: f64 = match self.unit_conversions.get(
                        &UnitConversion { unit: *initial_mass_unit, main_unit: self.main_unit }
                ) {
                     Some(factor) => *factor,
                     None => panic!("factor not found"),
                };

                let factor = match unit.si_factor().zip(initial_mass_unit.si_factor()).map(|(unit, initial)| (unit / initial) * initial_conversion_factor * new_factor) {
                    Some(factor) => factor,
                    None => return Err("factor not found"),
                };
                self.unit_conversions.insert(conversion, factor);
            }

            for unit in included_mass_units {
                if self.main_unit != new_main_unit {
                    let conversion = UnitConversion {
                        unit, main_unit: self.main_unit
                    };
                    let conversion_factor: f64 = match self.unit_conversions.get(&conversion) {
                        Some(factor) => *factor,
                        None => return Err("factor not found"),
                    };
                    let new_conversion_factor = conversion_factor * new_factor;
                    self.unit_conversions.insert(conversion, new_conversion_factor);
                 }
            }
        }

        if !volume_units.is_empty() {
            let all_volume_units = VolumeUnit::get_enumerations().iter()
                .map(|volume_enum| NutrientUnit::Volume(*volume_enum)
            ).collect::<BTreeSet<NutrientUnit>>();

            let (included_volume_units, unincluded_volume_units): (Vec<NutrientUnit>, Vec<NutrientUnit>) = 
                all_volume_units
                    .into_iter()
                    .partition(|unit| accepted_units.contains(unit));

            let initial_volume_unit = match included_volume_units.first() {
                 Some(unit) => unit,
                 None => return Err("No volume unit despite containing volume"),
            };

            for unit in unincluded_volume_units {
                let conversion = UnitConversion {
                    unit: unit,
                    main_unit: self.main_unit,
                };

                let initial_conversion_factor: f64 = match self.unit_conversions.get(
                        &UnitConversion { unit: *initial_volume_unit, main_unit: self.main_unit }
                ) {
                     Some(factor) => *factor,
                     None => return Err("factor not found"),
                };

                let factor = match unit.si_factor().zip(initial_volume_unit.si_factor()).map(|(unit, initial)| (unit / initial) * initial_conversion_factor * new_factor) {
                    Some(factor) => factor,
                    None => return Err("factor not found"),
                };
                self.unit_conversions.insert(conversion, factor);
            }

            for unit in included_volume_units {
                 if self.main_unit != new_main_unit {
                    let conversion = UnitConversion {
                        unit, main_unit: self.main_unit
                    };
                    let conversion_factor: f64 = match self.unit_conversions.get(&conversion) {
                        Some(factor) => *factor,
                        None => return Err("factor not found"),
                    };
                    let new_conversion_factor = conversion_factor * new_factor;
                    self.unit_conversions.insert(conversion, new_conversion_factor);
                }
            }
        }

        if !energy_units.is_empty() {
            let all_energy_units = EnergyUnit::get_enumerations().iter()
                .map(|energy_enum| NutrientUnit::Energy(*energy_enum)
            ).collect::<BTreeSet<NutrientUnit>>();

            let (included_energy_units, unincluded_mass_units): (Vec<NutrientUnit>, Vec<NutrientUnit>) = 
                all_energy_units
                    .into_iter()
                    .partition(|unit| accepted_units.contains(unit));

            let initial_energy_unit = match included_energy_units.first() {
                 Some(unit) => unit,
                 None => return Err("No mass unit despite containing mass"),
            };

            for unit in unincluded_mass_units {
                let conversion = UnitConversion {
                    unit: unit,
                    main_unit: self.main_unit,
                };

                let initial_conversion_factor: f64 = match self.unit_conversions.get(
                        &UnitConversion { unit: *initial_energy_unit, main_unit: self.main_unit }
                ) {
                     Some(factor) => *factor,
                     None => return Err("factor not found"),
                };

                let factor = match unit.si_factor().zip(initial_energy_unit.si_factor()).map(|(unit, initial)| (unit / initial) * initial_conversion_factor * new_factor) {
                    Some(factor) => factor,
                    None => return Err("factor not found"),
                };
                self.unit_conversions.insert(conversion, factor);
            }

            for unit in included_energy_units {
                if self.main_unit != new_main_unit {
                    let conversion = UnitConversion {
                       unit, main_unit: self.main_unit
                    };
                    let conversion_factor: f64 = match self.unit_conversions.get(&conversion) {
                        Some(factor) => *factor,
                        None => return Err("factor not found"),
                    };
                    let new_conversion_factor = conversion_factor * new_factor;
                    self.unit_conversions.insert(conversion, new_conversion_factor);
                }
            }
        }

        for unit in other_units {
            if self.main_unit != new_main_unit {
                let conversion = UnitConversion {
                    unit: *unit, main_unit: self.main_unit
                };
                let conversion_factor: f64 = match self.unit_conversions.get(&conversion) {
                    Some(factor) => *factor,
                    None => return Err("factor not found"),
                };
                let new_conversion_factor = conversion_factor * new_factor;
                self.unit_conversions.insert(conversion, new_conversion_factor);
            }
        }

        return Ok(())
    }

    pub fn convert(&self, value: f64, from: NutrientUnit, to: NutrientUnit) -> Result<f64, String> {
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
            let to_conversion = UnitConversion { unit: to, main_unit: self.main_unit };
            let from_conversion = UnitConversion { unit: from, main_unit: self.main_unit };

            let to_factor = match self.unit_conversions.get(&to_conversion) {
                Some(factor) => *factor,
                None => return Err(String::from("Conversion factor not found")),
            };
            let from_factor = match self.unit_conversions.get(&from_conversion) {
                Some(factor) => *factor,
                None => return Err(String::from("Conversion factor not found")),
            };
            let factor = to_factor / from_factor;
            let new_value = value * factor;
            return Ok(new_value)
        } else {
            Err(String::from(
                "Conversion must use values accepted by nutrient",
            ))
        }
    }

    pub fn remove_conversion(&mut self, unit: NutrientUnit) -> Result<(), &'static str> {
        if unit == self.main_unit {
            return Err("Cannot remove a conversion if it is the main unit")
        }

        self.unit_conversions.remove(&UnitConversion { unit, main_unit: self.main_unit });

        match unit {
            NutrientUnit::Mass(_) => {
                let all_masses = MassUnit::get_enumerations();
                all_masses.iter().for_each(|mass_unit| {
                    self.unit_conversions.remove(&UnitConversion { 
                        unit: NutrientUnit::Mass(*mass_unit),
                        main_unit: self.main_unit,
                    });
                });
            },
            NutrientUnit::Volume(_) => {
                let all_volumes = VolumeUnit::get_enumerations();
                all_volumes.iter().for_each(|volume_unit| {
                    self.unit_conversions.remove(&UnitConversion {
                        unit: NutrientUnit::Volume(*volume_unit),
                        main_unit: self.main_unit,
                    });
                });
            },
            NutrientUnit::Energy(_) => {
                let all_energies = EnergyUnit::get_enumerations();
                all_energies.iter().for_each(|energy_unit| {
                    self.unit_conversions.remove(&UnitConversion {
                        unit: NutrientUnit::Energy(*energy_unit),
                        main_unit: self.main_unit
                    });
                });
            },
            other_unit => {
                self.unit_conversions.remove(&UnitConversion {
                    unit: other_unit,
                    main_unit: self.main_unit,
                });
            },
        }

        return Ok(())
    }

    pub fn add_conversion(
        &mut self,
        unit: NutrientUnit,
        factor: f64,
    ) -> Result<(), &'static str> {
        match unit {
            NutrientUnit::Mass(mass_unit) => {
                let si_factor = match unit.si_factor() {
                    Some(si_factor) => si_factor,
                    None => return Err("invalid mass"),
                };
                MassUnit::get_enumerations()
                    .iter()
                    .for_each(|mass_unit| {
                        let conversion = UnitConversion {
                            unit: NutrientUnit::Mass(*mass_unit),
                            main_unit: self.main_unit,
                        };

                        let mass_factor = si_factor / mass_unit.si_factor();

                        self.unit_conversions.insert(
                            conversion,
                            mass_factor,
                        );
                    });
            },
            NutrientUnit::Volume(volume_unit) => {
                let si_factor = match unit.si_factor() {
                    Some(si_factor) => si_factor,
                    None => return Err("invalid mass"),
                };
                VolumeUnit::get_enumerations()
                    .iter()
                    .for_each(|volume_unit| {
                        let conversion = UnitConversion {
                            unit: NutrientUnit::Volume(*volume_unit),
                            main_unit: self.main_unit,
                        };

                        let mass_factor = si_factor / volume_unit.si_factor();

                        self.unit_conversions.insert(
                            conversion,
                            mass_factor,
                        );
                    });
            },
            NutrientUnit::Energy(energy_unit) => {
                let si_factor = match unit.si_factor() {
                    Some(si_factor) => si_factor,
                    None => return Err("invalid mass"),
                };
                EnergyUnit::get_enumerations()
                    .iter()
                    .for_each(|energy_unit| {
                        let conversion = UnitConversion {
                            unit: NutrientUnit::Energy(*energy_unit),
                            main_unit: self.main_unit,
                        };

                        let mass_factor = si_factor / energy_unit.si_factor();

                        self.unit_conversions.insert(
                            conversion,
                            mass_factor,
                        );
                    });
            },
            _ => {
                self.unit_conversions.insert(
                    UnitConversion {
                        unit,
                        main_unit: self.main_unit,
                    },
                    factor,
                );
            },
        }

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
