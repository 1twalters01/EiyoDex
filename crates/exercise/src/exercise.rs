use units::{
    duration::quantity::DurationQuantity, energy::quantity::EnergyQuantity,
    power::quantity::PowerQuantity,
};
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct ExerciseAmount {
    duration: DurationQuantity,
    exercise: Exercise,
}

impl ExerciseAmount {
    pub fn get_calories(&self) -> EnergyQuantity {
        self.exercise.power.clone() * self.duration
    }
}

#[derive(Clone, PartialEq)]
pub struct Exercise {
    id: Uuid,
    name: String,
    description: String,
    power: PowerQuantity, // EnergyQuantity burned per time unit
}

impl Exercise {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
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

    pub fn get_power(&self) -> PowerQuantity {
        self.power.clone()
    }

    pub fn set_power(&mut self, power: PowerQuantity) {
        self.power = power;
    }
}
