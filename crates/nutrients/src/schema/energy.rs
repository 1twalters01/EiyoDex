#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Energy {
    Carbohydrates(Carbohydrates),
    Proteins(Proteins),
    Lipids(Lipids),
    Alcohol,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Carbohydrates {
    Fiber,
    Starch,
    Sugar,
    SugarAlcohols,
    AddedSugar,
}

impl Carbohydrates {
    pub fn use_in_net_calculation(&self) -> bool {
        match self {
            Carbohydrates::Fiber => false,
            Carbohydrates::SugarAlcohols => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Proteins {
    EssentialAminoAcids,
    ConditionallyEssentialAminoAcids,
    NonEssentialAminoAcids,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Lipids {
    Fats(Fats),
    TransFats(TransFats),
    Cholesterol,
    Phytosterol,
    Phospholipids,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Fats {
    Monounsaturated,
    Polyunsaturated,
    Saturated,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TransFats {
    Natural,
    Artificial,
}
