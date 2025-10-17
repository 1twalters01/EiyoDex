use nutrients::schema::{
    energy::{Carbohydrate, Energy, Protein},
    nutrients::NutrientType,
};

#[test]
pub fn test_nutrients_carbohydrate_net_calculation() {
    let fiber = NutrientType::Energy(Energy::Carbohydrate(Carbohydrate::Fiber));
    assert_eq!(fiber.use_in_net_carbs(), false);

    let starch = NutrientType::Energy(Energy::Carbohydrate(Carbohydrate::Starch));
    assert_eq!(starch.use_in_net_carbs(), true);

    let sugar = NutrientType::Energy(Energy::Carbohydrate(Carbohydrate::Sugar));
    assert_eq!(sugar.use_in_net_carbs(), true);

    let sugar_alcohol = NutrientType::Energy(Energy::Carbohydrate(Carbohydrate::SugarAlcohol));
    assert_eq!(sugar_alcohol.use_in_net_carbs(), false);
}

#[test]
pub fn test_nutrients_is_macronutrient() {
    let essential_amino_acid = NutrientType::Energy(Energy::Protein(Protein::EssentialAminoAcid));
    assert_eq!(essential_amino_acid.is_macronutrient(), true);

    let water = NutrientType::Water;
    assert_eq!(water.is_macronutrient(), false);
}

#[test]
pub fn test_nutrients_is_micronutrient() {
    let vitamin = NutrientType::Vitamin;
    assert_eq!(vitamin.is_micronutrient(), true);

    let mineral = NutrientType::Mineral;
    assert_eq!(mineral.is_micronutrient(), true);

    let water = NutrientType::Water;
    assert_eq!(water.is_micronutrient(), false);
}

#[test]
pub fn test_nutrients_is_conditionally_essential() {
    let vitamin = NutrientType::Vitamin;
    assert_eq!(vitamin.is_conditionally_essential(), false);

    let conditionally_essential = NutrientType::ConditionallyEssentialNutrient;
    assert_eq!(conditionally_essential.is_conditionally_essential(), true);
}

#[test]
pub fn test_nutrients_is_phytonutrient() {
    let phytonutrient = NutrientType::Phytonutrient;
    assert_eq!(phytonutrient.is_phytonutrient(), true);

    let water = NutrientType::Water;
    assert_eq!(water.is_phytonutrient(), false);
}

#[test]
pub fn test_nutrients_is_antinutrient() {
    let antinutrient = NutrientType::Antinutrient;
    assert_eq!(antinutrient.is_antinutrient(), true);

    let water = NutrientType::Water;
    assert_eq!(water.is_antinutrient(), false);
}
