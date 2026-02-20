use nutrients::schema::{
    carbohydrate::{Carbohydrate, CarbohydrateNutrient},
    energy::EnergyYieldingNutrients,
    nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
    nutrient_type::NutrientType,
    protein::ProteinNutrient,
};

#[test]
pub fn test_nutrients_carbohydrate_net_calculation() {
    let fiber = NutrientType::new(
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carb_type: Carbohydrate::Fiber,
                is_added: false,
                glycemic_index: None,
            },
        )),
        QuantityType::Macronutrient,
        Some(EssentialityType::NonEssential),
    );
    assert_eq!(fiber.use_in_net_carbs(), false);

    let starch = NutrientType::new(
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carb_type: Carbohydrate::Starch,
                is_added: false,
                glycemic_index: None,
            },
        )),
        QuantityType::Macronutrient,
        Some(EssentialityType::NonEssential),
    );
    assert_eq!(starch.use_in_net_carbs(), true);

    let sugar = NutrientType::new(
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carb_type: Carbohydrate::Sugar,
                is_added: true,
                glycemic_index: None,
            },
        )),
        QuantityType::Macronutrient,
        Some(EssentialityType::NonEssential),
    );
    assert_eq!(sugar.use_in_net_carbs(), true);

    let sugar_alcohol = NutrientType::new(
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carb_type: Carbohydrate::SugarAlcohol,
                is_added: false,
                glycemic_index: None,
            },
        )),
        QuantityType::Macronutrient,
        Some(EssentialityType::NonEssential),
    );
    assert_eq!(sugar_alcohol.use_in_net_carbs(), false);
}

#[test]
pub fn test_nutrients_is_macronutrient() {
    let essential_amino_acid = NutrientType::new(
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient {
            is_bcaa: false,
        })),
        QuantityType::Macronutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(essential_amino_acid.is_macronutrient(), true);

    let water = NutrientType::new(
        ChemicalType::Water,
        QuantityType::NonNutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(water.is_macronutrient(), false);
}

#[test]
pub fn test_nutrients_is_micronutrient() {
    let vitamin = NutrientType::new(
        ChemicalType::Vitamin,
        QuantityType::Micronutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(vitamin.is_micronutrient(), true);

    let mineral = NutrientType::new(
        ChemicalType::Mineral,
        QuantityType::Micronutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(mineral.is_micronutrient(), true);

    let water = NutrientType::new(
        ChemicalType::Water,
        QuantityType::NonNutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(water.is_micronutrient(), false);
}

#[test]
pub fn test_nutrients_is_conditionally_essential() {
    let vitamin = NutrientType::new(
        ChemicalType::Vitamin,
        QuantityType::Micronutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(vitamin.is_conditionally_essential(), false);

    let conditionally_essential = NutrientType::new(
        ChemicalType::Other,
        QuantityType::NonNutrient,
        Some(EssentialityType::ConditionallyEssential),
    );
    assert_eq!(conditionally_essential.is_conditionally_essential(), true);
}

#[test]
pub fn test_nutrients_is_phytonutrient() {
    let phytonutrient = NutrientType::new(
        ChemicalType::Phytonutrient,
        QuantityType::NonNutrient,
        Some(EssentialityType::NonEssential),
    );
    assert_eq!(phytonutrient.is_phytonutrient(), true);

    let water = NutrientType::new(
        ChemicalType::Water,
        QuantityType::NonNutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(water.is_phytonutrient(), false);
}

#[test]
pub fn test_nutrients_is_antinutrient() {
    let antinutrient = NutrientType::new(
        ChemicalType::Antinutrient,
        QuantityType::Micronutrient,
        Some(EssentialityType::NonEssential),
    );
    assert_eq!(antinutrient.is_antinutrient(), true);

    let water = NutrientType::new(
        ChemicalType::Water,
        QuantityType::NonNutrient,
        Some(EssentialityType::Essential),
    );
    assert_eq!(water.is_antinutrient(), false);
}
