use std::{borrow::Cow, thread::sleep, time::Duration};

use nutrients::{records::nutrient_type_record::NutrientTypeRecord, schema::{carbohydrate::{Carbohydrate, CarbohydrateNutrient}, energy::EnergyYieldingNutrients, lipid::{Fat, Lipid, LipidNutrient, Sterol, TransFat}, nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType, protein::ProteinNutrient}};
use utils::database::DatabaseService;

#[test]
fn test_from_nutrient_type() {
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::Essential), QuantityType::NonNutrient, ChemicalType::Water)
        ),
        NutrientTypeRecord::from_values(1, 3, 2, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::Essential), QuantityType::Micronutrient, ChemicalType::Vitamin)
        ),
        NutrientTypeRecord::from_values(1, 2, 3, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::ConditionallyEssential), QuantityType::Micronutrient, ChemicalType::Mineral)
        ),
        NutrientTypeRecord::from_values(2, 2, 4, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(Some(EssentialityType::NonEssential), QuantityType::Micronutrient, ChemicalType::Mineral)
        ),
        NutrientTypeRecord::from_values(3, 2, 4, None, None, None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(None, QuantityType::NonNutrient, ChemicalType::Phytonutrient)
        ),
        NutrientTypeRecord::from_values(4, 3, 5, None, None, None, None, None, None, None),
    );
   
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Fiber } ))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(1), None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Starch } ))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(2), None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Sugar } ))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(3), None, None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::SugarAlcohol } ))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(4), None, None, None, None, None),
    );

    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: true }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(2), None, Some(true), None, None, None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: false }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(2), None, Some(false), None, None, None, None),
    );

    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Cholesterol) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(1), Some(1), None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Phytosterol) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(1), Some(2), None, None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Monounsaturated) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(1), None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Polyunsaturated) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(2), None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Saturated) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(3), None),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Natural) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(3), None, None, Some(1)),
    );
    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Artificial) }))
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(3), None, None, Some(2)),
    );

    assert_eq!(
        NutrientTypeRecord::from_nutrient_type(
            NutrientType::new(
                Some(EssentialityType::Essential),
                QuantityType::Macronutrient,
                ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Alcohol)
            )
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(4), None, None, None, None, None, None),
    ); 
}

#[test]
fn test_to_nutrient_type() {
    assert_eq!(
        NutrientType::new(Some(EssentialityType::Essential), QuantityType::NonNutrient, ChemicalType::Water),
        NutrientTypeRecord::from_values(1, 3, 2, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(Some(EssentialityType::Essential), QuantityType::Micronutrient, ChemicalType::Vitamin),
        NutrientTypeRecord::from_values(1, 2, 3, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(Some(EssentialityType::ConditionallyEssential), QuantityType::Micronutrient, ChemicalType::Mineral),
        NutrientTypeRecord::from_values(2, 2, 4, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(Some(EssentialityType::NonEssential), QuantityType::Micronutrient, ChemicalType::Mineral),
        NutrientTypeRecord::from_values(3, 2, 4, None, None, None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(None, QuantityType::NonNutrient, ChemicalType::Phytonutrient),
        NutrientTypeRecord::from_values(4, 3, 5, None, None, None, None, None, None, None).to_nutrient_type(),
    );
   
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Fiber } ))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(1), None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Starch } ))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(2), None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Sugar } ))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(3), None, None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::SugarAlcohol } ))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(4), None, None, None, None, None).to_nutrient_type(),
    );

    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: true }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(2), None, Some(true), None, None, None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: false }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(2), None, Some(false), None, None, None, None).to_nutrient_type(),
    );

    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Cholesterol) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(1), Some(1), None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Sterols(Sterol::Phytosterol) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(1), Some(2), None, None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Monounsaturated) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(1), None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Polyunsaturated) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(2), None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Saturated) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(3), None).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Natural) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(3), None, None, Some(1)).to_nutrient_type(),
    );
    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::TransFats(TransFat::Artificial) }))
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(3), None, None, Some(2)).to_nutrient_type(),
    );

    assert_eq!(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Alcohol)
        ),
        NutrientTypeRecord::from_values(1, 1, 1, Some(4), None, None, None, None, None, None).to_nutrient_type(),
    ); 
}

