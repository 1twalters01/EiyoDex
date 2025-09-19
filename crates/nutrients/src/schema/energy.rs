#[derive(PartialEq, Eq, Hash)]
pub enum Energy {
    Carbohydrates(Carbohydrates),
    Proteins(Proteins),
    Lipids(Lipids),
    Alcohol,
}

#[derive(PartialEq, Eq, Hash)]
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
            _ => true
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Proteins {
    EssentialAminoAcids,
    ConditionallyEssentialAminoAcids,
    NonEssentialAminoAcids,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Lipids {
    Fats(Fats),
    TransFats(TransFats),
    Cholesterol,
    Phytosterol,
    Phospholipids,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Fats {
    Monounsaturated,
    Polyunsaturated,
    Saturated,
}

#[derive(PartialEq, Eq, Hash)]
pub enum TransFats {
    Natural,
    Artificial,
}
