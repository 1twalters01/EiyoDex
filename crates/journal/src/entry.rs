use std::{cell::RefCell, rc::Rc};

use chrono::NaiveDateTime;
use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

use crate::entry_item::EntryItem;

#[derive(Clone)]
pub struct Entry {
    id: Uuid,
    note: String,
    entry_item: Rc<RefCell<EntryItem>>,
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

    pub fn get_entry_item(&self) -> Rc<RefCell<EntryItem>> {
        self.entry_item.clone()
    }

    pub fn set_entry_item(&mut self, entry_item: Rc<RefCell<EntryItem>>) {
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

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        self.entry_item.borrow().get_calories()
    }

    pub fn get_protein(&self) {}
    pub fn get_carbs(&self) {}
    pub fn get_fats(&self) {}
    pub fn get_water(&self) {}

    pub fn get_nutrient_quantity(&self, nutrient: Rc<RefCell<Nutrient>>) -> Option<NutrientQuantity> {
        self.entry_item.borrow().get_nutrient_quantity(nutrient)
    }

    pub fn get_flat_nutrient_quantity(&self, nutrient: Rc<RefCell<Nutrient>>) -> NutrientQuantity {
        let nutrient_amount_option = self.entry_item.borrow().get_nutrient_quantity(nutrient.clone());
        match nutrient_amount_option {
            Some(nutrient_amount) => return nutrient_amount,
            None => {
                return NutrientQuantity::from_rc_refcell(0f64, nutrient.clone(), nutrient.borrow().get_main_unit())
                    .unwrap()
            }
        }
    }

    pub fn contains_nutrient(&self, nutrient: Rc<RefCell<Nutrient>>) -> bool {
        self.entry_item.borrow().contains_nutrient(nutrient)
    }
}
