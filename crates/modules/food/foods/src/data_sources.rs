use std::{cell::RefCell, rc::{Rc, Weak}};

use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DataSourceProvider {
    name: String,
    description: String,
    data_source_instances: Vec<Rc<RefCell<DataSourceInstance>>>,
}

impl DataSourceProvider {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            data_source_instances: Vec::new(),
        }
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
        for instance in self.get_data_source_instances() {
            if Rc::ptr_eq(&instance.borrow().get_data_source_version_strong(), &data_source_version) {
                return true;
            }
        }
        return false
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
    version: String,
    description: String,
}

impl DataSourceVersion {
    pub fn new(version: String) -> Self {
        Self {
            version,
            description: String::new(),
        }
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
    description: String,
    data_source_provider: Weak<RefCell<DataSourceProvider>>,
    data_source_version: Weak<RefCell<DataSourceVersion>>,
    nutrient_quantity_list: Rc<RefCell<NutrientQuantityList>>,
}

impl DataSourceInstance {
    pub fn new(data_source_provider: Rc<RefCell<DataSourceProvider>>, data_source_version: Rc<RefCell<DataSourceVersion>>, nutrient_quantity_list: Rc<RefCell<NutrientQuantityList>>) -> Self {
        let data_source_provider_weak = Rc::downgrade(&data_source_provider);
        let data_source_version_weak = Rc::downgrade(&data_source_version);
        Self {
            description: String::new(),
            data_source_provider: data_source_provider_weak,
            data_source_version: data_source_version_weak,
            nutrient_quantity_list,
        }
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn get_data_source_provider(&self) -> Weak<RefCell<DataSourceProvider>> {
        self.data_source_provider.clone()
    }

    pub fn get_data_source_provider_strong(&self) -> Rc<RefCell<DataSourceProvider>> {
        if let Some(dsp_rc) = self.data_source_provider.upgrade() {
            return dsp_rc
        } else {
            panic!("missing parent");
        }
    }

    pub fn set_data_source_provider(&mut self, data_source_provider: Rc<RefCell<DataSourceProvider>>) {
        let data_source_provider_weak = Rc::downgrade(&data_source_provider);

        self.data_source_provider = data_source_provider_weak;
    }

    pub fn get_data_source_version(&self) -> Weak<RefCell<DataSourceVersion>> {
        self.data_source_version.clone()
    }

    pub fn get_data_source_version_strong(&self) -> Rc<RefCell<DataSourceVersion>> {
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
