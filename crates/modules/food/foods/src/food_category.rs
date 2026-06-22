use std::{cell::RefCell, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::food_taxonomy::FoodTaxonomy;

pub enum FoodCategoryChild {
    FoodCategory(Rc<RefCell<FoodCategory>>),
    FoodTaxonomy(Rc<RefCell<FoodTaxonomy>>),
}

pub struct FoodCategory {
    name: String,
    description: String,
    parent: Option<Weak<RefCell<FoodCategory>>>,
    children: Vec<Rc<RefCell<FoodCategoryChild>>>,
}

impl FoodCategory {
    pub fn new(food_category: Option<Weak<RefCell<FoodCategory>>>) -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            parent: food_category,
            children: Vec::new(),
        }
    }
    
    pub fn new_rc_refcell(food_category: Option<Weak<RefCell<FoodCategory>>>) -> Rc<RefCell<Self>> {
        Rc::new(
            Refcell::new(
                Self {
                    name: String::new(),
                    description: String::new(),
                    parent: food_category,
                    children: Vec::new(),
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

    pub fn get_parent(&self) -> Option<Weak<RefCell<FoodCategory>>> {
        self.parent.clone()
    }

    pub fn set_parent(&mut self, food_category: Option<Weak<RefCell<FoodCategory>>>) {
        self.parent = food_category;
    }

    pub fn get_children(&mut self) -> Vec<Rc<RefCell<FoodCategoryChild>>> {
        self.children.clone()
    }

    pub fn set_children(&mut self, children: Vec<Rc<RefCell<FoodCategoryChild>>>) {
        self.children = children;
    }

    pub fn push_child(&mut self, child: Rc<RefCell<FoodCategoryChild>>) {
        self.children.push(child)
    }

    pub fn remove_child(&mut self, child: Rc<RefCell<FoodCategoryChild>>) {
        self.children.retain(|c| !Rc::ptr_eq(c, &child))
    }
}
