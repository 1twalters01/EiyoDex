use crate::schema::nutrient_classes::{ChemicalClass, EssentialityClass, QuantityClass};

pub struct NutrientType {
    pub chemical_class: ChemicalClass,
    pub quantity_class: QuantityClass,
    pub essentiality_class: Option<EssentialityClass>,
}

impl NutrientType {
    pub fn use_in_net_carbs(&self) -> bool {
        self.chemical_class.use_in_net_carbs()
    }
}
