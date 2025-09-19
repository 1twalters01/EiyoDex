pub enum Energy {
    Carbohydrates(Carbohydrates),
    Proteins(Proteins),
    Lipids(Lipids),
    Alcohol,
}

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

pub enum Proteins {
    EssentialAminoAcids,
    ConditionallyEssentialAminoAcids,
    NonEssentialAminoAcids,
}

pub enum Lipids {
    Fats(Fats),
    TransFats(TransFats),
    Cholesterol,
    Phytosterol,
    Phospholipids,
}

pub enum Fats {
    Monounsaturated,
    Polyunsaturated,
    Saturated,
}

pub enum TransFats {
    Natural,
    Artificial,
}
