use nutrients::{nutrient_units::NutrientUnit, records::nutrient_unit_record::NutrientUnitRecord};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient_unit() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::Mass(MassUnit::Gram), &pool).await,
        NutrientUnitRecord::from_values(1, Some(MassUnit::Gram.get_database_id(&pool).await.unwrap()), None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::Volume(VolumeUnit::Liter), &pool).await,
        NutrientUnitRecord::from_values(2, None, Some(VolumeUnit::Liter.get_database_id(&pool).await.unwrap()), None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::Energy(EnergyUnit::Kilocalorie), &pool).await,
        NutrientUnitRecord::from_values(3, None, None, Some(EnergyUnit::Kilocalorie.get_database_id(&pool).await.unwrap())).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::Energy(EnergyUnit::Kilojoule), &pool).await,
        NutrientUnitRecord::from_values(3, None, None, Some(EnergyUnit::Kilojoule.get_database_id(&pool).await.unwrap())).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::IU, &pool).await,
        NutrientUnitRecord::from_values(4, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::DFE, &pool).await,
        NutrientUnitRecord::from_values(5, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::NE, &pool).await,
        NutrientUnitRecord::from_values(6, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::RAE, &pool).await,
        NutrientUnitRecord::from_values(7, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::PDCAAS, &pool).await,
        NutrientUnitRecord::from_values(8, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::DIAAS1, &pool).await,
        NutrientUnitRecord::from_values(9, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::DIAAS2, &pool).await,
        NutrientUnitRecord::from_values(10, None, None, None).await,
    );

    assert_eq!(
        NutrientUnitRecord::from_nutrient_unit(NutrientUnit::DIAAS3, &pool).await,
        NutrientUnitRecord::from_values(11, None, None, None).await,
    );
}

#[tokio::test]
async fn test_to_nutrient_unit() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;

    assert_eq!(
        NutrientUnit::Mass(MassUnit::Gram),
        NutrientUnitRecord::from_values(1, Some(MassUnit::Gram.get_database_id(&pool).await.unwrap()), None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::Volume(VolumeUnit::Liter),
        NutrientUnitRecord::from_values(2, None, Some(VolumeUnit::Liter.get_database_id(&pool).await.unwrap()), None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::Energy(EnergyUnit::Kilocalorie),
        NutrientUnitRecord::from_values(3, None, None, Some(EnergyUnit::Kilocalorie.get_database_id(&pool).await.unwrap())).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::Energy(EnergyUnit::Kilojoule),
        NutrientUnitRecord::from_values(3, None, None, Some(EnergyUnit::Kilojoule.get_database_id(&pool).await.unwrap())).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::IU,
        NutrientUnitRecord::from_values(4, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::DFE,
        NutrientUnitRecord::from_values(5, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::NE,
        NutrientUnitRecord::from_values(6, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::RAE,
        NutrientUnitRecord::from_values(7, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::PDCAAS,
        NutrientUnitRecord::from_values(8, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::DIAAS1,
        NutrientUnitRecord::from_values(9, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::DIAAS2,
        NutrientUnitRecord::from_values(10, None, None, None).await.to_nutrient_unit(&pool).await,
    );

    assert_eq!(
        NutrientUnit::DIAAS3,
        NutrientUnitRecord::from_values(11, None, None, None).await.to_nutrient_unit(&pool).await,
    );
}

#[tokio::test]
async fn test_load_from_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let unit_name = String::from("bv"); // biological value
    let res_1 = NutrientUnitRecord::add_unit_type(unit_name.clone(), &pool).await;
    assert!(res_1.is_ok());
    let unit_id = NutrientUnitRecord::get_unit_type_database_id(unit_name.clone(), &pool).await.unwrap();
    assert_eq!(unit_name, NutrientUnitRecord::load_unit_type_from_database(unit_id, &pool).await.unwrap());

    let nutrient_record = NutrientUnitRecord::from_values(unit_id, None, None, None).await;
    let res_2 = nutrient_record.save_to_database(&pool).await;
    println!("res: {:#?}", res_2);
    assert!(res_2.is_ok());
    let nutrient_record_id = nutrient_record.get_database_id(&pool).await.unwrap();
    println!("nutrient_record: {:#?}", nutrient_record);
    println!("{:#?}",NutrientUnitRecord::load_from_database(nutrient_record_id, &pool).await.unwrap()); 
    assert_eq!(nutrient_record, NutrientUnitRecord::load_from_database(nutrient_record_id, &pool).await.unwrap());


    assert!(NutrientUnitRecord::delete_unit_type_from_database(unit_name.clone(), &pool).await.is_err());

    let res_3 = NutrientUnitRecord::delete_unit_type_from_database(unit_name.clone(), &pool).await;
    assert!(res_3.is_err());

    let res_4 = nutrient_record.delete_from_database(&pool).await;
    assert!(res_4.is_ok());
    let res_5 = NutrientUnitRecord::delete_unit_type_from_database(unit_name, &pool).await;
    assert!(res_5.is_ok());
}

