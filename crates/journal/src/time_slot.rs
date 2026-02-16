use chrono::NaiveDateTime;
use nutrients::{nutrient::Nutrient, nutrient_amount::NutrientAmount};
use units::energy::Energy;
use uuid::Uuid;

use crate::entry::Entry;

// (e.g. breakfast, lunch, snack 1)
#[derive(Clone, PartialEq)]
pub struct TimeSlot {
    id: Uuid,
    name: String,
    description: String,
    datetime_created: NaiveDateTime,
    datetime_last_modified: NaiveDateTime,
    entries: Vec<Entry>,
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

    pub fn get_entries(&mut self) -> Vec<Entry> {
        self.entries.clone()
    }

    pub fn set_entries(&mut self, entries: Vec<Entry>) {
        self.entries = entries;
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, entry: &Entry) {
        if let Some(pos) = self.entries.iter().position(|x| x == entry) {
            self.entries.remove(pos);
        }
    }

    pub fn get_calories(&self) -> Energy {
        self.entries
            .iter()
            .map(|entry| entry.get_calories())
            .sum()
    }

    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient_amount(&self, nutrient: Nutrient) -> Option<NutrientAmount> {
        self.entries
            .iter()
            .map(|entry| entry.get_nutrient_amount(nutrient.clone()))
            .sum()
    }
    pub fn get_flat_nutrient_amount(&self, nutrient: Nutrient) -> NutrientAmount {
        self.entries
            .iter()
            .map(|entry| entry.get_flat_nutrient_amount(nutrient.clone()))
            .sum()
    }
    pub fn contains_nutrient(&self, nutrient: Nutrient) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.contains_nutrient(nutrient.clone()))
    }
}
