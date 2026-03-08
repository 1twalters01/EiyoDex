use std::{cell::RefCell, collections::{BTreeSet}, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{food_instance::FoodInstance, food_taxonomy::FoodTaxonomy, food_variant_modifiers::{FoodAttribute, FoodTags, PreparationMethod}};

pub struct FoodVariant {
    id: Uuid,
    name: String,
    description: String,
    preparation_method: Rc<RefCell<PreparationMethod>>,
    food_attribute: BTreeSet<Rc<RefCell<FoodAttribute>>>,
    food_tags: Vec<Rc<RefCell<FoodTags>>>,
    food_instances: BTreeSet<Rc<RefCell<FoodInstance>>>,
    parent: Weak<RefCell<FoodTaxonomy>>,
}

impl FoodVariant {
    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_preparation_method(&self) -> Rc<RefCell<PreparationMethod>> {
        self.preparation_method.clone()
    }

    pub fn set_preparation_method(&mut self, preparation_method: Rc<RefCell<PreparationMethod>>) {
        self.preparation_method = preparation_method;
    }

    pub fn get_food_attribute(&self) -> BTreeSet<Rc<RefCell<FoodAttribute>>> {
        self.food_attribute.clone()
    }

    pub fn set_food_attribute(&mut self, food_attribute_hashset: BTreeSet<Rc<RefCell<FoodAttribute>>>) {
        self.food_attribute = food_attribute_hashset;
    }

    pub fn push_food_attribute(&mut self, food_attribute: Rc<RefCell<FoodAttribute>>) {
        self.food_attribute.insert(food_attribute);
    }
}
