use crate::schema::energy::EnergyYieldingNutrients;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum NutrientType {
    Energy(EnergyYieldingNutrients),
    Water,
    Vitamin,
    Mineral,
    OtherEssentialNutrient,
    ConditionallyEssentialNutrient,
    Phytonutrient,
    Antinutrient,
    Other,
}

impl NutrientType {
    pub fn is_nutrient_type(&self, nutrient_type: &NutrientType) -> bool {
        if self == nutrient_type {
            return true;
        } else {
            return false;
        }
    }
}
