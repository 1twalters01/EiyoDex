use nutrients::{nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList};
use uuid::Uuid;

use crate::data_sources::DataSource;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodNutritionData {
    id: Uuid,
    data_source: DataSource,
    nutrient_quantity_list: NutrientQuantityList,
}

impl FoodNutritionData {
    pub fn new(data_source: DataSource, nutrient_quantity_list: NutrientQuantityList) -> Self {
        Self {
            id: Uuid::new_v4(),
            data_source,
            nutrient_quantity_list,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_data_source(&self) -> DataSource {
        self.data_source.clone()
    }

    pub fn set_data_source(&mut self, data_source: DataSource) {
        self.data_source = data_source;
    }

    pub fn get_nutrient_amount_list(&self) -> NutrientQuantityList {
        self.nutrient_quantity_list.clone()
    }

    pub fn set_nutrient_amount_list(&mut self, nutrient_quantity_list: NutrientQuantityList) {
        self.nutrient_quantity_list = nutrient_quantity_list;
    }

    pub fn add_nutrient_amount(&mut self, nutrient_amount: NutrientQuantity) -> bool {
        self.nutrient_quantity_list.push(nutrient_amount)
    }

    pub fn extend_nutrient_amounts(&mut self, nutrient_amount_vec: Vec<NutrientQuantity>) {
        self.nutrient_quantity_list.extend(nutrient_amount_vec)
    }

    pub fn remove_nutrient(&mut self, nutrient_amount: &NutrientQuantity) {
        self.nutrient_quantity_list.remove(nutrient_amount);
    }
}
