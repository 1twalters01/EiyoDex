use uuid::Uuid;
use crate::sources::DataSource;
use nutrients::nutrient::Nutrient;
use std::collections::BTreeSet;
use units::specific_currency::SpecificCurrency;

pub struct FoodAmount {
    value: f64,
    food: Food,
}

impl FoodAmount {
    pub fn new() {
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_food(&self) -> &Food {
        &self.food
    }
}

// TODO - per whatever unig (100g?)
pub struct Food {
    id: Uuid,
    name: String,
    cost: Option<SpecificCurrency>,
}

impl Food {
    pub fn new(id: Option<Uuid>, name: String, cost: Option<SpecificCurrency>) -> Self{
        let id = if let Some(internal_id) = id {
            internal_id
        } else {
            Uuid::new_v4()
        };

        Self { id, name, cost }
    }
}

pub struct FoodInstance {
    id: Uuid,
    name: String,
    data_source: DataSource,
    nutrients: BTreeSet<Nutrient>,
}
