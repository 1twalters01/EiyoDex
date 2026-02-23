use exercise::exercise::ExerciseAmount;
use foods::food_amount::FoodAmount;
use nutrients::{nutrient::Nutrient, nutrient_amount::NutrientAmount};
use units::energy::quantity::EnergyQuantity;

#[derive(Clone, PartialEq)]
pub enum EntryItem {
    FoodAmount(FoodAmount),
    ExerciseAmount(ExerciseAmount),
}

impl EntryItem {
    pub fn get_calories(&self) -> EnergyQuantity {
        match self {
            Self::FoodAmount(food_amount) => food_amount.get_calories(),
            Self::ExerciseAmount(exercise_amount) => exercise_amount.get_calories(),
        }
    }

    pub fn get_nutrient_amount(&self, nutrient: Nutrient) -> Option<NutrientAmount> {
        match self {
            Self::FoodAmount(food_amount) => food_amount.get_nutrient_amount(nutrient),
            Self::ExerciseAmount(_) => None,
        }
    }

    pub fn contains_nutrient(&self, nutrient: Nutrient) -> bool {
        match self {
            Self::FoodAmount(food_amount) => food_amount.contains_nutrient(nutrient),
            Self::ExerciseAmount(_) => false,
        }
    }
}
