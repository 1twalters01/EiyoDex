use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::nutrient::Nutrient;

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientList {
    // id: Uuid,
    name: String,
    description: String,
    nutrients: Vec<Rc<RefCell<Nutrient>>>,
}

impl NutrientList {
    pub fn new() -> Self {
        Self {
            // id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            nutrients: Vec::new(),
        }
    }

    pub fn from_vec(nutrients: Vec<Rc<RefCell<Nutrient>>>) -> Self {
        Self {
            // id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            nutrients: nutrients,
        }
    }

    // pub fn get_id(&self) -> Uuid {
    //     self.id
    // }
    //
    // pub fn set_id(&mut self, id: Uuid) {
    //     self.id = id;
    // }

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

    pub fn get_nutrients(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        self.nutrients.clone()
    }

    pub fn set_nutrients(&mut self, nutrients: Vec<Rc<RefCell<Nutrient>>>) {
        self.nutrients = nutrients
    }

    pub fn get_nutrient_names(&self) -> Vec<String> {
        self.nutrients
            .iter()
            .map(|nutrient| nutrient.borrow().get_name())
            .collect()
    }

    pub fn push(&mut self, nutrient: Rc<RefCell<Nutrient>>) {
        self.nutrients.push(nutrient)
    }

    pub fn extend(&mut self, nutrients: Vec<Rc<RefCell<Nutrient>>>) {
        self.nutrients.extend(nutrients);
    }

    pub fn remove(&mut self, nutrient: Rc<RefCell<Nutrient>>) {
        self.nutrients.retain(|n| !Rc::ptr_eq(n, &nutrient))
    }

    pub fn sort_by_name(&mut self) {
        self.nutrients.sort_by(|a, b| {
            let name_a = a.borrow().get_name();
            let name_b = b.borrow().get_name();
            name_a.cmp(&name_b)
        });
    }
}
