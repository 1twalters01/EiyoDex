use nutrients::{records::nutrient_type_record::NutrientTypeRecord, schema::{carbohydrate::{Carbohydrate, CarbohydrateNutrient}, energy::EnergyYieldingNutrients, lipid::{Fat, Lipid, LipidNutrient, Sterol, TransFat}, nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType, protein::ProteinNutrient}};

#[test]
fn test_from_nutrient_type() {
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::Essential), QuantityType::NonNutrient, ChemicalType::Water)
        ),
        NutrientTypeRecord::from_values(Some(1), 3, 2, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::Essential), QuantityType::Micronutrient, ChemicalType::Vitamin)
        ),
        NutrientTypeRecord::from_values(Some(1), 2, 3, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::ConditionallyEssential), QuantityType::Micronutrient, ChemicalType::Mineral)
        ),
        NutrientTypeRecord::from_values(Some(2), 2, 4, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::NonEssential), QuantityType::Micronutrient, ChemicalType::Mineral)
        ),
        NutrientTypeRecord::from_values(Some(3), 2, 4, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(None, QuantityType::NonNutrient, ChemicalType::Phytonutrient)
        ),
        NutrientTypeRecord::from_values(None, 3, 5, None, None, None, None, None, None, None),
    );
   
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Fiber } ))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(1), None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Starch } ))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(2), None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Sugar } ))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(3), None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::SugarAlcohol } ))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(4), None, None, None, None, None),
    );

    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: true }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(2), None, Some(true), None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: false }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(2), None, Some(false), None, None, None, None),
    );

    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Cholesterol) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(1), Some(1), None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Phytosterol) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(1), Some(2), None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Monounsaturated) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(2), None, Some(1), None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Polyunsaturated) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(2), None, Some(2), None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Saturated) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(2), None, Some(3), None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Natural) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(3), None, None, Some(1)),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Artificial) }))
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(3), None, None, Some(2)),
    );

    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Alcohol)
            )
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(4), None, None, None, None, None, None),
    ); 
}

#[test]
fn test_to_nutrient_type() {
    assert_eq!(
        NutrientType::new(Some(EssentialityType::Essential), QuantityType::NonNutrient, ChemicalType::Water),
        NutrientTypeRecord::from_values(Some(1), 3, 2, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(Some(EssentialityType::Essential), QuantityType::Micronutrient, ChemicalType::Vitamin),
        NutrientTypeRecord::from_values(Some(1), 2, 3, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(Some(EssentialityType::ConditionallyEssential), QuantityType::Micronutrient, ChemicalType::Mineral),
        NutrientTypeRecord::from_values(Some(2), 2, 4, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(Some(EssentialityType::NonEssential), QuantityType::Micronutrient, ChemicalType::Mineral),
        NutrientTypeRecord::from_values(Some(3), 2, 4, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(None, QuantityType::NonNutrient, ChemicalType::Phytonutrient),
        NutrientTypeRecord::from_values(None, 3, 5, None, None, None, None, None, None, None).to_nutrient_type(),
    );
   
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Fiber } ))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(1), None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Starch } ))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(2), None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Sugar } ))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(3), None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::SugarAlcohol } ))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(1), Some(4), None, None, None, None, None).to_nutrient_type(),
    );

    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: true }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(2), None, Some(true), None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: false }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(2), None, Some(false), None, None, None, None).to_nutrient_type(),
    );

    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Cholesterol) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(1), Some(1), None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Phytosterol) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(1), Some(2), None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Monounsaturated) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(2), None, Some(1), None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Polyunsaturated) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(2), None, Some(2), None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Saturated) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(2), None, Some(3), None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Natural) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(3), None, None, Some(1)).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Artificial) }))
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(3), None, None, Some(3), None, None, Some(2)).to_nutrient_type(),
    );

    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Alcohol)
        ),
        NutrientTypeRecord::from_values(Some(1), 1, 1, Some(4), None, None, None, None, None, None).to_nutrient_type(),
    ); 
}

#[test]
fn test_save_to_database() {
}

#[test]
fn test_load_from_database() {
}

#[test]
fn test_delete_from_database() {
}

