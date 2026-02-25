use std::{cell::RefCell, collections::HashSet, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::food_taxonomy::FoodTaxonomy;

pub struct FoodVariant {
    id: Uuid,
    name: String,
    description: String,
    preparation_method: Rc<RefCell<PreparationMethod>>,
    food_attribute: HashSet<Rc<RefCell<FoodAttribute>>>,
    food_tags: Vec<Rc<RefCell<FoodTags>>>,
    parent: Weak<RefCell<FoodTaxonomy>>,
}

