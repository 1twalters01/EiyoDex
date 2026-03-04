use crate::{nutrient::Nutrient, records::nutrient_record::{NutrientLinkRecord, NutrientRecord}};

#[derive(Debug, Clone)]
pub struct NutrientList {
    nutrients: Vec<Nutrient>
}

impl NutrientList {
    pub fn new() -> Self {
        Self {
            nutrients: Vec::new(),
        }
    }

    pub fn from_vec(nutrients: Vec<Nutrient>) -> Self {
        Self {
            nutrients: nutrients
        }
    }

    pub fn get_nutrients(&self) -> Vec<Nutrient> {
        self.nutrients.clone()
    }

    pub fn set_nutrients(&mut self, nutrients: Vec<Nutrient>) {
        self.nutrients = nutrients
    }

    pub fn push(&mut self, nutrient: Nutrient) {
        self.nutrients.push(nutrient)
    }

    pub fn remove(&mut self, nutrient: &Nutrient) {
        self.nutrients.retain(|n| n != nutrient)
    }

    pub fn generate_nutrient_record_vec(&self) -> Vec<NutrientRecord> {
        self.nutrients.iter().map(|nutrient| NutrientRecord::from_nutrient(nutrient.clone())).collect()
    }

    pub fn generate_nutrient_link_record_vec(&self) -> Result<Vec<NutrientLinkRecord>, &'static str> {
        self.nutrients.iter().map(|nutrient| NutrientLinkRecord::from_nutrient(nutrient.clone())).collect()
    }
}
