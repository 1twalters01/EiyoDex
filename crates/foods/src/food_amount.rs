use std::{cell::RefCell, rc::Rc};

use units::energy::Energy;
use uuid::Uuid;

use crate::{data_sources::DataSource, food::Food};

#[derive(Clone, PartialEq)]
pub struct FoodAmount {
    value: f64,
    food: Food,
    data_source: Rc<RefCell<DataSource>>,
    food_data_uuid: Uuid,
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

    pub fn get_calories(&self) -> Energy {
        self.food.get_calories(self.food_data_uuid, self.data_source.borrow().get_id()) * self.value
    }
}