#[tokio::test]
async fn test_database_operations_for_water() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let water_values_record = NutrientTypeRecord::from_values(1, 3, 2, None, None, None, None, None, None, None);
    let water_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(Some(EssentialityType::Essential), QuantityType::NonNutrient, ChemicalType::Water)
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = water_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());

    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(1, 3, 2, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&water_values_record));

    let delete_res = water_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_vitamin() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let vitamin_values_record = NutrientTypeRecord::from_values(1, 2, 3, None, None, None, None, None, None, None);
    let vitamin_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(Some(EssentialityType::Essential), QuantityType::Micronutrient, ChemicalType::Vitamin)
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = vitamin_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(1, 2, 3, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&vitamin_values_record));

    let delete_res = vitamin_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_mineral() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let mineral_values_record = NutrientTypeRecord::from_values(2, 2, 4, None, None, None, None, None, None, None);
    let mineral_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(Some(EssentialityType::ConditionallyEssential), QuantityType::Micronutrient, ChemicalType::Mineral)
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = mineral_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(2, 2, 4, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&mineral_values_record));

    let delete_res = mineral_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_phytonutrient() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let phytonutrient_values_record = NutrientTypeRecord::from_values(4, 3, 5, None, None, None, None, None, None, None);
    let phytonutrient_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(None, QuantityType::NonNutrient, ChemicalType::Phytonutrient)
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = phytonutrient_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(4, 3, 5, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&phytonutrient_values_record));

    let delete_res = phytonutrient_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_carbohydrate() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let starch_values_record = NutrientTypeRecord::from_values(1, 1, 1, Some(1), Some(2), None, None, None, None, None);
    let starch_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type: Carbohydrate::Starch } ))
        )
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = starch_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(1, 1, 1, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&starch_values_record));

    let delete_res = starch_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_protein() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let protein_values_record = NutrientTypeRecord::from_values(1, 1, 1, Some(2), None, Some(false), None, None, None, None);
    let protein_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: false }))
        ),
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = protein_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(1, 1, 1, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&protein_values_record));

    let delete_res = protein_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_lipid() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let polyunsaturated_fat_values_record = NutrientTypeRecord::from_values(1, 1, 1, Some(3), None, None, Some(2), None, Some(2), None);
    let polyunsaturated_fat_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(LipidNutrient { lipid_type: Lipid::Fats(Fat::Polyunsaturated) }))
        ),
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = polyunsaturated_fat_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(1, 1, 1, &pool).await;
    println!("{:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&polyunsaturated_fat_values_record));

    let delete_res = polyunsaturated_fat_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

#[tokio::test]
async fn test_database_operations_for_alcohol() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let alcohol_values_record = NutrientTypeRecord::from_values(1, 1, 1, Some(4), None, None, None, None, None, None);
    let alcohol_record = NutrientTypeRecord::from_nutrient_type(
        NutrientType::new(
            Some(EssentialityType::Essential),
            QuantityType::Macronutrient,
            ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Alcohol)
        )
    );

    let max_tries = 5;
    let mut tries = 0;
    let save_res = loop {
        let save_res = alcohol_record.save_to_database(&pool).await;

        match save_res {
            Ok(val) => break Ok(val),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some(Cow::Borrowed("5")) && tries < max_tries => {
                tries += 1;
                sleep(Duration::from_millis(50));
            }
            Err(e) => break Err(e),
        }
    };
    println!("save_res: {:#?}", save_res);
    assert!(save_res.is_ok());
        
    let response_record = NutrientTypeRecord::load_from_database_from_nutrient_type_ids(1, 1, 1, &pool).await;
    println!("loaded: {:#?}", response_record);
    assert!(response_record.is_ok());

    assert!(response_record.unwrap().contains(&alcohol_values_record));

    let delete_res = alcohol_record.delete_from_database_from_nutrient_type_id(&pool).await;

    assert!(delete_res.is_ok());
}

