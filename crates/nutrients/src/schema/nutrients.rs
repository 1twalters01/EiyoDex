pub enum NutrientTypes {
    Energy(Energy),
    Water(Water),
    Vitamins(Vitamins),
    Minerals(Minerals),
    EssentialNutrients(OtherEssentialNutrients),
    ConditionallyEssentialNutrients(ConditionallyEssentialNutrients),
    Phytonutrients(Phytonutrients),
    Antinutrients(Antinutrients),
}