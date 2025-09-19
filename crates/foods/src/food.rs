use uuid::Uuid;
use crate::sources::DataSource;
use nutrients::nutrient::Nutrient;
use std::collections::BTreeSet;

pub struct Food {
    id: Uuid,
    name: String,
}

pub struct FoodInstance {
    id: Uuid,
    name: String,
    data_source: DataSource,
    nutrients: BTreeSet<Nutrient>,
}
