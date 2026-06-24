use std::{cell::RefCell, collections::{BTreeSet}, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{food_instance::FoodInstance, food_taxonomy::FoodTaxonomy, food_variant_modifiers::{FoodAttribute, FoodTag, PreparationMethod}};

pub struct FoodVariant {
    name: String,
    description: String,
    preparation_method: Option<Rc<RefCell<PreparationMethod>>>, // change to vec or btreeset? Something could be baked andfried
    food_attributes: BTreeSet<Rc<RefCell<FoodAttribute>>>,
    food_tags: Vec<Rc<RefCell<FoodTag>>>,
    food_instances: Vec<Rc<RefCell<FoodInstance>>>,
    parent: Weak<RefCell<FoodTaxonomy>>,
}

impl FoodVariant {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            preparation_method: None,
            food_attributes: BTreeSet::new(),
            food_tags: Vec::new(),
            food_instances: Vec::new(),
            parent: food_taxonomy,
        }
    }

    pub fn new_rc_refcell(food_taxonomy: Weak<RefCell<FoodTaxonomy>>) -> Self {
        Rc::new(
            RefCell::new(
                Self {
                    name: String::new(),
                    description: String::new(),
                    preparation_method: None,
                    food_attributes: BTreeSet::new(),
                    food_tags: Vec::new(),
                    food_instances: Vec::new(),
                    parent: food_taxonomy,
                }
            )
        )
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

    pub fn get_preparation_method(&self) -> Option<Rc<RefCell<PreparationMethod>>> {
        self.preparation_method.clone()
    }

    pub fn set_preparation_method(&mut self, preparation_method: Option<Rc<RefCell<PreparationMethod>>>) {
        self.preparation_method = preparation_method;
    }

    pub fn get_food_attributes(&self) -> BTreeSet<Rc<RefCell<FoodAttribute>>> {
        self.food_attributes.clone()
    }

    pub fn set_food_attributes(&mut self, food_attribute_hashset: BTreeSet<Rc<RefCell<FoodAttribute>>>) {
        self.food_attributes = food_attribute_hashset;
    }

    pub fn push_food_attribute(&mut self, food_attribute: Rc<RefCell<FoodAttribute>>) -> bool {
        self.food_attributes.insert(food_attribute)
    }

    pub fn remove_food_attribute(&mut self, food_attribute: Rc<RefCell<FoodAttribute>>) -> bool {
        self.food_attributes.remove(&food_attribute)
    }

    pub fn get_food_tags(&self) -> Vec<Rc<RefCell<FoodTag>>> {
        self.food_tags.clone()
    }

    pub fn set_food_tags(&mut self, food_tags: Vec<Rc<RefCell<FoodTag>>>) {
        self.food_tags = food_tags;
    }

    pub fn push_food_tag(&mut self, food_tag: Rc<RefCell<FoodTag>>) {
        self.food_tags.push(food_tag)
    }

    pub fn remove_food_tag(&mut self, food_tag: Rc<RefCell<FoodTag>>) {
        if let Some(index) = self.food_tags.iter().position(|x| Rc::ptr_eq(x, &food_tag)) {
            self.food_tags.remove(index);
        }
    }

    pub fn get_food_instances(&self) -> Vec<Rc<RefCell<FoodInstance>>> {
        self.food_instances.clone()
    }

    pub fn set_food_instances(&mut self, food_instances: Vec<Rc<RefCell<FoodInstance>>>) {
        self.food_instances = food_instances
    }

    pub fn push_food_instance(&mut self, food_instance: Rc<RefCell<FoodInstance>>) {
        if !self.food_instances.iter().any(|fi| Rc::ptr_eq(fi, &food_instance)) {
            self.food_instances.push(food_instance)
        }
    }

    pub fn remove_food_instance(&mut self, food_instance: Rc<RefCell<FoodInstance>>) {
        if let Some(index) = self.food_instances.iter().position(|x| Rc::ptr_eq(x, &food_instance)) {
            self.food_instances.remove(index);
        }
    }

    pub fn get_parent(&self) -> Weak<RefCell<FoodTaxonomy>> {
        self.parent.clone()
    }

    pub fn get_parent_strong(&self) -> Rc<RefCell<FoodTaxonomy>> {
        if let Some(parent_strong) = self.get_parent().upgrade() {
            return parent_strong
        } else {
            panic!("Parent does not exist");
        }
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<FoodTaxonomy>>) {
        let parent_weak = Rc::downgrade(&parent);
        self.parent = parent_weak;
    }
    
    pub fn set_parent_weak(&mut self, parent_weak: Weak<RefCell<FoodTaxonomy>>) {
        self.parent = parent_weak;
    }
}
