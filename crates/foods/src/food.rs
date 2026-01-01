use units::energy::Energy;
use uuid::Uuid;

use crate::{food_instance::FoodInstance, price_metadata::PriceMetadata};


// TODO - per whatever unit (100g?)
#[derive(Clone, PartialEq)]
pub struct Food {
    id: Uuid,
    name: String,
    food_instance: FoodInstance,
    price_metadata: Option<PriceMetadata>,
}

impl Food {
    pub fn new(
        id: Option<Uuid>,
        name: String,
        food_instance: FoodInstance,
        price_metadata: Option<PriceMetadata>,
    ) -> Self {
        let id: Uuid = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            price_metadata,
            food_instance,
        }
    }

    pub fn get_calories(&self, food_data_uuid: Uuid) -> Energy {
        self.food_instance.get_calories(food_data_uuid)
    }
}
