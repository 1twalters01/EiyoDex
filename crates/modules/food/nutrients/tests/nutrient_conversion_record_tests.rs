use identity::{entity::Entity, inner_id::InnerIdType, Id};
use nutrients::{
    nutrient::Nutrient,
    nutrient_units::NutrientUnit,
    records::{
        nutrient_record::{NutrientConversionRecord, NutrientRecord},
        nutrient_type_record::NutrientTypeRecord,
        nutrient_unit_record::NutrientUnitRecord
    },
    schema::{
        nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
        nutrient_type::NutrientType
    },
};
use units::{energy::unit::EnergyUnit, mass::{quantity::MassQuantity, unit::MassUnit}, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

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

    let nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let mut nutrient_entity = Entity::new(nutrient.borrow().clone());
    let nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let save_res = nutrient_record.save_to_database(&pool).await;

    let new_entity_id = save_res.unwrap();
    nutrient_entity.set_id(Id::from_bytes(InnerIdType::Uuid, new_entity_id.try_into().unwrap()));

    let nutrient_conversion_record = NutrientConversionRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    println!("nutrient conversion record: {:#?}", nutrient_conversion_record);
    println!("conversions: {:#?}", nutrient.borrow().get_unit_conversions());

    assert_eq!(nutrient_conversion_record.len(), 5);

    let nutrient_entity_id_vec = nutrient_entity.get_id().to_bytes().to_vec();
    let ounces_id = NutrientUnitRecord::from_nutrient_unit(NutrientUnit::Mass(MassUnit::Ounce), &pool).await.get_database_id(&pool).await.unwrap();
    let ounces_value = MassQuantity::new(1f64, MassUnit::Ounce).as_mg();
    let ounce_conversion_instance = NutrientConversionRecord::from_values(nutrient_entity_id_vec, ounces_id, ounces_value);
    assert!(nutrient_conversion_record.contains(&ounce_conversion_instance));
}

#[tokio::test]
async fn test_to_btreemap() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

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

    let nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let mut nutrient_entity = Entity::new(nutrient.borrow().clone());
    let nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let save_res = nutrient_record.save_to_database(&pool).await;

    let new_entity_id = save_res.unwrap();
    nutrient_entity.set_id(Id::from_bytes(InnerIdType::Uuid, new_entity_id.try_into().unwrap()));

    let nutrient_conversion_record = NutrientConversionRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let conversion_btreemap = NutrientConversionRecord::to_btree_map_from_vec(nutrient_conversion_record, &pool).await.unwrap();
    assert_eq!(
        conversion_btreemap,
        nutrient.borrow().get_unit_conversions()
    );
}

#[tokio::test]
async fn test_database_operations() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

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

    let nutrient = Nutrient::new_rc_refcell(name.clone(), nutrient_type, main_unit);
    nutrient.borrow_mut().set_description(description.clone());
    let mut nutrient_entity = Entity::new(nutrient.borrow().clone());

    let nutrient_record = NutrientRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    let save_res = nutrient_record.save_to_database(&pool).await;

    let new_entity_id = save_res.unwrap();
    nutrient_entity.set_id(Id::from_bytes(InnerIdType::Uuid, new_entity_id.try_into().unwrap()));

    let mut nutrient_conversion_record = NutrientConversionRecord::from_nutrient_entity(nutrient_entity.clone(), &pool).await.unwrap();
    println!("nutrient conversion record: {:#?}", nutrient_conversion_record);
    let save_res = NutrientConversionRecord::save_vec_to_database(&nutrient_conversion_record, &pool).await;
    println!("save res: {:#?}", save_res);
    assert!(save_res.is_ok());

    let nutrient_id = nutrient_entity.get_id();
    let load_res = NutrientConversionRecord::load_from_database(nutrient_id.clone(), &pool).await;
    println!("load res: {:#?}", load_res);
    assert!(load_res.is_ok());

    let mut loaded_nutrient_conversion_record = load_res.unwrap();
    NutrientConversionRecord::sort_records(&mut loaded_nutrient_conversion_record);
    NutrientConversionRecord::sort_records(&mut nutrient_conversion_record);
    assert_eq!(loaded_nutrient_conversion_record, nutrient_conversion_record);

    let delete_res = NutrientConversionRecord::delete_all_conversions_from_database(nutrient_id, &pool).await;
    assert!(delete_res.is_ok())
}
