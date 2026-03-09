use crate::nutrient_ratio::NutrientRatio;

pub struct NutrientBalance {
    name: String,
    description: String,
    nutrient_ratio: NutrientRatio,
    // reference range upper and lower, make it a vec?
}

impl NutrientBalance {
    pub fn new(name: String, description: String, nutrient_ratio: NutrientRatio) -> Self {
        Self {
            name,
            description,
            nutrient_ratio,
        }
    }
}
