use std::{cell::RefCell, rc::Rc};

use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

use crate::{data_sources::DataSource, food::Food};

#[derive(Clone, PartialEq)]
pub struct FoodQuantity {
    value: f64,
    food: Food,
    data_source: Rc<RefCell<DataSource>>,
    food_instance_uuid: Uuid, // The 
}

impl FoodQuantity {
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

    pub fn get_calories(&self) -> EnergyQuantity {
        self.food
            .get_calories(self.food_instance_uuid, self.data_source.borrow().get_id())
            * self.value
    }

    pub fn get_nutrient_amount(&self, nutrient: Nutrient) -> Option<NutrientQuantity> {
        self.food
            .get_nutrient_amount(
                nutrient,
                self.food_instance_uuid,
                self.data_source.borrow().get_id(),
            )
            .and_then(|amount| Some(amount * self.value))
    }

    pub fn contains_nutrient(&self, nutrient: Nutrient) -> bool {
        self.food
            .get_nutrient_amount(
                nutrient,
                self.food_instance_uuid,
                self.data_source.borrow().get_id(),
            )
            .is_some()
    }
}
