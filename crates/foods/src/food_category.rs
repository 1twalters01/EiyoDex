use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::food_instance::FoodInstance;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodCategory {
    id: Uuid,
    name: String,
    description: String,
    parents: Vec<Rc<RefCell<FoodCategory>>>,
    // Make parents a Vec<Weak<RefCell>> as parents are not owned by children
    children: Vec<Rc<RefCell<FoodCategory>>>,
    food_instance: Option<FoodInstance>,
}

impl FoodCategory {
    pub fn new() -> FoodCategory {
        FoodCategory {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            parents: Vec::new(),
            children: Vec::new(),
            food_instance: None,
        }
    }

    pub fn get_parents(&self) -> Vec<Rc<RefCell<FoodCategory>>> {
        self.parents.clone()
    }

    pub fn set_parents(&mut self, food_categories: Vec<Rc<RefCell<FoodCategory>>>) {
        self.parents = food_categories;
    }

    pub fn add_parent(&mut self, food_category: FoodCategory) {
        self.parents.push(Rc::new(RefCell::new(food_category)));
    }

    pub fn add_parent_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        self.parents.push(food_category);
    }

    pub fn remove_parent(&mut self, food_category: FoodCategory) {
        if let Some(pos) = self
            .parents
            .iter()
            .position(|x| *x.borrow() == food_category)
        {
            self.parents.remove(pos);
        }
    }

    pub fn remove_parent_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        if let Some(pos) = self
            .parents
            .iter()
            .position(|x| Rc::ptr_eq(x, &food_category))
        {
            self.parents.remove(pos);
        }
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<FoodCategory>>> {
        self.children.clone()
    }

    pub fn set_children(&mut self, food_categories: Vec<Rc<RefCell<FoodCategory>>>) {
        self.children = food_categories;
    }

    pub fn add_child(&mut self, food_category: FoodCategory) {
        self.children.push(Rc::new(RefCell::new(food_category)));
    }

    pub fn add_child_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        self.children.push(food_category);
    }

    pub fn remove_child(&mut self, food_category: FoodCategory) {
        if let Some(pos) = self
            .children
            .iter()
            .position(|x| *x.borrow() == food_category)
        {
            self.children.remove(pos);
        }
    }

    pub fn remove_child_rc_refcell(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        if let Some(pos) = self
            .children
            .iter()
            .position(|x| Rc::ptr_eq(x, &food_category))
        {
            self.children.remove(pos);
        }
    }

    pub fn get_food_instance(&self) -> Option<FoodInstance> {
        self.food_instance.clone()
    }

    pub fn set_food_instance(&mut self, food_instance: Option<FoodInstance>) {
        self.food_instance = food_instance;
    }
}

