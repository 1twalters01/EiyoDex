use crate::schema::energy::Energy;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum NutrientType {
    Energy(Energy),
    Water,
    Vitamins,
    Minerals,
    OtherEssentialNutrients,
    ConditionallyEssentialNutrients,
    Phytonutrients,
    Antinutrients,
    Other,
}
