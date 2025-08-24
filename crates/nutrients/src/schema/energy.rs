pub enum Energy {
    Carbohydrates(Carbohydrates),
    Proteins(Proteins),
    Lipids(Lipids),
    Alcohol(Alcohol),
}

pub enum Carbohydrates {
    Fiber(Fiber),
    Starch(Starch),
    Sugar(Sugar),
    SugarAlcohols(SugarAlcohols),
    AddedSugar(AddedSugar),
}

impl Carbohydrates {
    pub fn use_in_net_calculation(&self) -> &'static str {
        match self {
            Fiber => false,
            SugarAlcohols => false,
            _ => true
        }
    }
}

pub enum Proteins {
    EssentialAminoAcids(EssentialAminoAcids),
    ConditionallyEssentialAminoAcids(ConditionallyEssentialAminoAcids),
    NonEssentialAminoAcids(NonEssentialAminoAcids),
}

pub enum Lipids {
    Fats(Fats),
    TransFats(TransFats),
    Cholesterol(Cholesterol),
    Phytosterol(Phytosterol),
    Phospholipids(Phospholipids),
}

pub enum Fats {
    Monounsaturated(Monounsaturated),
    Polyunsaturated(Polyunsaturated),
    Saturated(Saturated),
}

pub enum TransFats {
    Natural(Natural),
    Artificial(Artificial),
}