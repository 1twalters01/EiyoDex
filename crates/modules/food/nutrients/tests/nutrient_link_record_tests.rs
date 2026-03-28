use std::collections::HashMap;

use identity::{inner_id::InnerIdType, Id};
use nutrients::{
    entity::Entity,
    nutrient::{link_parent_child, Nutrient},
    nutrient_units::NutrientUnit,
    records::{
        nutrient_record::{NutrientConversionRecord, NutrientLinkRecord, NutrientRecord},
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
    let (mut iron_link_record, iron_link_names) = NutrientLinkRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let (mut heme_iron_link_record, heme_iron_link_names) = NutrientLinkRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_link_record, non_heme_iron_link_names) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_a_link_record, non_heme_iron_a_link_names) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_b_link_record, non_heme_iron_b_link_names) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();
    // println!("iron link record: {:#?}", iron_link_record);
    // println!("iron link names: {:#?}", iron_link_names);
    println!("heme iron link names: {:#?}", heme_iron_link_names);
    println!("non heme iron link names: {:#?}", non_heme_iron_link_names);
    // println!("heme iron link record: {:#?}", heme_iron_link_record);


    // Get nutrient entity Vec
    let nutrient_name_id_map: HashMap<String, Vec<u8>> = HashMap::from([
        (iron_entity.clone().get_inner().get_name(), iron_entity_id.clone()),
        (heme_iron_entity.clone().get_inner().get_name(), heme_iron_entity_id.clone()),
        (non_heme_iron_entity.clone().get_inner().get_name(), non_heme_iron_entity_id.clone()),
        (non_heme_iron_a_entity.clone().get_inner().get_name(), non_heme_iron_a_entity_id.clone()),
        (non_heme_iron_b_entity.clone().get_inner().get_name(), non_heme_iron_b_entity_id.clone()),
    ]);


    // Update link records with new ids
    for id in &mut iron_link_record.child_ids {
        if let Some(name) = iron_link_names.child_map.get(id) {
            if let Some(new_id) = nutrient_name_id_map.get(name) {
                *id = new_id.clone();
            }
        }
    }

    for id in &mut heme_iron_link_record.parent_ids {
        if let Some(name) = heme_iron_link_names.parent_map.get(id) {
            if let Some(new_id) = nutrient_name_id_map.get(name) {
                *id = new_id.clone();
            }
        }
    }

    for id in &mut non_heme_iron_link_record.parent_ids {
        if let Some(name) = non_heme_iron_link_names.parent_map.get(id) {
            if let Some(new_id) = nutrient_name_id_map.get(name) {
                *id = new_id.clone();
            }
        }
    }

    for id in &mut non_heme_iron_link_record.child_ids {
        if let Some(name) = non_heme_iron_link_names.child_map.get(id) {
            if let Some(new_id) = nutrient_name_id_map.get(name) {
                *id = new_id.clone();
            }
        }
    }

    for id in &mut non_heme_iron_a_link_record.parent_ids {
        if let Some(name) = non_heme_iron_a_link_names.parent_map.get(id) {
            if let Some(new_id) = nutrient_name_id_map.get(name) {
                *id = new_id.clone();
            }
        }
    }

    for id in &mut non_heme_iron_b_link_record.parent_ids {
        if let Some(name) = non_heme_iron_b_link_names.parent_map.get(id) {
            if let Some(new_id) = nutrient_name_id_map.get(name) {
                *id = new_id.clone();
            }
        }
    }


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
async fn test_to_nutrients() {
}

