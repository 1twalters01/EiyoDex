use std::collections::BTreeSet;

use nutrients::{nutrient::Nutrient, nutrient_amount::NutrientAmount};
use units::energy::Energy;
use uuid::Uuid;

use crate::{food_instance::FoodInstance, price_metadata::PriceMetadata};


// TODO - per whatever unit (100g?)
#[derive(Clone, PartialEq)]
pub struct Food {
    id: Uuid,
    name: String,
    description: String,
    food_instances: BTreeSet<FoodInstance>,
    food_instance_uuid: Uuid,
    data_source_uuid: Uuid,
    price_metadata: Option<PriceMetadata>,
    // Move price metadata to be inside food instances?
    // Or create a container type with food instances and price metadata inside?
}

impl Food {
    pub fn new(
        id: Option<Uuid>,
        name: String,
        food_instances: Vec<FoodInstance>,
        food_instance_uuid: Uuid,
        data_source_uuid: Uuid,
        price_metadata: Option<PriceMetadata>,
    ) -> Self {
        let id: Uuid = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };
        let food_instances_set: BTreeSet<FoodInstance> = food_instances.iter().cloned().collect();

        Self {
            id,
            name,
            description: String::new(),
            price_metadata,
            food_instances: food_instances_set,
            food_instance_uuid,
            data_source_uuid,
        }
    }

    pub fn get_calories(&self, food_instance_uuid: Uuid, data_source_uuid: Uuid) -> Energy {
        let food_instance = self.food_instances
            .iter()
            .filter(|instance| instance.get_id() == food_instance_uuid)
            .cloned()
            .collect::<Vec<FoodInstance>>()
            .first().cloned();
        match food_instance {
            Some(instance) => instance.get_calories(data_source_uuid),
            None => Energy::new(0f64, units::energy::EnergyUnit::Kilocalorie),
        }
    }

    pub fn get_nutrient_amount(&self, nutrient: Nutrient, food_instance_uuid: Uuid, data_source_uuid: Uuid) -> Option<NutrientAmount> {
        let food_instance = self.food_instances
            .iter()
            .filter(|instance| instance.get_id() == food_instance_uuid)
            .cloned()
            .collect::<Vec<FoodInstance>>()
            .first().cloned();
        match food_instance {
            Some(instance) => instance.get_nutrient_amount(nutrient, data_source_uuid),
            None => { None },
        }
    }
}
