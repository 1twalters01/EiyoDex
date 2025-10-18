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
    unit_to_main_unit_factor: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Nutrient {
    id: Uuid,
    name: String,
    description: String,
    categories: HashSet<NutrientType>,
    main_unit: NutrientUnit,
    accepted_units: BTreeSet<NutrientUnit>,
    unit_conversions: BTreeSet<UnitConversion>,
    parent_uuids: Vec<Weak<Refcell>>,
    child_uuids: Vec<Rc<Refcell>>,
}

impl Nutrient {
    pub fn new(id: Option<Uuid>, name: String, main_unit: NutrientUnit) -> Self {
        let id = id.unwrap_or_else(Uuid::new_v4);
        let (unit_conversions, accepted_units): (BTreeSet<UnitConversion>, BTreeSet<NutrientUnit>) = match main_unit {
            NutrientUnit::Mass(mass_unit) => {
                let mass_enums = MassUnit::get_enumerations();
                let unit_conversions: BTreeSet<UnitConversion> = mass_enums
                    .iter()
                    .map(|mass_enum| UnitConversion {
                        unit: NutrientUnit::Mass(*mass_enum)),
                        main_unit: main_unit,
                        unit_to_main_unit_factor: mass_enum.si_factor() / mass_unit.si_factor(),
                    .collect::BTreeSet<UnitConversion>();
                let accepted_units = unit_conversions
                    .iter()
                    .map(|unit_conversion| unit_conversion.unit)
                    .collect::BTreeSet<NutrientUnit>();
                (unit_conversions, accepted_units)
            },
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeSet<UnitConversion> = volume_enums
                    .iter()
                    .map(|mass_enum| UnitConversion {
                        unit: NutrientUnit::Volume(*volume_enum)),
                        main_unit: main_unit,
                        unit_to_main_unit_factor: volume_enum.si_factor() / volume_unit.si_factor(),
                    .collect::BTreeSet<UnitConversion>();
                let accepted_units = unit_conversions
                    .iter()
                    .map(|unit_conversion| unit_conversion.unit)
                    .collect::BTreeSet<NutrientUnit>();
                (unit_conversions, accepted_units)
            },
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeSet<UnitConversion> = energy_enums
                    .iter()
                    .map(|energy_enum| UnitConversion {
                        unit: NutrientUnit::Energy(*energy_enum)),
                        main_unit: main_unit,
                        unit_to_main_unit_factor: energy_enum.si_factor() / energy_unit.si_factor(),
                    .collect::BTreeSet<UnitConversion>();
                let accepted_units = unit_conversions
                    .iter()
                    .map(|unit_conversion| unit_conversion.unit)
                    .collect::BTreeSet<NutrientUnit>();
                (unit_conversions, accepted_units)
            },
            _ => {
                let unit_conversions = BTreeSet::from(
                    UnitConversion {
                        unit: main_unit,
                        main_unit: main_unit,
                        unit_to_main_unit_factor: 1f64,
                    }
                );
                let accepted_units = BTreeSet::from(main_unit);
                (unit_conversions, accepted_units)
        };

        Nutrient {
            id,
            name,
            description: String::new(),
            categories: HashSet::new(),
            main_unit,
            accepted_units: BTreeSet::from([main_unit]),
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
                let unit_conversions: BTreeSet<UnitConversion> = mass_enums
                    .iter()
                    .map(|mass_enum| UnitConversion {
                        unit: NutrientUnit::Mass(*mass_enum)),
                        main_unit: main_unit,
                        unit_to_main_unit_factor: mass_enum.si_factor() / mass_unit.si_factor(),
                    .collect::BTreeSet<UnitConversion>();
                let accepted_units = unit_conversions
                    .iter()
                    .map(|unit_conversion| unit_conversion.unit)
                    .collect::BTreeSet<NutrientUnit>();
                (unit_conversions, accepted_units)
            },
            NutrientUnit::Volume(volume_unit) => {
                let volume_enums = VolumeUnit::get_enumerations();
                let unit_conversions: BTreeSet<UnitConversion> = volume_enums
                    .iter()
                    .map(|mass_enum| UnitConversion {
                        unit: NutrientUnit::Volume(*volume_enum)),
                        main_unit: main_unit,
                        unit_to_main_unit_factor: volume_enum.si_factor() / volume_unit.si_factor(),
                    .collect::BTreeSet<UnitConversion>();
                let accepted_units = unit_conversions
                    .iter()
                    .map(|unit_conversion| unit_conversion.unit)
                    .collect::BTreeSet<NutrientUnit>();
                (unit_conversions, accepted_units)
            },
            NutrientUnit::Energy(energy_unit) => {
                let energy_enums = EnergyUnit::get_enumerations();
                let unit_conversions: BTreeSet<UnitConversion> = energy_enums
                    .iter()
                    .map(|energy_enum| UnitConversion {
                        unit: NutrientUnit::Energy(*energy_enum)),
                        main_unit: main_unit,
                        unit_to_main_unit_factor: energy_enum.si_factor() / energy_unit.si_factor(),
                    .collect::BTreeSet<UnitConversion>();
                let accepted_units = unit_conversions
                    .iter()
                    .map(|unit_conversion| unit_conversion.unit)
                    .collect::BTreeSet<NutrientUnit>();
                (unit_conversions, accepted_units)
            },
            _ => {
                let unit_conversions = BTreeSet::from(
                    UnitConversion {
                        unit: main_unit,
                        main_unit: main_unit,
                        unit_to_main_unit_factor: 1f64,
                    }
                );
                let accepted_units = BTreeSet::from(main_unit);
                (unit_conversions, accepted_units)
        };
        
        Rc::new(RefCell::new(Nutrient {
            id,
            name,
            description: String::new(),
            categories: HashSet::new(),
            main_unit,
            accepted_units: accepted_units,
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
        if self.accepted_units.contains(&main_unit) {
            self.main_unit = main_unit;
            return Ok(());
        }

        match self.main_unit {
            NutrientUnit::Mass(_) => {
                if let NutrientUnit::Mass(_) = main_unit {
                    let mass_units = MassUnit::get_enumerations();
                    self.accepted_units.extend(
                        mass_units
                            .iter()
                            .map(|mass_enum| NutrientUnit::Mass(*mass_enum)),
                    );
                    self.main_unit = main_unit;
                    return Ok(());
                }
            }
            NutrientUnit::Volume(_) => {
                if let NutrientUnit::Volume(_) = main_unit {
                    self.accepted_units.insert(main_unit);
                    self.main_unit = main_unit;
                    return Ok(());
                }
            }
            NutrientUnit::Energy(_) => {
                if let NutrientUnit::Energy(_) = main_unit {
                    self.accepted_units.insert(main_unit);
                    self.main_unit = main_unit;
                    return Ok(());
                }
            }
            _ => return Err("New main unit not in accepted units"),
        }

        Err("New main unit not in accepted units")
    }

    pub fn get_accepted_units(&self) -> BTreeSet<NutrientUnit> {
        let mut accepted_units = self.accepted_units.clone();
        let mut mass_added = false;
        let mut volume_added = false;
        let mut energy_added = false;
        for unit in accepted_units.clone() {
            match unit {
                NutrientUnit::Mass(_) => {
                    if !mass_added {
                        accepted_units.extend(
                            MassUnit::get_enumerations()
                                .iter()
                                .map(|unit| NutrientUnit::Mass(*unit)),
                        );
                        mass_added = true;
                    }
                }
                NutrientUnit::Volume(_) => {
                    if !volume_added {
                        accepted_units.extend(
                            VolumeUnit::get_enumerations()
                                .iter()
                                .map(|unit| NutrientUnit::Volume(*unit)),
                        );
                        volume_added = true;
                    }
                }
                NutrientUnit::Energy(_) => {
                    if !energy_added {
                        accepted_units.extend(
                            EnergyUnit::get_enumerations()
                                .iter()
                                .map(|unit| NutrientUnit::Energy(*unit)),
                        );
                        energy_added = true;
                    }
                }
                _ => {}
            }
        }
        accepted_units
    }

    pub fn push_to_accepted_units(&mut self, accepted_unit: NutrientUnit) {
        self.accepted_units.insert(accepted_unit);
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
