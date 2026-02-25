use crate::schema::energy::EnergyYieldingNutrients;
use sqlx::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum ChemicalType {
    EnergyYieldingNutrients(EnergyYieldingNutrients),
    Water,
    Vitamin,
    Mineral,
    Phytonutrient,
    Antinutrient,
    Other,
}

impl ChemicalType {
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Self::EnergyYieldingNutrients(energy_yielding_nutrients) => {
                energy_yielding_nutrients.use_in_net_carbs()
            }
            _ => false,
        }
    }

    pub fn is_chemical_type(&self, chemical_type: &ChemicalType) -> bool {
        if self == chemical_type {
            true
        } else {
            false
        }
    }

    pub fn is_energy_yielding_nutrient(&self) -> bool {
        match self {
            Self::EnergyYieldingNutrients(_) => true,
            _ => false,
        }
    }

    pub fn is_water(&self) -> bool {
        match self {
            Self::Water => true,
            _ => false,
        }
    }

    pub fn is_vitamin(&self) -> bool {
        match self {
            Self::Vitamin => true,
            _ => false,
        }
    }

    pub fn is_mineral(&self) -> bool {
        match self {
            Self::Mineral => true,
            _ => false,
        }
    }

    pub fn is_phytonutrient(&self) -> bool {
        match self {
            Self::Phytonutrient => true,
            _ => false,
        }
    }

    pub fn is_antinutrient(&self) -> bool {
        match self {
            Self::Antinutrient => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuantityType {
    Macronutrient,
    Micronutrient,
    NonNutrient,
}

impl QuantityType {
    pub fn is_quantity_type(&self, quantity_type: &QuantityType) -> bool {
        if self == quantity_type {
            true
        } else {
            false
        }
    }

    pub fn is_macronutrient(&self) -> bool {
        match self {
            Self::Macronutrient => true,
            _ => false,
        }
    }

    pub fn is_micronutrient(&self) -> bool {
        match self {
            Self::Micronutrient => true,
            _ => false,
        }
    }

    pub fn is_non_nutrient(&self) -> bool {
        match self {
            Self::NonNutrient => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum EssentialityType {
    Essential,
    ConditionallyEssential,
    NonEssential,
}

impl EssentialityType {
    pub fn is_essentiality_type(&self, essentiality_type: &EssentialityType) -> bool {
        if self == essentiality_type {
            true
        } else {
            false
        }
    }

    pub fn is_essential(&self) -> bool {
        match self {
            Self::Essential => true,
            _ => false,
        }
    }

    pub fn is_conditionally_essential(&self) -> bool {
        match self {
            Self::ConditionallyEssential => true,
            _ => false,
        }
    }

    pub fn is_nonessential(&self) -> bool {
        match self {
            Self::NonEssential => true,
            _ => false,
        }
    }
}
