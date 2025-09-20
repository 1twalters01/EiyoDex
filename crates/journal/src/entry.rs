use chrono::{NaiveDate, NaiveDateTime};
use exercise::exercise::ExerciseAmount;
use foods::food::FoodAmount;
use profiles::profile::Profile;
use uuid::Uuid;

pub struct JournalEntry {
    profile: Profile,
    note: String,
    date: NaiveDate,
    uncategorised_time_slot: Vec<Entry>,
    time_slots: Vec<TimeSlot>,
}

impl JournalEntry {
    pub fn get_calories(&self) {}
    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient(&self) {}
    pub fn get_nutrients(&self) {}
}

// (e.g. breakfast, lunch, snack 1)
pub struct TimeSlot {
    name: String,
    entries: Vec<Entry>,
}

impl TimeSlot {
    pub fn get_calories(&self) {}
    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient(&self) {}
    pub fn get_nutrients(&self) {}
}

pub struct Entry {
    id: Uuid,
    note: String,
    entry_item: EntryItem,
    datetime_eaten: NaiveDateTime,
    datetime_created: NaiveDateTime,
    datetime_last_modified: NaiveDateTime,
}

impl Entry {
    pub fn get_calories(&self) {}
    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient(&self) {}
    pub fn get_nutrients(&self) {}
}

pub enum EntryItem {
    FoodAmount(FoodAmount),
    ExerciseAmount(ExerciseAmount),
}
