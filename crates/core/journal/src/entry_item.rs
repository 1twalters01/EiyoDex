use std::{cell::RefCell, rc::Rc};

use exercise::exercise_quantity::ExerciseQuantity;
use foods::food_quantity::FoodQuantity;
use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use units::energy::quantity::EnergyQuantity;

#[derive(Clone)]
pub enum EntryItem {
    FoodQuantity(FoodQuantity),
    ExerciseQuantity(ExerciseQuantity),
}

impl EntryItem {
    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        match self {
            Self::FoodQuantity(food_amount) => food_amount.get_calories(),
            Self::ExerciseQuantity(exercise_amount) => Ok(exercise_amount.get_calories()),
        }
    }

    pub fn get_nutrient_quantity(
        &self,
        nutrient: Rc<RefCell<Nutrient>>,
    ) -> Option<NutrientQuantity> {
        match self {
            Self::FoodQuantity(food_amount) => food_amount.get_nutrient_quantity(nutrient),
            Self::ExerciseQuantity(_) => None,
        }
    }

    pub fn contains_nutrient(&self, nutrient: Rc<RefCell<Nutrient>>) -> bool {
        match self {
            Self::FoodQuantity(food_amount) => food_amount.contains_nutrient(nutrient),
            Self::ExerciseQuantity(_) => false,
        }
    }
}
