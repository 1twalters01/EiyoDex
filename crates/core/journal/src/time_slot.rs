use std::{cell::RefCell, rc::Rc};

use chrono::NaiveDateTime;
use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use units::energy::{quantity::EnergyQuantity, unit::EnergyUnit};
use uuid::Uuid;

use crate::entry::Entry;

// (e.g. breakfast, lunch, snack 1)
#[derive(Clone)]
pub struct TimeSlot {
    id: Uuid,
    name: String,
    description: String,
    datetime_created: NaiveDateTime,
    datetime_last_modified: NaiveDateTime,
    entries: Vec<Rc<RefCell<Entry>>>,
}

impl TimeSlot {
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

    pub fn get_datetime_created(&self) -> NaiveDateTime {
        self.datetime_created
    }

    pub fn get_datetime_last_modified(&self) -> NaiveDateTime {
        self.datetime_last_modified
    }

    pub fn set_datetime_last_modified(&mut self, datetime: NaiveDateTime) {
        self.datetime_last_modified = datetime;
    }

    pub fn get_entries(&mut self) -> Vec<Rc<RefCell<Entry>>> {
        self.entries.clone()
    }

    pub fn set_entries(&mut self, entries: Vec<Rc<RefCell<Entry>>>) {
        self.entries = entries;
    }

    pub fn add_entry(&mut self, entry: Rc<RefCell<Entry>>) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, entry: &Rc<RefCell<Entry>>) {
        if let Some(pos) = self.entries.iter().position(|x| Rc::ptr_eq(x, entry)) {
            self.entries.remove(pos);
        }
    }

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        let mut sum = EnergyQuantity::new(0f64, EnergyUnit::Kilocalorie);
        for entry in &self.entries {
            let calories = entry.borrow().get_calories()?;
            sum = sum + calories;
        }
        return Ok(sum);
    }

    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient_quantity(
        &self,
        nutrient: Rc<RefCell<Nutrient>>,
    ) -> Option<NutrientQuantity> {
        self.entries
            .iter()
            .map(|entry| entry.borrow().get_nutrient_quantity(nutrient.clone()))
            .sum()
    }
    pub fn get_flat_nutrient_quantity(&self, nutrient: Rc<RefCell<Nutrient>>) -> NutrientQuantity {
        self.entries
            .iter()
            .map(|entry| entry.borrow().get_flat_nutrient_quantity(nutrient.clone()))
            .sum()
    }
    pub fn contains_nutrient(&self, nutrient: Rc<RefCell<Nutrient>>) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.borrow().contains_nutrient(nutrient.clone()))
    }
}
