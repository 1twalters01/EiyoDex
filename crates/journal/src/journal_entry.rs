use chrono::NaiveDate;
use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use profiles::profile::Profile;
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

use crate::{entry::Entry, time_slot::TimeSlot};

pub struct JournalEntry {
    id: Uuid,
    profile: Profile,
    note: String,
    date: NaiveDate,
    uncategorised_time_slots: Vec<Entry>,
    time_slots: Vec<TimeSlot>,
}

impl JournalEntry {
    pub fn new() {}

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_profile(&self) -> Profile {
        self.profile.clone()
    }

    pub fn set_profile(&mut self, profile: Profile) {
        self.profile = profile;
    }

    pub fn get_note(&self) -> String {
        self.note.clone()
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }

    pub fn get_date(&self) -> NaiveDate {
        self.date
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }

    pub fn get_uncategorised_time_slots(&self) -> Vec<Entry> {
        self.uncategorised_time_slots.clone()
    }

    pub fn set_uncategorised_time_slots(&mut self, entries: Vec<Entry>) {
        self.uncategorised_time_slots = entries;
    }

    pub fn add_uncategorised_time_slot(&mut self, entry: Entry) {
        self.uncategorised_time_slots.push(entry);
    }

    pub fn remove_uncategorised_time_slot(&mut self, entry: Entry) {
        if let Some(pos) = self
            .uncategorised_time_slots
            .iter()
            .position(|x| x == &entry)
        {
            self.uncategorised_time_slots.remove(pos);
        }
    }

    pub fn get_time_slots(&self) -> Vec<TimeSlot> {
        self.time_slots.clone()
    }

    pub fn set_time_slots(&mut self, time_slots: Vec<TimeSlot>) {
        self.time_slots = time_slots;
    }

    pub fn add_time_slot(&mut self, time_slot: TimeSlot) {
        self.time_slots.push(time_slot);
    }

    pub fn remove_time_slot(&mut self, time_slot: TimeSlot) {
        if let Some(pos) = self.time_slots.iter().position(|x| x == &time_slot) {
            self.time_slots.remove(pos);
        }
    }

    pub fn get_calories(&self) -> EnergyQuantity {
        let uncategorised_calories: EnergyQuantity = self
            .uncategorised_time_slots
            .iter()
            .map(|entry| entry.get_calories())
            .sum();
        let time_slot_calories: EnergyQuantity = self
            .time_slots
            .iter()
            .map(|time_slot| time_slot.get_calories())
            .sum();

        let total_calories: EnergyQuantity = uncategorised_calories + time_slot_calories;
        return total_calories;
    }

    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient_amount(&self, nutrient: Nutrient) -> NutrientQuantity {
        let uncategorised_nutrient_amount: NutrientQuantity = self
            .uncategorised_time_slots
            .iter()
            .map(|entry| entry.get_flat_nutrient_amount(nutrient.clone()))
            .sum();
        let time_slot_nutrient_amount: NutrientQuantity = self
            .time_slots
            .iter()
            .map(|time_slot| time_slot.get_flat_nutrient_amount(nutrient.clone()))
            .sum();

        let total_nutrient_amount: NutrientQuantity =
            uncategorised_nutrient_amount + time_slot_nutrient_amount;
        return total_nutrient_amount;
    }

    pub fn contains_nutrient(&self, nutrient: Nutrient) -> bool {
        let uncategorised_nutrient_res: bool = self
            .uncategorised_time_slots
            .iter()
            .any(|entry| entry.contains_nutrient(nutrient.clone()));

        let time_slot_nutrient_res: bool = self
            .time_slots
            .iter()
            .any(|time_slot| time_slot.contains_nutrient(nutrient.clone()));

        let contains_nutrient = uncategorised_nutrient_res || time_slot_nutrient_res;
        return contains_nutrient;
    }
}
