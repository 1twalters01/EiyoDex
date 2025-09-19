use uuid::Uuid;
use crate::sources::DataSource;
use nutrients::nutrient::Nutrient;
use std::collections::BTreeSet;
use units::currency::Currency;

pub struct Food {
    id: Uuid,
    name: String,
    cost: Currency,
}

impl Food {
    pub fn new() {}
}

pub struct FoodInstance {
    id: Uuid,
    name: String,
    data_source: DataSource,
    nutrients: BTreeSet<Nutrient>,
}
