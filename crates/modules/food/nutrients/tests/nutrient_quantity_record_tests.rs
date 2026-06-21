use identity::{entity::Entity, inner_id::InnerIdType, Id};
use nutrients::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity, nutrient_units::NutrientUnit, records::{nutrient_quantity_record::NutrientQuantityRecord, nutrient_record::NutrientRecord, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::mass::unit::MassUnit;
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient_quantity() {
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

    let nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let mut nutrient_entity = Entity::new(nutrient.borrow().clone());

    let mut nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let save_res = nutrient_record.save_to_database(&pool).await;
    let new_entity_id = save_res.unwrap();
    nutrient_entity.set_id(Id::from_bytes(InnerIdType::Uuid, new_entity_id.clone().try_into().unwrap()));

    let value = 5.5;
    let output_unit = NutrientUnit::Mass(MassUnit::Kilogram);
    let nutrient_quantity = NutrientQuantity::from_rc_refcell(value, nutrient, output_unit).unwrap();
    let quantity_record = NutrientQuantityRecord::from_nutrient_quantity(nutrient_quantity, &pool).await.unwrap();

    let nutrient_id = NutrientRecord::load_from_database_using_id(nutrient_entity.get_id(), &pool).await.unwrap().nutrient_id;
    let output_unit_id = NutrientUnitRecord::from_nutrient_unit(output_unit, &pool).await.get_database_id(&pool).await.unwrap();
    let manual_record = NutrientQuantityRecord::from_values(quantity_record.clone().id, value, nutrient_id, output_unit_id);

    assert_eq!(quantity_record, manual_record);
}

#[tokio::test]
async fn test_to_nutrient_quantity() {
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

    let nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let mut nutrient_entity = Entity::new(nutrient.borrow().clone());

    let mut nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let save_res = nutrient_record.save_to_database(&pool).await;
    let new_entity_id = save_res.unwrap();
    nutrient_entity.set_id(Id::from_bytes(InnerIdType::Uuid, new_entity_id.clone().try_into().unwrap()));

    let value = 5.5;
    let output_unit = NutrientUnit::Mass(MassUnit::Kilogram);
    let nutrient_quantity = NutrientQuantity::from_rc_refcell(value, nutrient, output_unit).unwrap();
    let quantity_record = NutrientQuantityRecord::from_nutrient_quantity(nutrient_quantity.clone(), &pool).await.unwrap();

    let nutrient_id = NutrientRecord::load_from_database_using_id(nutrient_entity.get_id(), &pool).await.unwrap().nutrient_id;
    let output_unit_id = NutrientUnitRecord::from_nutrient_unit(output_unit, &pool).await.get_database_id(&pool).await.unwrap();
    let manual_record = NutrientQuantityRecord::from_values(quantity_record.clone().id, value, nutrient_id, output_unit_id);
    let manual_nutrient_quantity = manual_record.to_nutrient_quantity(&pool).await.unwrap();

    assert_eq!(nutrient_quantity, manual_nutrient_quantity);
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

    let nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let mut nutrient_entity = Entity::new(nutrient.borrow().clone());

    let mut nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let save_res = nutrient_record.save_to_database(&pool).await;
    let new_entity_id = save_res.unwrap();
    nutrient_entity.set_id(Id::from_bytes(InnerIdType::Uuid, new_entity_id.clone().try_into().unwrap()));

    let value = 5.5;
    let output_unit = NutrientUnit::Mass(MassUnit::Kilogram);
    let nutrient_quantity = NutrientQuantity::from_rc_refcell(value, nutrient, output_unit).unwrap();
    let quantity_record = NutrientQuantityRecord::from_nutrient_quantity(nutrient_quantity, &pool).await.unwrap();

    quantity_record.save_to_database(&pool).await.unwrap();

    let retrieved_quantity = NutrientQuantityRecord::load_from_database(quantity_record.clone().id, &pool).await.unwrap();
    assert_eq!(quantity_record, retrieved_quantity);

    quantity_record.delete_nutrient_quantity(&pool).await.unwrap();
}

