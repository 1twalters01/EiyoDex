use crate::schema::{
    carbohydrate::CarbohydrateNutrient, lipid::LipidNutrient, protein::ProteinNutrient,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EnergyYieldingNutrients {
    Carbohydrate(CarbohydrateNutrient),
    Protein(ProteinNutrient),
    Lipid(LipidNutrient),
    Alcohol,
}

impl EnergyYieldingNutrients {
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Self::Carbohydrate(carbohydrate_nutrient) => carbohydrate_nutrient.use_in_net_carbs(),
            _ => false,
        }
    }
}
