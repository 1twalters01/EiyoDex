use crate::{nutrient::Nutrient, exercise_quantity::ExerciseQuantity};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExerciseQuantityList {
    id: Uuid,
    name: String,
    description: String,
    exercise_quantities: BTreeSet<ExerciseQuantity>,
}

impl ExerciseQuantityList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            exercise_quantities: BTreeSet::new(),
        }
    }

    pub fn from_vec(exercise_amount_vec: Vec<ExerciseQuantity>) -> Self {
        let exercise_quantities: BTreeSet<ExerciseQuantity> =
            exercise_amount_vec.into_iter().collect();

        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            exercise_quantities,
        }
    }

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

    pub fn get_exercise_quantities(&self) -> BTreeSet<ExerciseQuantity> {
        self.exercise_quantities.clone()
    }

    pub fn set_exericise_amounts(&mut self, nutrient_quantities: BTreeSet<ExerciseQuantity>) {
        self.exercise_quantities = exercise_quantities;
    }



  

}
