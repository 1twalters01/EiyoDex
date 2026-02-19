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
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Self::Energy(energy) => energy.use_in_net_carbs(),
            _ => false,
        }
    }

    pub fn is_macronutrient(&self) -> bool {
        match self {
            Self::Energy(_) => true,
            _ => false,
        }
    }

    pub fn is_micronutrient(&self) -> bool {
        match self {
            Self::Vitamin | Self::Mineral => true,
            _ => false,
        }
    }

    pub fn is_nutrient_type(&self, nutrient_type: &NutrientType) -> bool {
        if self == nutrient_type {
            return true;
        } else {
            return false;
        }
    }

    pub fn is_conditionally_essential(&self) -> bool {
        match self {
            Self::ConditionallyEssentialNutrient => true,
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
