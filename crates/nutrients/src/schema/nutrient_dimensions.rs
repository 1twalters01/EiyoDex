use crate::schema::energy::EnergyYieldingNutrients;

pub struct NutrientType {
    pub chemical_class: ChemicalClass,
    pub quantity_class: QuantityClass,
    pub essentiality: Option<Essentiality>,
}

pub enum ChemicalClass {
    EnergyYieldingNutrients(EnergyYieldingNutrients),
    Water,
    Vitamin,
    Mineral,
    Phytonutrient,
    Antinutrient,
    Other,
}

pub enum QuantityClass {
    Macronutrient,
    Micronutrient,
    NonNutrient,
}

pub enum Essentiality {
    Essential,
    ConditionallyEssential,
    NonEssential,
}

