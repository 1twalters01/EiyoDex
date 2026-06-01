use units::power::{quantity::PowerQuantity, unit::PowerUnit};
use identity::entity::Entity;

#[derive(Debug, Clone)]
pub struct Exercise {
    name: String,
    description: String,
    power_quantity_entity: Entity<PowerQuantity>, // Energy burned per time unit
}

impl PartialEq for Exercise {
    fn eq(&self, other: &Self) -> bool {
            self.name == other.name
            && self.description == other.description
            && self.power_quantity_entity == other.power_quantity_entity
    }
}

impl Exercise {
    pub fn from_values(name: String, description: String, power_quantity_entity: Entity<PowerQuantity>) -> Self {
        Self { name, description, power_quantity_entity }
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

    pub fn get_power_quantity_entity(&self) -> Entity<PowerQuantity> {
        self.power_quantity_entity.clone()
    }

    pub fn get_power_quantity(&self) -> PowerQuantity {
        self.power_quantity_entity.get_inner().clone()
    }

    pub fn get_power_unit(&self) -> PowerUnit {
        self.power_quantity_entity.get_inner().get_unit()
    }

    pub fn set_power_quantity_entity(&mut self, power_quantity_entity: Entity<PowerQuantity>) {
        self.power_quantity_entity = power_quantity_entity;
    }
}
