use std::{cell::RefCell, collections::{BTreeSet}, rc::Rc};

use uuid::Uuid;

use crate::food_category::FoodCategory;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodTag {
    id: Uuid,
    name: String,
    description: String,
    // applicable_category_ids: BTreeSet<Uuid>, // Will want to save Uuids into db
    // #[serde(skip)]
    // applicable_categories: Vec<Rc<RefCell<FoodCategory>>>,
    applicable_categories: BTreeSet<Rc<RefCell<FoodCategory>>>,
}

impl FoodTag {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            description: String::new(),
            applicable_categories: BTreeSet::new(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
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

    pub fn get_applicable_categories(&self) -> BTreeSet<Rc<RefCell<FoodCategory>>> {
        self.applicable_categories.clone()
    }

    pub fn set_applicable_categories(&mut self, food_categories: BTreeSet<Rc<RefCell<FoodCategory>>>) {
        self.applicable_categories = food_categories;
    }

    pub fn add_applicable_category(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        self.applicable_categories.insert(food_category);
    }

    pub fn extend_applicable_categories(&mut self, food_categories: Vec<Rc<RefCell<FoodCategory>>>) {
        self.applicable_categories.extend(food_categories);
    }

    pub fn remove_applicable_category(&mut self, food_category: Rc<RefCell<FoodCategory>>) {
        self.applicable_categories.retain(|category| !Rc::ptr_eq(category, &food_category));
    }
}

