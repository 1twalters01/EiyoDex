#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Energy {
    Carbohydrate(Carbohydrate),
    Protein(Protein),
    Lipid(Lipid),
    Alcohol,
}

impl Energy {
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Self::Carbohydrate(carbohydrate) => carbohydrate.use_in_net_carbs(),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Carbohydrate {
    Fiber,
    Starch,
    Sugar,
    SugarAlcohol,
    AddedSugar,
}

impl Carbohydrate {
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Carbohydrate::Fiber | Carbohydrate::SugarAlcohol => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Protein {
    EssentialAminoAcid,
    ConditionallyEssentialAminoAcid,
    NonEssentialAminoAcid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lipid {
    Fats(Fat),
    TransFats(TransFat),
    Cholesterol,
    Phytosterol,
    Phospholipid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Fat {
    Monounsaturated,
    Polyunsaturated,
    Saturated,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TransFat {
    Natural,
    Artificial,
}
