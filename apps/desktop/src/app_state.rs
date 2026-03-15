use std::{cell::RefCell, rc::Rc};

use nutrients::nutrient::Nutrient;
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};

pub struct AppState {
    masses: Vec<MassUnit>,
    volumes: Vec<VolumeUnit>,
    energies: Vec<EnergyUnit>,
    nutrients: Vec<Rc<RefCell<Nutrient>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            masses: MassUnit::get_enumerations().to_vec(),
            volumes: VolumeUnit::get_enumerations().to_vec(),
            energies: EnergyUnit::get_enumerations().to_vec(),
            nutrients: Vec::new(),
        }
    }
}
