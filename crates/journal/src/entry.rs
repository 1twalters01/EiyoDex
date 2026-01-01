use chrono::{NaiveDate, NaiveDateTime};
use exercise::exercise::ExerciseAmount;
use foods::food_amount::FoodAmount;
use profiles::profile::Profile;
use units::energy::Energy;
use uuid::Uuid;

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

    pub fn get_calories(&self) -> Energy {
        let uncategorised_calories: Energy = self
            .uncategorised_time_slots
            .iter()
            .map(|entry| entry.get_calories())
            .sum();
        let time_slot_calories: Energy = self
            .time_slots
            .iter()
            .map(|time_slot| time_slot.get_calories())
            .sum();
        let total_calories: Energy = uncategorised_calories + time_slot_calories;
        return total_calories;
    }
    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient(&self) {}
    pub fn get_nutrients(&self) {}
}

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
            .map(|time_slot| time_slot.get_calories())
            .sum()
    }

    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient(&self) {}
    pub fn get_nutrients(&self) {}
}

#[derive(Clone, PartialEq)]
pub struct Entry {
    id: Uuid,
    note: String,
    entry_item: EntryItem,
    datetime_eaten: Option<NaiveDateTime>,
    datetime_created: NaiveDateTime,
    datetime_last_modified: NaiveDateTime,
}

impl Entry {
    pub fn new() {}

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_note(&self) -> String {
        self.note.clone()
    }

    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }

    pub fn get_entry_item(&self) -> EntryItem {
        self.entry_item.clone()
    }

    pub fn set_entry_item(&mut self, entry_item: EntryItem) {
        self.entry_item = entry_item;
    }

    pub fn get_datetime_eaten(&self) -> Option<NaiveDateTime> {
        self.datetime_eaten
    }

    pub fn set_datetime_eaten(&mut self, datetime: Option<NaiveDateTime>) {
        self.datetime_eaten = datetime;
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

    pub fn get_calories(&self) -> Energy {
        self.entry_item.get_calories()
    }

    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}
    pub fn get_nutrient(&self) {}
    pub fn get_nutrients(&self) {}
}

#[derive(Clone, PartialEq)]
pub enum EntryItem {
    FoodAmount(FoodAmount, Uuid),
    ExerciseAmount(ExerciseAmount),
}

impl EntryItem {
    pub fn get_calories(&self) -> Energy {
        match self {
            Self::FoodAmount(food_amount, food_data_uuid) => {
                food_amount.get_calories(food_data_uuid.clone())
            }
            Self::ExerciseAmount(exercise_amount) => exercise_amount.get_calories(),
        }
    }
}
