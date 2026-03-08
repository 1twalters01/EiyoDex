use std::{cell::RefCell, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{data_sources::DataSourceInstance, food_variant::FoodVariant};

pub struct FoodInstance {
    id: Uuid,
    food_variant: Weak<RefCell<FoodVariant>>,
    data_source_instance: Rc<RefCell<DataSourceInstance>>,
}

impl FoodInstance {
    pub fn get_id() {}
    pub fn set_id() {}
    pub fn get_food_variant() {}
    pub fn set_food_variant() {}
    pub fn get_data_source() {}
    pub fn set_data_source() {}
}
