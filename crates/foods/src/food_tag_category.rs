use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodTagCategory {
    id: Uuid,
    name: String,
    description: String,
    parents: Vec<Rc<RefCell<FoodTagCategory>>>,
    // Make parents a Vec<Weak<RefCell>> as parents are not owned by children
    children: Vec<Rc<RefCell<FoodTagCategory>>>,
}

impl FoodTagCategory {
    pub fn new() -> FoodTagCategory {
        FoodTagCategory {
            id: Uuid::new_v4(),
            name: String::new(),
            description: String::new(),
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn get_parents(&self) -> Vec<Rc<RefCell<FoodTagCategory>>> {
        self.parents.clone()
    }

    pub fn set_parents(&mut self, food_categories: Vec<Rc<RefCell<FoodTagCategory>>>) {
        self.parents = food_categories;
    }

    pub fn add_parent(&mut self, food_category: FoodTagCategory) {
        self.parents.push(Rc::new(RefCell::new(food_category)));
    }

    pub fn add_parent_rc_refcell(&mut self, food_category: Rc<RefCell<FoodTagCategory>>) {
        self.parents.push(food_category);
    }

    pub fn remove_parent(&mut self, food_category: FoodTagCategory) {
        if let Some(pos) = self
            .parents
            .iter()
            .position(|x| *x.borrow() == food_category)
        {
            self.parents.remove(pos);
        }
    }

    pub fn remove_parent_rc_refcell(&mut self, food_category: Rc<RefCell<FoodTagCategory>>) {
        if let Some(pos) = self
            .parents
            .iter()
            .position(|x| Rc::ptr_eq(x, &food_category))
        {
            self.parents.remove(pos);
        }
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<FoodTagCategory>>> {
        self.children.clone()
    }

    pub fn set_children(&mut self, food_categories: Vec<Rc<RefCell<FoodTagCategory>>>) {
        self.children = food_categories;
    }

    pub fn add_child(&mut self, food_category: FoodTagCategory) {
        self.children.push(Rc::new(RefCell::new(food_category)));
    }

    pub fn add_child_rc_refcell(&mut self, food_category: Rc<RefCell<FoodTagCategory>>) {
        self.children.push(food_category);
    }

    pub fn remove_child(&mut self, food_category: FoodTagCategory) {
        if let Some(pos) = self
            .children
            .iter()
            .position(|x| *x.borrow() == food_category)
        {
            self.children.remove(pos);
        }
    }

    pub fn remove_child_rc_refcell(&mut self, food_category: Rc<RefCell<FoodTagCategory>>) {
        if let Some(pos) = self
            .children
            .iter()
            .position(|x| Rc::ptr_eq(x, &food_category))
        {
            self.children.remove(pos);
        }
    }
}
