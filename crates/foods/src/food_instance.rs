use crate::sources::DataSource;
use nutrients::nutrient::Nutrient;
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use uuid::Uuid;

pub struct FoodCategory {
    parents: Vec<Rc<RefCell<FoodCategory>>>,
    children: Vec<Rc<RefCell<FoodCategory>>>,
    food_instance: Option<FoodInstance>,
}

pub struct FoodInstance {
    id: Uuid,
    name: String,
    favourite: bool,
    tags: BTreeSet<FoodTag>,
    food_data: BTreeSet<FoodData>,
}

pub struct FoodData {
    data_source: DataSource,
    nutrients: BTreeSet<Nutrient>,
}

pub struct FoodTag {
    id: Uuid,
    name: String,
    description: String,
    applicable_categories: Vec<FoodCategory>,
}
