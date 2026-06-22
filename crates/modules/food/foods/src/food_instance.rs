use std::{cell::RefCell, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{data_sources::DataSourceInstance, food_variant::FoodVariant};

pub struct FoodInstance {
    food_variant: Weak<RefCell<FoodVariant>>,
    data_source_instance: Rc<RefCell<DataSourceInstance>>,
}

impl FoodInstance {
    pub fn get_food_variant(&self) -> Weak<RefCell<FoodVariant>> {
        self.food_variant.clone()
    }

    pub fn set_food_variant(&mut self, food_variant: Weak<RefCell<FoodVariant>>) {
        self.food_variant = food_variant;
    }

    pub fn get_data_source_instance(&self) -> Rc<RefCell<DataSourceInstance>> {
        self.data_source_instance.clone()
    }

    pub fn set_data_source_instance(&mut self, data_source_instance: Rc<RefCell<DataSourceInstance>>) {
        self.data_source_instance = data_source_instance;
    }
}
