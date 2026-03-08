use std::{cell::RefCell, rc::{Rc, Weak}};

use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

use crate::food_nutrition_data::FoodNutritionData;

#[derive(Debug, Clone)]
pub struct DataSourceProvider {
    id: Uuid,
    name: String,
    description: String,
    data_source_instances: Vec<Rc<RefCell<DataSourceInstance>>>,
}

impl DataSourceProvider {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            description: String::new(),
            data_source_instances: Vec::new(),
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

    pub fn get_data_source_instances(&self) -> Vec<Rc<RefCell<DataSourceInstance>>> {
        self.data_source_instances.clone()
    }

    pub fn is_data_source_version_valid(&self, data_source_version: Rc<RefCell<DataSourceVersion>>) -> bool {
        true
    }

    pub fn get_data_source_instance(&self, data_source_version: Rc<RefCell<DataSourceVersion>>) -> Option<Rc<RefCell<DataSourceInstance>>> {
        let data_source_instance_vec = self.get_data_source_instances();
        data_source_instance_vec
            .into_iter()
            .find(|dsi| {
                Rc::ptr_eq(
                    &dsi.borrow().get_data_source_version_strong(),
                    &data_source_version,
                )
            })
    }

    pub fn set_data_source_instances(&mut self, data_source_instances: Vec<Rc<RefCell<DataSourceInstance>>>) {
        self.data_source_instances = data_source_instances;
    }

    pub fn push_data_source_instances(&mut self, data_source_instance: Rc<RefCell<DataSourceInstance>>) {
        self.data_source_instances.push(data_source_instance);
    }

    pub fn remove_data_source_instance(&mut self, data_source_instance: Rc<RefCell<DataSourceInstance>>) {
        self.data_source_instances.retain(|dsi| !Rc::ptr_eq(&data_source_instance, dsi));
    }
}

#[derive(Debug, Clone)]
pub struct DataSourceVersion {
    id: Uuid,
    version: String,
    description: String,
}

impl DataSourceVersion {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

#[derive(Debug, Clone)]
pub struct DataSourceInstance {
    id: Uuid,
    description: String,
    data_source_provider: Weak<RefCell<DataSourceProvider>>,
    data_source_version: Weak<RefCell<DataSourceVersion>>,
    food_nutrition_data: Rc<RefCell<FoodNutritionData>>
}

impl DataSourceInstance {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_data_source_version(&self) -> Weak<RefCell<DataSourceVersion>>{
        self.data_source_version.clone()
    }

    pub fn get_data_source_version_strong(&self) -> Rc<RefCell<DataSourceVersion>>{
        if let Some(dsv_rc) = self.data_source_version.upgrade() {
            return dsv_rc
        } else {
            panic!("missing parent");
        }
    }

    pub fn set_data_source_version(&mut self, data_source_version: Rc<RefCell<DataSourceVersion>>) {
        let data_source_version_weak = Rc::downgrade(&data_source_version);

        self.data_source_version = data_source_version_weak;
    }

    pub fn get_food_nutrition_data(&self) -> Rc<RefCell<FoodNutritionData>> {
        self.food_nutrition_data.clone()
    }

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        self.food_nutrition_data.borrow().get_calories()
    }

    pub fn get_nutrient_quantity(&self, nutrient: Rc<RefCell<Nutrient>>) -> Option<NutrientQuantity> {
        self.food_nutrition_data.borrow().get_nutrient_quantity(nutrient)
    }
    pub fn contains_nutrient(&self, nutrient: Rc<RefCell<Nutrient>>) -> bool {
        self.food_nutrition_data.borrow().contains_nutrient(nutrient)
    }
}
