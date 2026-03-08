use std::{cell::RefCell, rc::{Rc, Weak}};

use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity, nutrient_quantity_list::{self, NutrientQuantityList}};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

use crate::data_sources::DataSourceInstance;

#[derive(Debug, Clone)]
pub struct FoodNutritionData {
    id: Uuid,
    data_source_instance: Weak<RefCell<DataSourceInstance>>,
    description: String,
    nutrient_quantity_list: Rc<RefCell<NutrientQuantityList>>,
}

impl FoodNutritionData {
    pub fn new(data_source_instance: Weak<RefCell<DataSourceInstance>>, nutrient_quantity_list: Rc<RefCell<NutrientQuantityList>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            data_source_instance,
            description: String::new(),
            nutrient_quantity_list,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_data_source(&self) -> Weak<RefCell<DataSourceInstance>> {
        self.data_source_instance.clone()
    }

    pub fn set_data_source(&mut self, data_source_instance: Weak<RefCell<DataSourceInstance>>) {
        self.data_source_instance = data_source_instance;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_nutrient_quantity_list(&self) -> Rc<RefCell<NutrientQuantityList>> {
        self.nutrient_quantity_list.clone()
    }

    pub fn set_nutrient_quantity_list(&mut self, nutrient_quantity_list: Rc<RefCell<NutrientQuantityList>>) {
        self.nutrient_quantity_list = nutrient_quantity_list;
    }

    pub fn add_nutrient_quantity(&mut self, nutrient_quantity: NutrientQuantity) -> bool {
        self.nutrient_quantity_list.borrow_mut().push(nutrient_quantity)
    }

    pub fn extend_nutrient_quantity_vec(&mut self, nutrient_amount_vec: Vec<NutrientQuantity>) {
        self.nutrient_quantity_list.borrow_mut().extend(nutrient_amount_vec)
    }

    pub fn remove_nutrient(&mut self, nutrient_quantity: &NutrientQuantity) {
        self.nutrient_quantity_list.borrow_mut().remove(nutrient_quantity);
    }

    pub fn get_nutrient_quantity(&self, nutrient: Rc<RefCell<Nutrient>>) -> Option<NutrientQuantity> {
        let nutrient_quantity_hashset = self.nutrient_quantity_list.borrow().get_nutrient_quantities();
        nutrient_quantity_hashset.iter().find(|nutrient_quantity| Rc::ptr_eq(&nutrient_quantity.get_nutrient(), &nutrient)).cloned()
    }

    pub fn contains_nutrient(&self, nutrient: Rc<RefCell<Nutrient>>) -> bool {
        let nutrient_quantity_hashset = self.nutrient_quantity_list.borrow().get_nutrient_quantities();
        nutrient_quantity_hashset.iter().find(|nutrient_quantity| Rc::ptr_eq(&nutrient_quantity.get_nutrient(), &nutrient)).is_some()
    }

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        self.nutrient_quantity_list.borrow().get_calories()
    }
}
