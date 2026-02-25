use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::food_nutrition_data::FoodNutritionData;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataSource {
    id: Uuid,
    name: String,
    description: String,
}

impl DataSource {
    pub fn new(id: Option<Uuid>, name: String) -> Self {
        let id = match id {
            Some(id) => id,
            None => Uuid::new_v4(),
        };

        Self {
            id,
            name,
            description: String::new(),
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataSourceVersion {
    id: Uuid,
    data_source: Rc<RefCell<DataSource>>,
    version: String,
}

impl DataSourceVersion {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_data_source(&self) -> Rc<RefCell<DataSource>> {
        self.data_source.clone()
    }

    pub fn set_data_source(&mut self, data_source: Rc<RefCell<DataSource>>) {
        self.data_source = data_source;
    }

    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DataSourceInstance {
    id: Uuid,
    data_source_version: Rc<RefCell<DataSourceVersion>>,
    food_nutrition_data: Vec<Rc<RefCell<FoodNutritionData>>>, // Make a hashset
}

impl DataSourceInstance {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_data_source_version(&self) -> Rc<RefCell<DataSourceVersion>>{
        self.data_source_version.clone()
    }

    pub fn set_data_source_version(&mut self, data_source_version: Rc<RefCell<DataSourceVersion>>) {
        self.data_source_version = data_source_version;
    }

    pub fn get_food_nutrition_data(&self) -> Vec<Rc<RefCell<FoodNutritionData>>> {
        self.food_nutrition_data.clone()
    }
}
