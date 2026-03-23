use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::nutrient::Nutrient;

#[derive(Debug, Clone)]
pub struct NutrientList {
    id: Uuid,
    nutrients: Vec<Rc<RefCell<Nutrient>>>,
}

impl NutrientList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            nutrients: Vec::new(),
        }
    }

    pub fn from_vec(nutrients: Vec<Rc<RefCell<Nutrient>>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            nutrients: nutrients,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_nutrients(&self) -> Vec<Rc<RefCell<Nutrient>>> {
        self.nutrients.clone()
    }

    pub fn set_nutrients(&mut self, nutrients: Vec<Rc<RefCell<Nutrient>>>) {
        self.nutrients = nutrients
    }

    pub fn push(&mut self, nutrient: Rc<RefCell<Nutrient>>) {
        self.nutrients.push(nutrient)
    }

    pub fn remove(&mut self, nutrient: Rc<RefCell<Nutrient>>) {
        self.nutrients.retain(|n| Rc::ptr_eq(n, &nutrient))
    }
}
