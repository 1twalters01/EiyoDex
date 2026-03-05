use crate::nutrient::Nutrient;

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
}
