#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct LipidNutrient {
    pub lipid_type: Lipid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lipid {
    Fats(Fat),
    TransFats(TransFat),
    Sterols(Sterols),
    Phospholipid,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Sterols {
    Cholesterol,
    Phytosterol,
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
