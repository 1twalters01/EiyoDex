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
    parent: Vec<Weak<Refcell<Nutrient>>>,
    child: Vec<Rc<Refcell<Nutrient>>>,
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
                let unit_conversions: BTreeSet<UnitConversion> = volume_enums
                    .iter()
                    .map(|mass_enum| (
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
                let unit_conversions: BTreeSet<UnitConversion> = energy_enums
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
            parent_uuids: Vec::new(),
            child_uuids: Vec::new(),
        }
    }

    pub fn new_rc_refcell(
        id: Option<Uuid>,
        name: String,
        main_unit: NutrientUnit,
    ) -> Rc<RefCell<Self>> {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let (unit_conversions, accepted_units): (BTreeSet<UnitConversion>, BTreeSet<NutrientUnit>) = match main_unit {
            NutrientUnit::Mass(mass_unit) => {
                let mass_enums = MassUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = mass_enums
                    .iter()
                    .map(|mass_enum| (
                        UnitConversion {
                            unit: NutrientUnit::Mass(*mass_enum),
                            main_unit: main_unit,
                        },
                        mass_enum.si_factor() / mass_unit.si_factor(),
                    ))
                    .collect();
                unit_conversions
            },
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeMap<UnitConversion, f64> = volume_enums
                    .iter()
                    .map(|mass_enum| (
                        UnitConversion {
                            unit: NutrientUnit::Volume(*volume_enum),
                            main_unit: main_unit,
                        },
                        volume_enum.si_factor() / volume_unit.si_factor(),
                    ))
                    .collect();
                unit_conversions
            },
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeSet<UnitConversion> = energy_enums
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
                )]);
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

    pub fn get_main_unit(&self) -> NutrientUnit {
        self.main_unit
    }

    pub fn set_main_unit(&mut self, main_unit: NutrientUnit) -> Result<(), &'static str> {
        if self.main_unit == main_unit {
            return Ok(());
        }
     
        let accepted_units = self.get_accepted_units();
        if accepted_units.contains(&main_unit) {
            self.main_unit = main_unit;
            return Ok(());
        }

        Err("Cannot convert to selected unit")
    }

    pub fn get_accepted_units(&mut self) -> BTreeSet<NutrientUnit> {
        let accepted_units = self.unit_conversions.iter().map(|conversion| conversion.unit).collect();

        let contains_mass = accepted_units.iter().any(|unit| matches!(unit, NutrientUnit::Mass(_)));
        if contains_mass {
            let all_mass_units = MassUnit::get_enumerations().iter()
                .map(|mass_enum| UnitConversion {
                    NutrientUnit::Mass(*mass_enum)
                }).collect::<Vec<NutrientUnit>>();

            let all_included = all_mass_units.iter().all(|unit| accepted_units.contains(unit));
            if !all_included {
                let first_mass_option = accepted_units
                    .iter()
                    .find_map(|nt| {
                        if let NutrientUnit::Mass(unit) = nt {
                            Some(unit)
                        } else {
                            None
                        }
                    });
                if Some(first_mass) == first_mass_option {
                    let first_mass_conversion_factor = self.unit_conversions.get(
                        &UnitConversion {
                            unit: first_mass,
                            main_unit: self.main_unit
                        });

                    for mass_unit in all_mass_units {
                        let conversion = UnitConversion {
                            unit: NutrientUnit::Mass(unit),
                            main_unit: self.main_unit,
                        }
                        let factor = mass_unit.si_factor() / first_mass.si_factor * first_mass_conversion_factor;
                        self.unit_conversions.push(conversion, factor);
                        accepted_units.push(mass_unit);
                    }
                }
            }
        }

        let contains_volume = accepted_units.iter().any(|unit| matches!(unit, NutrientUnit::Volume(_)));
        if contains_volume {
            let all_volume_units = VolumeUnit::get_enumerations().iter()
                .map(|volume_enum| UnitConversion {
                    NutrientUnit::Volume(*volume_enum)
                }).collect::<Vec<NutrientUnit>>();

            let all_included = all_volume_units.iter().all(|unit| accepted_units.contains(unit));
            if !all_included {
                let first_volume_option = accepted_units
                    .iter()
                    .find_map(|nt| {
                        if let NutrientUnit::Volume(unit) = nt {
                            Some(unit)
                        } else {
                            None
                        }
                    });
                if Some(first_volume) == first_volume_option {
                    let first_mass_conversion_factor = self.unit_conversions.get(
                        &UnitConversion {
                            unit: first_volume,
                            main_unit: self.main_unit
                        });

                    for volume_unit in all_volume_units {
                        let conversion = UnitConversion {
                            unit: NutrientUnit::Volume(unit),
                            main_unit: self.main_unit,
                        }
                        let factor = volume_unit.si_factor() / first_volume.si_factor * first_volume_conversion_factor;
                        self.unit_conversions.push(conversion, factor);
                        accepted_units.push(volume_unit);
                    }
                }
            }
        }

        let contains_energy = accepted_units.iter().any(|unit| matches!(unit, NutrientUnit::Energy(_)));
        if contains_energy {
            let all_energy_units = massunit::get_enumerations().iter()
                .map(|energy_enum| UnitConversion {
                    NutrientUnit::Energy(*energy_enum)
                }).collect::<Vec<NutrientUnit>>();

            let all_included = all_energy_units.iter().all(|unit| accepted_units.contains(unit));
            if !all_included {
                let first_energy_option = accepted_units
                    .iter()
                    .find_map(|nt| {
                        if let NutrientUnit::Energy(unit) = nt {
                            Some(unit)
                        } else {
                            None
                        }
                    });
                if Some(first_energy) == first_energy_option {
                    let first_energy_conversion_factor = self.unit_conversions.get(
                        &UnitConversion {
                            unit: first_mass,
                            main_unit: self.main_unit
                        });

                    for energy_unit in all_energy_units {
                        let conversion = UnitConversion {
                            unit: NutrientUnit::Mass(unit),
                            main_unit: self.main_unit,
                        }
                        let factor = energy_unit.si_factor() / first_energy.si_factor * first_energy_conversion_factor;
                        self.unit_conversions.push(conversion, factor);
                        accepted_units.push(energy_unit);
                    }
                }
            }
        }

        return accepted_units
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

    pub fn remove_conversion(&mut self, from: NutrientUnit, to: NutrientUnit) {
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

    pub fn add_conversion(
        &mut self,
        from: NutrientUnit,
        to: NutrientUnit,
        factor: f64,
    ) -> Result<(), String> {
        if factor == 0 as f64 {
            return Err(String::from("Conversion factor may not equal 0"));
        }

        let (from_unit, from_factor): (NutrientUnit, f64) = match from {
            NutrientUnit::Mass(mass_unit) => {
                let _ = MassUnit::get_enumerations()
                    .iter()
                    .map(|mass_unit| self.accepted_units.insert(NutrientUnit::Mass(*mass_unit)));

                let mass = Mass::new(1 as f64, mass_unit);
                (NutrientUnit::Mass(MassUnit::Gram), mass.as_g())
            }
            NutrientUnit::Volume(volume_unit) => {
                let _ = VolumeUnit::get_enumerations().iter().map(|volume_unit| {
                    self.accepted_units
                        .insert(NutrientUnit::Volume(*volume_unit))
                });

                let volume = Volume::new(1 as f64, volume_unit);
                (NutrientUnit::Volume(VolumeUnit::Milliliter), volume.as_ml())
            }
            NutrientUnit::Energy(energy_unit) => {
                let _ = EnergyUnit::get_enumerations().iter().map(|energy_unit| {
                    self.accepted_units
                        .insert(NutrientUnit::Energy(*energy_unit))
                });

                let energy = Energy::new(1 as f64, energy_unit);
                (
                    NutrientUnit::Energy(EnergyUnit::Kilocalorie),
                    energy.as_kcal(),
                )
            }
            _ => {
                self.accepted_units.insert(from);
                (to, 1 as f64)
            }
        };

        let (to_unit, to_factor): (NutrientUnit, f64) = match to {
            NutrientUnit::Mass(mass_unit) => {
                let _ = MassUnit::get_enumerations()
                    .iter()
                    .map(|mass_unit| self.accepted_units.insert(NutrientUnit::Mass(*mass_unit)));

                let mass = Mass::new(1 as f64, mass_unit);
                (NutrientUnit::Mass(MassUnit::Gram), mass.as_g())
            }
            NutrientUnit::Volume(volume_unit) => {
                let _ = VolumeUnit::get_enumerations().iter().map(|volume_unit| {
                    self.accepted_units
                        .insert(NutrientUnit::Volume(*volume_unit))
                });

                let volume = Volume::new(1 as f64, volume_unit);
                (NutrientUnit::Volume(VolumeUnit::Milliliter), volume.as_ml())
            }
            NutrientUnit::Energy(energy_unit) => {
                let _ = EnergyUnit::get_enumerations().iter().map(|energy_unit| {
                    self.accepted_units
                        .insert(NutrientUnit::Energy(*energy_unit))
                });

                let energy = Energy::new(1 as f64, energy_unit);
                (
                    NutrientUnit::Energy(EnergyUnit::Kilocalorie),
                    energy.as_kcal(),
                )
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
