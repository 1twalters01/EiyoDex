use uuid::Uuid;
use nutrients::nutrient::Nutrient;

pub struct Food {
    id: Uuid,
    name: String,
}

pub struct FoodInstance {
    id: Uuid,
    name: String,
    nutrients: Vec<Nutrient>,
}
