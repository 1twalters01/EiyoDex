use crate::{data_sources::DataSource, food_nutrition_data::FoodNutritionData, food_tag::FoodTag};
use nutrients::{
    nutrient::Nutrient, nutrient_amount::NutrientAmount, nutrient_list::NutrientAmountList,
    units::NutrientUnit,
};
use std::{
    cell::RefCell,
    collections::BTreeSet,
    rc::{Rc, Weak},
};
use units::{energy::Energy, energy_unit::EnergyUnit};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FoodInstance {
    id: Uuid,
    name: String,
    description: String,
    tags: BTreeSet<FoodTag>,
    food_data: BTreeSet<FoodNutritionData>,
}

impl FoodInstance {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            description: String::new(),
            tags: BTreeSet::new(),
            food_data: BTreeSet::new(),
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

    pub fn get_tags(&self) -> BTreeSet<FoodTag> {
        self.tags.clone()
    }

    pub fn set_tags(&mut self, tags: BTreeSet<FoodTag>) {
        self.tags = tags;
    }

    pub fn add_tag(&mut self, tag: FoodTag) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag: FoodTag) {
        self.tags.remove(&tag);
    }

    pub fn get_food_data(&self) -> BTreeSet<FoodNutritionData> {
        self.food_data.clone()
    }

    pub fn set_food_data(&mut self, food_data: BTreeSet<FoodNutritionData>) {
        self.food_data = food_data;
    }

    pub fn add_food_data(&mut self, food_data: FoodNutritionData) {
        self.food_data.insert(food_data);
    }

    pub fn remove_food_data(&mut self, food_data: FoodNutritionData) {
        self.food_data.remove(&food_data);
    }

    pub fn get_calories(&self, data_source_uuid: Uuid) -> Energy {
        let mut food_data: Option<FoodNutritionData> = None;
        for data in self.food_data.clone() {
            if data.get_data_source().get_id() == data_source_uuid {
                food_data = Some(data)
            }
        }

        let mut energy: Energy = Energy::new(0f64, EnergyUnit::Kilocalorie);
        match food_data {
            Some(data) => {
                for nutrient_amount in data.get_nutrient_amount_list().get_nutrient_amounts() {
                    match nutrient_amount.get_nutrient() {
                        Some(nutrient) => {
                            if nutrient.borrow().get_name() == "Calories" {
                                let unit = nutrient.borrow().get_main_unit();
                                match unit {
                                    NutrientUnit::Energy(energy_unit) => {
                                        energy =
                                            Energy::new(nutrient_amount.get_value(), energy_unit);
                                        break;
                                    }
                                    _ => {}
                                };
                            }
                        }
                        None => {}
                    }
                }
            }
            None => energy = Energy::new(0f64, EnergyUnit::Kilocalorie),
        };
        return energy;
    }

    pub fn get_nutrient_amount(
        &self,
        nutrient: Nutrient,
        data_source_uuid: Uuid,
    ) -> Option<NutrientAmount> {
        None
    }
}
