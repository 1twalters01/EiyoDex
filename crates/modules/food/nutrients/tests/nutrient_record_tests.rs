use nutrients::{entity::Entity, nutrient::Nutrient, nutrient_units::NutrientUnit, records::{nutrient_record::NutrientRecord, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord }, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::mass::unit::MassUnit;
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;

    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let main_unit_record = NutrientUnitRecord::from_nutrient_unit(main_unit, &pool).await;
    let unit_res = main_unit_record.save_to_database(&pool).await;
    assert!(unit_res.is_ok());

    let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient_type.clone());
    let type_res = nutrient_type_record.save_to_database(&pool).await;
    assert!(type_res.is_ok());

    let main_unit_id = main_unit_record.get_database_id(&pool).await.unwrap();
    let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
    let quantity_type_id = nutrient_type_record.get_quantity_type_id();
    let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();

    let mut nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());

    let nutrient_entity = Entity::new(nutrient.borrow().clone());
    let nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let manual_record = NutrientRecord::from_values(
        nutrient_entity.get_id().to_bytes().to_vec(),
        name,
        description,
        main_unit_id,
        essentiality_type_id,
        quantity_type_id,
        chemical_id
    );

    assert_eq!(nutrient_record, manual_record);
}

#[tokio::test]
async fn test_to_nutrient() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;

    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let main_unit_record = NutrientUnitRecord::from_nutrient_unit(main_unit, &pool).await;
    let unit_res = main_unit_record.save_to_database(&pool).await;
    assert!(unit_res.is_ok());

    let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient_type.clone());
    let type_res = nutrient_type_record.save_to_database(&pool).await;
    assert!(type_res.is_ok());

    let main_unit_id = main_unit_record.get_database_id(&pool).await.unwrap();
    let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
    let quantity_type_id = nutrient_type_record.get_quantity_type_id();
    let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();

    let mut nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let nutrient_entity = Entity::new(nutrient.borrow().clone());
    
    let manual_record = NutrientRecord::from_values(
        nutrient_entity.get_id().to_bytes().to_vec(),
        name,
        description,
        main_unit_id,
        essentiality_type_id,
        quantity_type_id,
        chemical_id
    );

    let manual_entity = manual_record.to_nutrient_entity(&pool).await;
    assert_eq!(nutrient_entity, manual_entity);

    let manual_nutrient = manual_record.to_nutrient(&pool).await;
    assert_eq!(nutrient.borrow().clone(), manual_nutrient);
}

#[tokio::test]
async fn test_database_operations() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;

    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let main_unit_record = NutrientUnitRecord::from_nutrient_unit(main_unit, &pool).await;
    let unit_res = main_unit_record.save_to_database(&pool).await;
    assert!(unit_res.is_ok());

    let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient_type.clone());
    let type_res = nutrient_type_record.save_to_database(&pool).await;
    assert!(type_res.is_ok());

    let main_unit_id = main_unit_record.get_database_id(&pool).await.unwrap();
    let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
    let quantity_type_id = nutrient_type_record.get_quantity_type_id();
    let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();

    let mut nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let nutrient_entity = Entity::new(nutrient.borrow().clone());
    let nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();

    let save_res = nutrient_record.save_to_database(&pool).await;
    assert!(save_res.is_ok());
    
    let id = nutrient_entity.get_id();
    let load_res = NutrientRecord::load_from_database_using_id(id, &pool).await;
    assert!(load_res.is_ok());

    let loaded_nutrient_record = load_res.unwrap();
    assert_eq!(loaded_nutrient_record, nutrient_record);

    let res = nutrient_record.delete_nutrient_from_database(&pool).await;
}

