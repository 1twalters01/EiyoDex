use std::{cell::RefCell, rc::{Rc, Weak}};

use uuid::Uuid;

use crate::{food_category::FoodCategory, price_metadata::PriceMetadata};

pub struct FoodTaxonomy {
    id: Uuid,
    name: String,
    description: String,
    price_metadata: Vec<PriceMetadata>,
    parent: Weak<RefCell<FoodCategory>>,
    children: Vec<Rc<RefCell<FoodVariants>>>,
}

