use std::collections::HashMap;

use identity::{entity::Entity, inner_id::InnerIdType, Id};
use nutrients::{
    nutrient::{link_parent_child, Nutrient},
    nutrient_units::NutrientUnit,
    records::{
        nutrient_record::{NutrientLinkNames, NutrientLinkRecord, NutrientRecord},
        nutrient_type_record::NutrientTypeRecord,
        nutrient_unit_record::NutrientUnitRecord
    },
    schema::{
        nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
        nutrient_type::NutrientType
    },
};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;
use uuid::Uuid;

#[tokio::test]
async fn test_from_nutrients() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

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

    
    // Create nutrients
    let iron = Nutrient::new_rc_refcell(String::from("Iron"), nutrient_type.clone(), main_unit);
    let heme_iron = Nutrient::new_rc_refcell(String::from("Heme Iron"), nutrient_type.clone(), main_unit);
    let non_heme_iron = Nutrient::new_rc_refcell(String::from("Non-heme Iron"), nutrient_type.clone(), main_unit);
    let non_heme_iron_a = Nutrient::new_rc_refcell(String::from("Non-heme Iron A"), nutrient_type.clone(), main_unit);
    let non_heme_iron_b = Nutrient::new_rc_refcell(String::from("Non-heme Iron B"), nutrient_type.clone(), main_unit);


    // link nutrients
    link_parent_child(&iron, &heme_iron).unwrap();
    link_parent_child(&iron, &non_heme_iron).unwrap();
    link_parent_child(&non_heme_iron, &non_heme_iron_a).unwrap();
    link_parent_child(&non_heme_iron, &non_heme_iron_b).unwrap();


    // Create entities from nutrients
    let mut iron_entity = Entity::new(iron.borrow().clone());
    let mut heme_iron_entity = Entity::new(heme_iron.borrow().clone());
    let mut non_heme_iron_entity = Entity::new(non_heme_iron.borrow().clone());
    let mut non_heme_iron_a_entity = Entity::new(non_heme_iron_a.borrow().clone());
    let mut non_heme_iron_b_entity = Entity::new(non_heme_iron_b.borrow().clone());
    // println!("iron_entity: {:#?}", iron_entity);


    // Get real entity ids
    let iron_record = NutrientRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let iron_entity_id = iron_record.save_to_database(&pool).await.unwrap();

    let heme_iron_record = NutrientRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let heme_iron_entity_id = heme_iron_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_record = NutrientRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_entity_id = non_heme_iron_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_a_record = NutrientRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_a_entity_id = non_heme_iron_a_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_b_record = NutrientRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_b_entity_id = non_heme_iron_b_record.save_to_database(&pool).await.unwrap();


    // update entity ids
    iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, iron_entity_id.clone().try_into().unwrap()));
    heme_iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, heme_iron_entity_id.clone().try_into().unwrap()));
    non_heme_iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_entity_id.clone().try_into().unwrap()));
    non_heme_iron_a_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_a_entity_id.clone().try_into().unwrap()));
    non_heme_iron_b_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_b_entity_id.clone().try_into().unwrap()));
    // println!("iron_entity_id: {:#?}", iron_entity_id.clone());
    // println!("heme_iron_entity_id: {:#?}", heme_iron_entity_id.clone());
    // println!("non_heme_iron_entity_id: {:#?}", non_heme_iron_entity_id.clone());

    // println!("iron entity: {:#?}", iron_entity);


    // Create link records
    let (mut iron_link_record, iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let (mut heme_iron_link_record, heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_link_record, non_heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_a_link_record, non_heme_iron_a_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_b_link_record, non_heme_iron_b_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    // println!("iron link record: {:#?}", iron_link_record);
    // println!("iron link names: {:#?}", iron_link_hashes);
    println!("heme iron link names: {:#?}", heme_iron_link_hashes);
    println!("non heme iron link names: {:#?}", non_heme_iron_link_hashes);
    // println!("heme iron link record: {:#?}", heme_iron_link_record);


    // Get nutrient entity Vec
    let nutrient_name_new_id_map: HashMap<String, Vec<u8>> = HashMap::from([
        (iron_entity.clone().get_inner().get_name(), iron_entity_id.clone()),
        (heme_iron_entity.clone().get_inner().get_name(), heme_iron_entity_id.clone()),
        (non_heme_iron_entity.clone().get_inner().get_name(), non_heme_iron_entity_id.clone()),
        (non_heme_iron_a_entity.clone().get_inner().get_name(), non_heme_iron_a_entity_id.clone()),
        (non_heme_iron_b_entity.clone().get_inner().get_name(), non_heme_iron_b_entity_id.clone()),
    ]);


    // Update link records with new ids
    iron_link_record = iron_link_record.update_nutrient_link_ids(&iron_link_hashes, &nutrient_name_new_id_map);
    heme_iron_link_record = heme_iron_link_record.update_nutrient_link_ids(&heme_iron_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_link_record = non_heme_iron_link_record.update_nutrient_link_ids(&non_heme_iron_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_a_link_record = non_heme_iron_a_link_record.update_nutrient_link_ids(&non_heme_iron_a_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_b_link_record = non_heme_iron_b_link_record.update_nutrient_link_ids(&non_heme_iron_b_link_hashes, &nutrient_name_new_id_map);


    // Create from value
    let iron_parent_ids = Vec::new();
    let iron_child_ids = Vec::from([heme_iron_entity_id.clone(), non_heme_iron_entity_id.clone()]);
    let iron_link_record_from_value = NutrientLinkRecord::from_values(
        iron_entity_id.clone(),
        iron_parent_ids,
        iron_child_ids,
    );

    let heme_iron_parent_ids = Vec::from([iron_entity_id.clone()]);
    let heme_iron_child_ids = Vec::new();
    let heme_iron_link_record_from_value = NutrientLinkRecord::from_values(
        heme_iron_entity_id,
        heme_iron_parent_ids,
        heme_iron_child_ids,
    );

    let non_heme_iron_parent_ids = Vec::from([iron_entity_id.clone()]);
    let non_heme_iron_child_ids = Vec::from([non_heme_iron_a_entity_id.clone(), non_heme_iron_b_entity_id.clone()]);
    let non_heme_iron_link_record_from_value = NutrientLinkRecord::from_values(
        non_heme_iron_entity_id.clone(),
        non_heme_iron_parent_ids,
        non_heme_iron_child_ids,
    );

    let non_heme_iron_a_parent_ids = Vec::from([non_heme_iron_entity_id.clone()]);
    let non_heme_iron_a_child_ids = Vec::new();
    let non_heme_iron_a_link_record_from_value = NutrientLinkRecord::from_values(
        non_heme_iron_a_entity_id,
        non_heme_iron_a_parent_ids,
        non_heme_iron_a_child_ids,
    );

    let non_heme_iron_b_parent_ids = Vec::from([non_heme_iron_entity_id.clone()]);
    let non_heme_iron_b_child_ids = Vec::new();
    let non_heme_iron_b_link_record_from_value = NutrientLinkRecord::from_values(
        non_heme_iron_b_entity_id,
        non_heme_iron_b_parent_ids,
        non_heme_iron_b_child_ids,
    );

    // println!("iron link record: {:#?}", iron_link_record.clone());
    // println!("iron link record value: {:#?}", iron_link_record_from_value.clone());
    // println!("heme iron link record: {:#?}", heme_iron_link_record.clone());
    // println!("non heme iron link record: {:#?}", non_heme_iron_link_record.clone());
    assert_eq!(iron_link_record, iron_link_record_from_value);
    assert_eq!(heme_iron_link_record, heme_iron_link_record_from_value);
    assert_eq!(non_heme_iron_link_record, non_heme_iron_link_record_from_value);
    assert_eq!(non_heme_iron_a_link_record, non_heme_iron_a_link_record_from_value);
    assert_eq!(non_heme_iron_b_link_record, non_heme_iron_b_link_record_from_value);
}

#[tokio::test]
async fn test_database_operations() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

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

    
    // Create nutrients
    let iron = Nutrient::new_rc_refcell(String::from("Iron"), nutrient_type.clone(), main_unit);
    let heme_iron = Nutrient::new_rc_refcell(String::from("Heme Iron"), nutrient_type.clone(), main_unit);
    let non_heme_iron = Nutrient::new_rc_refcell(String::from("Non-heme Iron"), nutrient_type.clone(), main_unit);
    let non_heme_iron_a = Nutrient::new_rc_refcell(String::from("Non-heme Iron A"), nutrient_type.clone(), main_unit);
    let non_heme_iron_b = Nutrient::new_rc_refcell(String::from("Non-heme Iron B"), nutrient_type.clone(), main_unit);


    // link nutrients
    link_parent_child(&iron, &heme_iron).unwrap();
    link_parent_child(&iron, &non_heme_iron).unwrap();
    link_parent_child(&non_heme_iron, &non_heme_iron_a).unwrap();
    link_parent_child(&non_heme_iron, &non_heme_iron_b).unwrap();


    // Create entities from nutrients
    let mut iron_entity = Entity::new(iron.borrow().clone());
    let mut heme_iron_entity = Entity::new(heme_iron.borrow().clone());
    let mut non_heme_iron_entity = Entity::new(non_heme_iron.borrow().clone());
    let mut non_heme_iron_a_entity = Entity::new(non_heme_iron_a.borrow().clone());
    let mut non_heme_iron_b_entity = Entity::new(non_heme_iron_b.borrow().clone());
    // println!("iron_entity: {:#?}", iron_entity);


    // Get real entity ids
    let iron_record = NutrientRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let iron_entity_id = iron_record.save_to_database(&pool).await.unwrap();

    let heme_iron_record = NutrientRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let heme_iron_entity_id = heme_iron_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_record = NutrientRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_entity_id = non_heme_iron_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_a_record = NutrientRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_a_entity_id = non_heme_iron_a_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_b_record = NutrientRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_b_entity_id = non_heme_iron_b_record.save_to_database(&pool).await.unwrap();


    // update entity ids
    iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, iron_entity_id.clone().try_into().unwrap()));
    heme_iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, heme_iron_entity_id.clone().try_into().unwrap()));
    non_heme_iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_entity_id.clone().try_into().unwrap()));
    non_heme_iron_a_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_a_entity_id.clone().try_into().unwrap()));
    non_heme_iron_b_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_b_entity_id.clone().try_into().unwrap()));
    // println!("iron_entity_id: {:#?}", iron_entity_id.clone());
    // println!("heme_iron_entity_id: {:#?}", heme_iron_entity_id.clone());
    // println!("non_heme_iron_entity_id: {:#?}", non_heme_iron_entity_id.clone());

    // println!("iron entity: {:#?}", iron_entity);


    // Create link records
    let (mut iron_link_record, iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let (mut heme_iron_link_record, heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_link_record, non_heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_a_link_record, non_heme_iron_a_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_b_link_record, non_heme_iron_b_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    // println!("iron link record: {:#?}", iron_link_record);
    // println!("iron link names: {:#?}", iron_link_hashes);
    // println!("heme iron link names: {:#?}", heme_iron_link_hashes);
    // println!("non heme iron link names: {:#?}", non_heme_iron_link_hashes);
    println!("heme iron link record: {:#?}", heme_iron_link_record);


    // Get nutrient entity Vec
    let nutrient_name_new_id_map: HashMap<String, Vec<u8>> = HashMap::from([
        (iron_entity.clone().get_inner().get_name(), iron_entity_id.clone()),
        (heme_iron_entity.clone().get_inner().get_name(), heme_iron_entity_id.clone()),
        (non_heme_iron_entity.clone().get_inner().get_name(), non_heme_iron_entity_id.clone()),
        (non_heme_iron_a_entity.clone().get_inner().get_name(), non_heme_iron_a_entity_id.clone()),
        (non_heme_iron_b_entity.clone().get_inner().get_name(), non_heme_iron_b_entity_id.clone()),
    ]);


    // Update link records with new ids
    iron_link_record = iron_link_record.update_nutrient_link_ids(&iron_link_hashes, &nutrient_name_new_id_map);
    heme_iron_link_record = heme_iron_link_record.update_nutrient_link_ids(&heme_iron_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_link_record = non_heme_iron_link_record.update_nutrient_link_ids(&non_heme_iron_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_a_link_record = non_heme_iron_a_link_record.update_nutrient_link_ids(&non_heme_iron_a_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_b_link_record = non_heme_iron_b_link_record.update_nutrient_link_ids(&non_heme_iron_b_link_hashes, &nutrient_name_new_id_map);


    // Save nutrient record
    let res = iron_record.save_to_database(&pool).await.unwrap();
    println!("res of iron_record: {:?}", res);
    let res = heme_iron_record.save_to_database(&pool).await.unwrap();
    println!("res of heme_iron_record: {:?}", res);
    let res = non_heme_iron_record.save_to_database(&pool).await.unwrap();
    println!("res of non_heme_iron_record: {:?}", res);
    let res = non_heme_iron_a_record.save_to_database(&pool).await.unwrap();
    println!("res of non_heme_iron_a_record: {:?}", res);
    let res = non_heme_iron_b_record.save_to_database(&pool).await.unwrap();
    println!("res of non_heme_iron_b_record: {:?}", res);


    // Save nutrient record links to database
    println!("iron_link_record: {:?}", iron_link_record);
    iron_link_record.save_to_database(&pool).await.unwrap(); 
    heme_iron_link_record.save_to_database(&pool).await.unwrap(); 
    non_heme_iron_link_record.save_to_database(&pool).await.unwrap(); 
    non_heme_iron_a_link_record.save_to_database(&pool).await.unwrap(); 
    non_heme_iron_b_link_record.save_to_database(&pool).await.unwrap(); 

    // load nutrient record links
    let mut loaded_iron_link_record = NutrientLinkRecord::load_from_sqlite(Uuid::from_slice(&iron_entity_id).unwrap(), &pool).await.unwrap();
    let mut loaded_heme_iron_link_record = NutrientLinkRecord::load_from_sqlite(Uuid::from_slice(&heme_iron_entity_id).unwrap(), &pool).await.unwrap();
    let mut loaded_non_heme_iron_link_record = NutrientLinkRecord::load_from_sqlite(Uuid::from_slice(&non_heme_iron_entity_id).unwrap(), &pool).await.unwrap();
    let mut loaded_non_heme_iron_a_link_record = NutrientLinkRecord::load_from_sqlite(Uuid::from_slice(&non_heme_iron_a_entity_id).unwrap(), &pool).await.unwrap();
    let mut loaded_non_heme_iron_b_link_record = NutrientLinkRecord::load_from_sqlite(Uuid::from_slice(&non_heme_iron_b_entity_id).unwrap(), &pool).await.unwrap();

    // sort parents and children of link records
    loaded_iron_link_record.sort();
    loaded_heme_iron_link_record.sort();
    loaded_non_heme_iron_link_record.sort();
    loaded_non_heme_iron_a_link_record.sort();
    loaded_non_heme_iron_b_link_record.sort();

    iron_link_record.sort();
    heme_iron_link_record.sort();
    non_heme_iron_link_record.sort();
    non_heme_iron_a_link_record.sort();
    non_heme_iron_b_link_record.sort();

    assert_eq!(iron_link_record, loaded_iron_link_record);
    assert_eq!(heme_iron_link_record, loaded_heme_iron_link_record);
    assert_eq!(non_heme_iron_link_record, loaded_non_heme_iron_link_record);
    assert_eq!(non_heme_iron_a_link_record, loaded_non_heme_iron_a_link_record);
    assert_eq!(non_heme_iron_b_link_record, loaded_non_heme_iron_b_link_record);
}

#[tokio::test]
async fn test_get_link_names() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

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

    
    // Create nutrients
    let iron = Nutrient::new_rc_refcell(String::from("Iron"), nutrient_type.clone(), main_unit);
    let heme_iron = Nutrient::new_rc_refcell(String::from("Heme Iron"), nutrient_type.clone(), main_unit);
    let non_heme_iron = Nutrient::new_rc_refcell(String::from("Non-heme Iron"), nutrient_type.clone(), main_unit);
    let non_heme_iron_a = Nutrient::new_rc_refcell(String::from("Non-heme Iron A"), nutrient_type.clone(), main_unit);
    let non_heme_iron_b = Nutrient::new_rc_refcell(String::from("Non-heme Iron B"), nutrient_type.clone(), main_unit);


    // link nutrients
    link_parent_child(&iron, &heme_iron).unwrap();
    link_parent_child(&iron, &non_heme_iron).unwrap();
    link_parent_child(&non_heme_iron, &non_heme_iron_a).unwrap();
    link_parent_child(&non_heme_iron, &non_heme_iron_b).unwrap();


    // Create entities from nutrients
    let mut iron_entity = Entity::new(iron.borrow().clone());
    let mut heme_iron_entity = Entity::new(heme_iron.borrow().clone());
    let mut non_heme_iron_entity = Entity::new(non_heme_iron.borrow().clone());
    let mut non_heme_iron_a_entity = Entity::new(non_heme_iron_a.borrow().clone());
    let mut non_heme_iron_b_entity = Entity::new(non_heme_iron_b.borrow().clone());
    // println!("iron_entity: {:#?}", iron_entity);


    // Get real entity ids
    let iron_record = NutrientRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let iron_entity_id = iron_record.save_to_database(&pool).await.unwrap();

    let heme_iron_record = NutrientRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let heme_iron_entity_id = heme_iron_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_record = NutrientRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_entity_id = non_heme_iron_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_a_record = NutrientRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_a_entity_id = non_heme_iron_a_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_b_record = NutrientRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_b_entity_id = non_heme_iron_b_record.save_to_database(&pool).await.unwrap();


    // update entity ids
    iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, iron_entity_id.clone().try_into().unwrap()));
    heme_iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, heme_iron_entity_id.clone().try_into().unwrap()));
    non_heme_iron_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_entity_id.clone().try_into().unwrap()));
    non_heme_iron_a_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_a_entity_id.clone().try_into().unwrap()));
    non_heme_iron_b_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_b_entity_id.clone().try_into().unwrap()));
    // println!("iron_entity_id: {:#?}", iron_entity_id.clone());
    // println!("heme_iron_entity_id: {:#?}", heme_iron_entity_id.clone());
    // println!("non_heme_iron_entity_id: {:#?}", non_heme_iron_entity_id.clone());

    // println!("iron entity: {:#?}", iron_entity);


    // Create link records
    let (mut iron_link_record, iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let (mut heme_iron_link_record, heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_link_record, non_heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_a_link_record, non_heme_iron_a_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_b_link_record, non_heme_iron_b_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    // println!("iron link record: {:#?}", iron_link_record);
    // println!("iron link names: {:#?}", iron_link_hashes);
    // println!("heme iron link names: {:#?}", heme_iron_link_hashes);
    // println!("non heme iron link names: {:#?}", non_heme_iron_link_hashes);
    println!("heme iron link record: {:#?}", heme_iron_link_record);


    // Get nutrient entity Vec
    let nutrient_name_new_id_map: HashMap<String, Vec<u8>> = HashMap::from([
        (iron_entity.clone().get_inner().get_name(), iron_entity_id.clone()),
        (heme_iron_entity.clone().get_inner().get_name(), heme_iron_entity_id.clone()),
        (non_heme_iron_entity.clone().get_inner().get_name(), non_heme_iron_entity_id.clone()),
        (non_heme_iron_a_entity.clone().get_inner().get_name(), non_heme_iron_a_entity_id.clone()),
        (non_heme_iron_b_entity.clone().get_inner().get_name(), non_heme_iron_b_entity_id.clone()),
    ]);


    // Update link records with new ids
    iron_link_record = iron_link_record.update_nutrient_link_ids(&iron_link_hashes, &nutrient_name_new_id_map);
    heme_iron_link_record = heme_iron_link_record.update_nutrient_link_ids(&heme_iron_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_link_record = non_heme_iron_link_record.update_nutrient_link_ids(&non_heme_iron_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_a_link_record = non_heme_iron_a_link_record.update_nutrient_link_ids(&non_heme_iron_a_link_hashes, &nutrient_name_new_id_map);
    non_heme_iron_b_link_record = non_heme_iron_b_link_record.update_nutrient_link_ids(&non_heme_iron_b_link_hashes, &nutrient_name_new_id_map);


    // Save nutrient record
    let res = iron_record.save_to_database(&pool).await.unwrap();
    println!("res of iron_record: {:?}", res);
    let res = heme_iron_record.save_to_database(&pool).await.unwrap();
    println!("res of heme_iron_record: {:?}", res);
    let res = non_heme_iron_record.save_to_database(&pool).await.unwrap();
    println!("res of non_heme_iron_record: {:?}", res);
    let res = non_heme_iron_a_record.save_to_database(&pool).await.unwrap();
    println!("res of non_heme_iron_a_record: {:?}", res);
    let res = non_heme_iron_b_record.save_to_database(&pool).await.unwrap();
    println!("res of non_heme_iron_b_record: {:?}", res);


    // Save nutrient record links to database
    iron_link_record.save_to_database(&pool).await.unwrap(); 
    heme_iron_link_record.save_to_database(&pool).await.unwrap(); 
    non_heme_iron_link_record.save_to_database(&pool).await.unwrap(); 
    non_heme_iron_a_link_record.save_to_database(&pool).await.unwrap(); 
    non_heme_iron_b_link_record.save_to_database(&pool).await.unwrap(); 


    // Get nutrient link names
    let iron_nutrient_names = iron_link_record.get_nutrient_link_names(&pool).await.unwrap();
    let heme_iron_nutrient_names = heme_iron_link_record.get_nutrient_link_names(&pool).await.unwrap();
    let non_heme_iron_nutrient_names = non_heme_iron_link_record.get_nutrient_link_names(&pool).await.unwrap();
    let non_heme_iron_a_nutrient_names = non_heme_iron_a_link_record.get_nutrient_link_names(&pool).await.unwrap();
    let non_heme_iron_b_nutrient_names = non_heme_iron_b_link_record.get_nutrient_link_names(&pool).await.unwrap();


    // Create manual link names
    let iron_manual_names = NutrientLinkNames {
        parent_names: Vec::new(),
        child_names: Vec::from(["Heme Iron".to_string(), "Non-heme Iron".to_string()]),
    };
    let heme_iron_manual_names = NutrientLinkNames {
        parent_names: Vec::from(["Iron".to_string()]),
        child_names: Vec::new(),
    };
    let non_heme_iron_manual_names = NutrientLinkNames {
        parent_names: Vec::from(["Iron".to_string()]),
        child_names: Vec::from(["Non-heme Iron A".to_string(), "Non-heme Iron B".to_string()]),
    };
    let non_heme_iron_a_manual_names = NutrientLinkNames {
        parent_names: Vec::from(["Non-heme Iron".to_string()]),
        child_names: Vec::new(),
    };
    let non_heme_iron_b_manual_names = NutrientLinkNames {
        parent_names: Vec::from(["Non-heme Iron".to_string()]),
        child_names: Vec::new(),
    };
    assert_eq!(iron_nutrient_names, iron_manual_names);
    assert_eq!(heme_iron_nutrient_names, heme_iron_manual_names);
    assert_eq!(non_heme_iron_nutrient_names, non_heme_iron_manual_names);
    assert_eq!(non_heme_iron_a_nutrient_names, non_heme_iron_a_manual_names);
    assert_eq!(non_heme_iron_b_nutrient_names, non_heme_iron_b_manual_names);
}

