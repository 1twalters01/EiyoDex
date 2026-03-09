use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Utc};
use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

use crate::{data_sources::{DataSourceInstance, DataSourceProvider, DataSourceVersion}, food_variant::FoodVariant};

#[derive(Clone)]
pub struct FoodQuantity {
    id: Uuid,
    value: f64,
    food_variant: Rc<RefCell<FoodVariant>>,
    data_source_provider: Rc<RefCell<DataSourceProvider>>,
    data_source_version: Rc<RefCell<DataSourceVersion>>,
    created_at: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}

impl FoodQuantity {
    pub fn new(value: f64, food_variant: Rc<RefCell<FoodVariant>>, data_source_provider: Rc<RefCell<DataSourceProvider>>, data_source_version: Rc<RefCell<DataSourceVersion>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            value,
            food_variant,
            data_source_provider,
            data_source_version,
            created_at: Utc::now(),
            last_modified: Utc::now(),
        }
    }

    pub fn new_rc_refcell(value: f64, food_variant: Rc<RefCell<FoodVariant>>, data_source_provider: Rc<RefCell<DataSourceProvider>>, data_source_version: Rc<RefCell<DataSourceVersion>>) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {
                    id: Uuid::new_v4(),
                    value,
                    food_variant,
                    data_source_provider,
                    data_source_version,
                    created_at: Utc::now(),
                    last_modified: Utc::now(),
                }
            )
        )
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    
    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_food_variant(&self) -> Rc<RefCell<FoodVariant>> {
        self.food_variant.clone()
    }

    pub fn set_food_variant(&mut self, food_variant: Rc<RefCell<FoodVariant>>) {
        self.food_variant = food_variant;
    }

    pub fn get_data_source_provider(&self) -> Rc<RefCell<DataSourceProvider>> {
        self.data_source_provider.clone()
    }

    pub fn set_data_source(&mut self, data_source_provider: Rc<RefCell<DataSourceProvider>>, data_source_version: Rc<RefCell<DataSourceVersion>>) -> Result<(), &'static str> {
        // Add verification that it is valid?
        if data_source_provider.borrow().is_data_source_version_valid(data_source_version.clone()) {
            self.data_source_provider = data_source_provider;
            self.data_source_version = data_source_version;
            return Ok(())
        } else {
            return Err("Invalid data source provider and version combination")
        }
    }

    pub fn get_data_source_version(&self) -> Rc<RefCell<DataSourceVersion>> {
        self.data_source_version.clone()
    }

    pub fn set_data_source_version(&mut self, data_source_version: Rc<RefCell<DataSourceVersion>>) -> Result<(), &'static str> {
        if self.data_source_provider.borrow().is_data_source_version_valid(data_source_version.clone()) {
            self.data_source_version = data_source_version;
            return Ok(())
        } else {
            return Err("Invalid data source provider and version combination")
        }
    }

    pub fn get_data_source_instance(&self) -> Rc<RefCell<DataSourceInstance>> {
        self.data_source_provider.borrow().get_data_source_instance(self.data_source_version.clone()).expect("Invalid data source provider and version combination")
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn set_created_at(&mut self, datetime: DateTime<Utc>) {
        self.created_at = datetime;
    }

    pub fn get_last_modified(&self) -> DateTime<Utc> {
        self.last_modified
    }

    pub fn set_last_modified(&mut self, datetime: DateTime<Utc>) {
        self.last_modified = datetime;
    }

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        self.get_data_source_instance().borrow().get_calories()
    }

    pub fn get_nutrient_quantity(&self, nutrient: Rc<RefCell<Nutrient>>) -> Option<NutrientQuantity> {
        match self.get_data_source_instance().borrow().get_nutrient_quantity(nutrient) {
            Some(amount) => Some(amount * self.value),
            None => None,
        }
    }

    pub fn contains_nutrient(&self, nutrient: Rc<RefCell<Nutrient>>) -> bool {
        self.get_data_source_instance().borrow().contains_nutrient(nutrient)
    }
}
