use units::energy::Energy;
use uuid::Uuid;

use crate::food::Food;

#[derive(Clone, PartialEq)]
pub struct FoodAmount {
    value: f64,
    food: Food,
}

impl FoodAmount {
    pub fn new() {}

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_food(&self) -> &Food {
        &self.food
    }

    pub fn get_calories(&self, food_data_uuid: Uuid) -> Energy {
        self.food.get_calories(food_data_uuid) * self.value
    }
}

