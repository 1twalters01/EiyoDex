use crate::schema::nutrient_classes::{ChemicalType, EssentialityType, QuantityType};

#[derive(Clone, Debug, PartialEq)]
pub struct NutrientType {
    pub chemical_type: ChemicalType,
    pub quantity_type: QuantityType,
    pub essentiality_type: Option<EssentialityType>,
}

impl NutrientType {
    pub fn new(
        chemical_type: ChemicalType,
        quantity_type: QuantityType,
        essentiality_type: Option<EssentialityType>,
    ) -> Self {
        Self {
            chemical_type,
            quantity_type,
            essentiality_type,
        }
    }

    pub fn is_nutrient_type(&self, nutrient_type: &NutrientType) -> bool {
        if self == nutrient_type {
            true
        } else {
            false
        }
    }

    // Chemical type
    pub fn use_in_net_carbs(&self) -> bool {
        self.chemical_type.use_in_net_carbs()
    }

    pub fn is_chemical_type(&self, chemical_type: &ChemicalType) -> bool {
        self.chemical_type.is_chemical_type(chemical_type)
    }

    pub fn is_energy_yielding_nutrient(&self) -> bool {
        self.chemical_type.is_energy_yielding_nutrient()
    }

    pub fn is_water(&self) -> bool {
        self.chemical_type.is_water()
    }

    pub fn is_vitamin(&self) -> bool {
        self.chemical_type.is_vitamin()
    }

    pub fn is_mineral(&self) -> bool {
        self.chemical_type.is_mineral()
    }

    pub fn is_phytonutrient(&self) -> bool {
        self.chemical_type.is_phytonutrient()
    }

    pub fn is_antinutrient(&self) -> bool {
        self.chemical_type.is_antinutrient()
    }

    // Quantity type
    pub fn is_quantity_type(&self, quantity_type: &QuantityType) -> bool {
        self.quantity_type.is_quantity_type(quantity_type)
    }

    pub fn is_macronutrient(&self) -> bool {
        self.quantity_type.is_macronutrient()
    }

    pub fn is_micronutrient(&self) -> bool {
        self.quantity_type.is_micronutrient()
    }

    pub fn is_non_nutrient(&self) -> bool {
        self.quantity_type.is_non_nutrient()
    }

    // Essentiality type
    pub fn is_essential(&self) -> bool {
        match &self.essentiality_type {
            Some(essentiality_type) => essentiality_type.is_essential(),
            None => false,
        }
    }

    pub fn is_conditionally_essential(&self) -> bool {
        match &self.essentiality_type {
            Some(essentiality_type) => essentiality_type.is_conditionally_essential(),
            None => false,
        }
    }

    pub fn is_nonessential(&self) -> bool {
        match &self.essentiality_type {
            Some(essentiality_type) => essentiality_type.is_nonessential(),
            None => false,
        }
    }
}
