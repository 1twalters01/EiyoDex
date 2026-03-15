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
        Some(EssentialityType::NonEssential),
        QuantityType::Macronutrient,
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carbohydrate_type: Carbohydrate::Fiber,
            },
        )),
    );
    assert_eq!(fiber.use_in_net_carbs(), false);

    let starch = NutrientType::new(
        Some(EssentialityType::NonEssential),
        QuantityType::Macronutrient,
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carbohydrate_type: Carbohydrate::Starch,
            },
        )),
    );
    assert_eq!(starch.use_in_net_carbs(), true);

    let sugar = NutrientType::new(
        Some(EssentialityType::NonEssential),
        QuantityType::Macronutrient,
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carbohydrate_type: Carbohydrate::Sugar,
            },
        )),
    );
    assert_eq!(sugar.use_in_net_carbs(), true);

    let sugar_alcohol = NutrientType::new(
        Some(EssentialityType::NonEssential),
        QuantityType::Macronutrient,
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
            CarbohydrateNutrient {
                carbohydrate_type: Carbohydrate::SugarAlcohol,
            },
        )),
    );
    assert_eq!(sugar_alcohol.use_in_net_carbs(), false);
}

#[test]
pub fn test_nutrients_is_macronutrient() {
    let essential_amino_acid = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::Macronutrient,
        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient {
            is_bcaa: false,
        })),
    );
    assert_eq!(essential_amino_acid.is_macronutrient(), true);

    let water = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::NonNutrient,
        ChemicalType::Water,
    );
    assert_eq!(water.is_macronutrient(), false);
}

#[test]
pub fn test_nutrients_is_micronutrient() {
    let vitamin = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::Micronutrient,
        ChemicalType::Vitamin,
    );
    assert_eq!(vitamin.is_micronutrient(), true);

    let mineral = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::Micronutrient,
        ChemicalType::Mineral,
    );
    assert_eq!(mineral.is_micronutrient(), true);

    let water = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::NonNutrient,
        ChemicalType::Water,
    );
    assert_eq!(water.is_micronutrient(), false);
}

#[test]
pub fn test_nutrients_is_conditionally_essential() {
    let vitamin = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::Micronutrient,
        ChemicalType::Vitamin,
    );
    assert_eq!(vitamin.is_conditionally_essential(), false);

    let conditionally_essential = NutrientType::new(
        Some(EssentialityType::ConditionallyEssential),
        QuantityType::NonNutrient,
        ChemicalType::Other,
    );
    assert_eq!(conditionally_essential.is_conditionally_essential(), true);
}

#[test]
pub fn test_nutrients_is_phytonutrient() {
    let phytonutrient = NutrientType::new(
        Some(EssentialityType::NonEssential),
        QuantityType::NonNutrient,
        ChemicalType::Phytonutrient,
    );
    assert_eq!(phytonutrient.is_phytonutrient(), true);

    let water = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::NonNutrient,
        ChemicalType::Water,
    );
    assert_eq!(water.is_phytonutrient(), false);
}

#[test]
pub fn test_nutrients_is_antinutrient() {
    let antinutrient = NutrientType::new(
        Some(EssentialityType::NonEssential),
        QuantityType::Micronutrient,
        ChemicalType::Antinutrient,
    );
    assert_eq!(antinutrient.is_antinutrient(), true);

    let water = NutrientType::new(
        Some(EssentialityType::Essential),
        QuantityType::NonNutrient,
        ChemicalType::Water,
    );
    assert_eq!(water.is_antinutrient(), false);
}
