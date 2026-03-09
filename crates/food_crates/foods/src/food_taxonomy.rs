use std::{cell::RefCell, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{food_category::FoodCategory, food_variant::FoodVariant, price_metadata::PriceMetadata};

pub struct FoodTaxonomy {
    id: Uuid,
    name: String,
    description: String,
    price_metadata: Vec<Rc<RefCell<PriceMetadata>>>,
    parent: Weak<RefCell<FoodCategory>>,
    children: Vec<Rc<RefCell<FoodVariant>>>,
}

impl FoodTaxonomy {
    pub fn new(name: String, parent: &Rc<RefCell<FoodCategory>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            price_metadata: Vec::new(),
            parent: Rc::downgrade(parent),
            children: Vec::new()
        }
    }
    
    pub fn new_rc_refcell(name: String, parent: &Rc<RefCell<FoodCategory>>) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {
                    id: Uuid::new_v4(),
                    name,
                    description: String::new(),
                    price_metadata: Vec::new(),
                    parent: Rc::downgrade(parent),
                    children: Vec::new()
                }
            )
        )
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
        self.name = name
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_price_metadata(&self) -> Vec<Rc<RefCell<PriceMetadata>>> {
        self.price_metadata.clone()
    }

    pub fn set_price_metadata(&mut self, price_metadata: Vec<Rc<RefCell<PriceMetadata>>>) {
        self.price_metadata = price_metadata;
    }

    pub fn push_price_metadata(&mut self, price_metadata: Rc<RefCell<PriceMetadata>>) {
        self.price_metadata.push(price_metadata);
    }

    pub fn remove_price_metadata(&mut self, price_metadata: Rc<RefCell<PriceMetadata>>) {
        self.price_metadata.retain(|pm| !Rc::ptr_eq(pm, &price_metadata));
    }

    pub fn get_parent(&self) -> Weak<RefCell<FoodCategory>> {
        self.parent.clone()
    }

    pub fn set_parent(&mut self, parent: &Rc<RefCell<FoodCategory>>) {
        self.parent = Rc::downgrade(parent);
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<FoodVariant>>> {
        self.children.clone()
    }

    pub fn set_children(&mut self, children:Vec<Rc<RefCell<FoodVariant>>>) {
        self.children = children;
    }

    pub fn push_child(&mut self, child: Rc<RefCell<FoodVariant>>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: Rc<RefCell<FoodVariant>>) {
        self.children.retain(|c| !Rc::ptr_eq(c, &child));
    }
}
