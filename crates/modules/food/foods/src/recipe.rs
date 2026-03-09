use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::food_quantity::FoodQuantity;

pub struct FoodQuantityList {
    id: Uuid,
    food_quantities: Vec<Rc<RefCell<FoodQuantity>>>,
}

impl FoodQuantityList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            food_quantities: Vec::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_food_quantities(&self) -> Vec<Rc<RefCell<FoodQuantity>>> {
        self.food_quantities.clone()
    }

    pub fn set_food_quantities(&mut self, food_quantities: Vec<Rc<RefCell<FoodQuantity>>>) {
        self.food_quantities = food_quantities;
    }

    pub fn push_food_quantities(&mut self, food_quantity: Rc<RefCell<FoodQuantity>>) {
        self.food_quantities.push(food_quantity)
    }

    pub fn remove_food_quantities(&mut self, food_quantity: Rc<RefCell<FoodQuantity>>) {
        if let Some(index) = self.food_quantities.iter().position(|fq| Rc::ptr_eq(fq, &food_quantity)) {
            self.food_quantities.remove(index);
        }
    }
}
