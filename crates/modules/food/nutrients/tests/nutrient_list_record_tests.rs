use identity::{inner_id::InnerIdType, Id};
use nutrients::{
    entity::Entity, nutrient::{ link_parent_child, Nutrient }, nutrient_list::NutrientList, nutrient_units::NutrientUnit,
    records::{
        nutrient_list_record::{NutrientListItemRecord, NutrientListRecord},
        nutrient_record::NutrientRecord,
        nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord},
    schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
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
    nutrient_list.set_name(String::from("test_list"));
    
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


    // Add nutrients to nutrient list
    nutrient_list.push(iron);
    nutrient_list.push(heme_iron);
    nutrient_list.push(non_heme_iron);
    nutrient_list.push(non_heme_iron_a);
    nutrient_list.push(non_heme_iron_b);


    // Create nutrient list record
    let nutrient_list_record = NutrientListRecord::from_nutrient_list(nutrient_list.clone());
    let manual_list_record = NutrientListRecord::from_value(nutrient_list.get_id().as_bytes().to_vec(), nutrient_list.get_name(), nutrient_list.get_description());

    assert_eq!(nutrient_list_record, manual_list_record);
}

#[tokio::test]
async fn test_to_nutrient_list() {
    let nutrient_list = NutrientList::new();
    let nutrient_list_record = NutrientListRecord::from_nutrient_list(nutrient_list.clone());
    let converted_list = nutrient_list_record.to_nutrient_quantity_list();

    assert_eq!(nutrient_list.get_id(), converted_list.get_id())
}

#[tokio::test]
async fn test_database_operations() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let mut nutrient_list_a = NutrientList::new();
    let mut nutrient_list_b = NutrientList::new();
    let mut nutrient_list_c = NutrientList::new();
    
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


    // Add nutrients to nutrient list
    nutrient_list_a.push(iron.clone());
    nutrient_list_a.push(heme_iron.clone());
    nutrient_list_a.push(non_heme_iron.clone());
    nutrient_list_a.push(non_heme_iron_a);
    nutrient_list_a.push(non_heme_iron_b);

    nutrient_list_b.push(iron);
    nutrient_list_b.push(heme_iron);
    nutrient_list_b.push(non_heme_iron);


    // Create nutrient list record
    let nutrient_list_a_record = NutrientListRecord::from_nutrient_list(nutrient_list_a.clone());
    let nutrient_list_b_record = NutrientListRecord::from_nutrient_list(nutrient_list_b.clone());
    let nutrient_list_c_record = NutrientListRecord::from_nutrient_list(nutrient_list_c.clone());


    // save db
    nutrient_list_a_record.save_or_update_to_database(&pool).await.unwrap();
    nutrient_list_b_record.save_or_update_to_database(&pool).await.unwrap();
    nutrient_list_c_record.save_or_update_to_database(&pool).await.unwrap();

    let nutrient_list_a_item_record_vec = NutrientListItemRecord::from_nutrient_list(nutrient_list_a.clone(), &pool).await.unwrap();
    let mut refs: Vec<&NutrientListItemRecord> = nutrient_list_a_item_record_vec.iter().collect();
    NutrientListItemRecord::save_vec_to_database(refs, &pool).await.unwrap();


    // Get all
    let retrieved_nutrient_list_records = NutrientListRecord::get_all_from_sqlite(&pool).await.unwrap();
    assert!(retrieved_nutrient_list_records.iter().any(|item| item == &nutrient_list_a_record || item == &nutrient_list_b_record || item == &nutrient_list_c_record));

    let retrieved_nutrients = nutrient_list_a_record.load_nutrients_from_database(&pool).await.unwrap();
    let retrieved_nutrient_names: Vec<String> = retrieved_nutrients.iter().map(|nutrient| nutrient.get_name()).collect();
    let nutrient_names: Vec<String> = nutrient_list_a.get_nutrients().iter().map(|nutrient| nutrient.borrow().get_name()).collect();
    assert_eq!(nutrient_names.len(), retrieved_nutrient_names.len());
    assert!(nutrient_names.iter().all(|name| retrieved_nutrient_names.contains(name)));
    

    // delete from db
    nutrient_list_a_record.delete_from_database(&pool).await.unwrap();
    nutrient_list_b_record.delete_from_database(&pool).await.unwrap();
    nutrient_list_c_record.delete_from_database(&pool).await.unwrap();
}

