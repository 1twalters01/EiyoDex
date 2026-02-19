use crate::schema::energy::EnergyYieldingNutrients;

pub enum ChemicalClass {
    EnergyYieldingNutrients(EnergyYieldingNutrients),
    Water,
    Vitamin,
    Mineral,
    Phytonutrient,
    Antinutrient,
    Other,
}

impl ChemicalClass {
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Self::EnergyYieldingNutrients(energy_yielding_nutrients) => energy_yielding_nutrients.use_in_net_carbs(),
            _ => false,
        }
    }
}

pub enum QuantityClass {
    Macronutrient,
    Micronutrient,
    NonNutrient,
}

pub enum EssentialityClass {
    Essential,
    ConditionallyEssential,
    NonEssential,
}

