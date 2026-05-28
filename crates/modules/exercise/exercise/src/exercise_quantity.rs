use std::{cell::RefCell, rc::Rc};

use units::{
    duration::quantity::DurationQuantity, energy::quantity::EnergyQuantity,
};
use uuid::Uuid;

use crate::exercise::Exercise;

#[derive(Clone, PartialEq)]
pub struct ExerciseQuantity {
    duration_quantity: DurationQuantity,
    exercise: Rc<RefCell<Exercise>>,
}

impl ExerciseQuantity {
    pub fn new(duration: DurationQuantity, exercise: Rc<RefCell<Exercise>>) -> Self {
        Self {
            duration_quantity: duration,
            exercise: exercise,
        }
    }

    pub fn get_duration_quantity(&self) -> DurationQuantity {
        self.duration_quantity.clone()
    }

    pub fn set_duration_quantity(&mut self, duration_quantity: DurationQuantity) {
        self.duration_quantity = duration_quantity;
    }

    pub fn get_calories(&self) -> EnergyQuantity {
        self.exercise.borrow().get_power_quantity().clone() * self.get_duration_quantity()
    }
}

