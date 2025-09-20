use crate::schema::energy::Energy;

#[derive(PartialEq, Eq, Hash, Clone)]
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
