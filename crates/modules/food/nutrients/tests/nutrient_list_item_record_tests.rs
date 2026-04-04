use std::collections::HashMap;

use identity::{inner_id::InnerIdType, Id};
use nutrients::{
    entity::Entity, nutrient::{link_parent_child, Nutrient}, nutrient_list::NutrientList, nutrient_units::NutrientUnit, records::{nutrient_list_record::{NutrientListItemRecord, NutrientListRecord}, nutrient_record::{NutrientLinkRecord, NutrientRecord}, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient_list() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let mut nutrient_list = NutrientList::new();
    
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


    // Create link records
    let (mut iron_link_record, iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(iron_entity.clone(), &pool).await.unwrap();
    let (mut heme_iron_link_record, heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_link_record, non_heme_iron_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_a_link_record, non_heme_iron_a_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_a_entity.clone(), &pool).await.unwrap();
    let (mut non_heme_iron_b_link_record, non_heme_iron_b_link_hashes) = NutrientLinkRecord::from_nutrient_entity(non_heme_iron_b_entity.clone(), &pool).await.unwrap();


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
    iron_record.save_to_database(&pool).await.unwrap();
    heme_iron_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_a_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_b_record.save_to_database(&pool).await.unwrap();


    // Save nutrient record links to database
    let _ = iron_link_record.save_to_database(&pool).await.unwrap(); 
    let _ = heme_iron_link_record.save_to_database(&pool).await.unwrap(); 
    let _ = non_heme_iron_link_record.save_to_database(&pool).await.unwrap(); 
    let _ = non_heme_iron_a_link_record.save_to_database(&pool).await.unwrap(); 
    let _ = non_heme_iron_b_link_record.save_to_database(&pool).await.unwrap(); 


    // Add nutrients to nutrient list
    nutrient_list.push(iron);
    nutrient_list.push(heme_iron);
    nutrient_list.push(non_heme_iron);
    nutrient_list.push(non_heme_iron_a);
    nutrient_list.push(non_heme_iron_b);


    // Create nutrient list record and nutrient list items record
    let nutrient_list_record = NutrientListRecord::from_nutrient_list(nutrient_list.clone());
    let mut nutrient_list_item_record_vec = NutrientListItemRecord::from_nutrient_list(nutrient_list.clone(), &pool).await.unwrap();

    let nutrient_list_id = nutrient_list.get_id().as_bytes().to_vec();
    let manual_vec = Vec::from([
        NutrientListItemRecord::from_value(nutrient_list_id.clone(), iron_entity_id),
        NutrientListItemRecord::from_value(nutrient_list_id.clone(), heme_iron_entity_id),
        NutrientListItemRecord::from_value(nutrient_list_id.clone(), non_heme_iron_entity_id),
        NutrientListItemRecord::from_value(nutrient_list_id.clone(), non_heme_iron_a_entity_id),
        NutrientListItemRecord::from_value(nutrient_list_id.clone(), non_heme_iron_b_entity_id),
    ]);

    assert_eq!(nutrient_list_item_record_vec, manual_vec);
}

#[tokio::test]
async fn test_database_operations() {
}

