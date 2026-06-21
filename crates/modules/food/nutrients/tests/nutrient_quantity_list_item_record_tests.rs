use identity::{entity::Entity, inner_id::InnerIdType, Id};
use nutrients::{nutrient::{link_parent_child, Nutrient}, nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList, nutrient_units::NutrientUnit, records::{nutrient_quantity_list_record::{NutrientQuantityListItemRecord, NutrientQuantityListRecord}, nutrient_quantity_record::NutrientQuantityRecord, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;
use uuid::Uuid;

#[tokio::test]
async fn test_from_nutrient_quantity_list() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let mut nutrient_quantity_list = NutrientQuantityList::new();
    nutrient_quantity_list.set_name(String::from("test_list"));
    
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


    // create nutrient quantities
    let value = 5f64;
    let mut iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_a_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_b_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();


    // Create entities from nutrient quantities
    let mut iron_quantity_entity = Entity::new(iron_quantity.clone());
    let mut heme_iron_quantity_entity = Entity::new(heme_iron_quantity.clone());
    let mut non_heme_iron_quantity_entity = Entity::new(non_heme_iron_quantity.clone());
    let mut non_heme_iron_a_quantity_entity = Entity::new(non_heme_iron_a_quantity.clone());
    let mut non_heme_iron_b_quantity_entity = Entity::new(non_heme_iron_b_quantity.clone());


    // Get real entity ids
    let iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(iron_quantity_entity.clone(), &pool).await.unwrap();
    let iron_quantity_entity_id = iron_quantity_record.save_to_database(&pool).await.unwrap();

    let heme_iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(heme_iron_quantity_entity.clone(), &pool).await.unwrap();
    let heme_iron_quantity_entity_id = heme_iron_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_quantity_entity_id = non_heme_iron_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_a_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_a_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_a_quantity_entity_id = non_heme_iron_a_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_b_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_b_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_b_quantity_entity_id = non_heme_iron_b_quantity_record.save_to_database(&pool).await.unwrap();


    // update entity ids
    iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, iron_quantity_entity_id.clone().try_into().unwrap()));
    heme_iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, heme_iron_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_a_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_a_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_b_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_b_quantity_entity_id.clone().try_into().unwrap()));


    // Save nutrient quantity records
    iron_quantity_record.save_to_database(&pool).await.unwrap();
    heme_iron_quantity_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_quantity_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_a_quantity_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_b_quantity_record.save_to_database(&pool).await.unwrap();


    // Add nutrients to nutrient list
    nutrient_quantity_list.push(iron_quantity_entity);
    nutrient_quantity_list.push(heme_iron_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_a_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_b_quantity_entity);


    // Create nutrient quantity list items record
    let nutrient_quantity_list_item_record_vec = NutrientQuantityListItemRecord::from_nutrient_quantity_list(nutrient_quantity_list.clone()).await;

    let nutrient_quantity_list_id = nutrient_quantity_list.get_id().as_bytes().to_vec();
    let manual_vec = Vec::from([
        NutrientQuantityListItemRecord::from_value(nutrient_quantity_list_id.clone(), iron_quantity_entity_id),
        NutrientQuantityListItemRecord::from_value(nutrient_quantity_list_id.clone(), heme_iron_quantity_entity_id),
        NutrientQuantityListItemRecord::from_value(nutrient_quantity_list_id.clone(), non_heme_iron_quantity_entity_id),
        NutrientQuantityListItemRecord::from_value(nutrient_quantity_list_id.clone(), non_heme_iron_a_quantity_entity_id),
        NutrientQuantityListItemRecord::from_value(nutrient_quantity_list_id.clone(), non_heme_iron_b_quantity_entity_id),
    ]);

    assert_eq!(nutrient_quantity_list_item_record_vec.len(), manual_vec.len());
    assert!(nutrient_quantity_list_item_record_vec.iter().all(|item| manual_vec.contains(item)));
}

#[tokio::test]
async fn test_to_nutrient_quantity() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let mut nutrient_quantity_list = NutrientQuantityList::new();
    nutrient_quantity_list.set_name(String::from("test_list"));
    
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


    // create nutrient quantities
    let value = 5f64;
    let mut iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_a_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_b_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();


    // Create entities from nutrient quantities
    let mut iron_quantity_entity = Entity::new(iron_quantity.clone());
    let mut heme_iron_quantity_entity = Entity::new(heme_iron_quantity.clone());
    let mut non_heme_iron_quantity_entity = Entity::new(non_heme_iron_quantity.clone());
    let mut non_heme_iron_a_quantity_entity = Entity::new(non_heme_iron_a_quantity.clone());
    let mut non_heme_iron_b_quantity_entity = Entity::new(non_heme_iron_b_quantity.clone());


    // Get real entity ids
    let iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(iron_quantity_entity.clone(), &pool).await.unwrap();
    let iron_quantity_entity_id = iron_quantity_record.save_to_database(&pool).await.unwrap();

    let heme_iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(heme_iron_quantity_entity.clone(), &pool).await.unwrap();
    let heme_iron_quantity_entity_id = heme_iron_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_quantity_entity_id = non_heme_iron_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_a_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_a_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_a_quantity_entity_id = non_heme_iron_a_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_b_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_b_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_b_quantity_entity_id = non_heme_iron_b_quantity_record.save_to_database(&pool).await.unwrap();


    // update entity ids
    iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, iron_quantity_entity_id.clone().try_into().unwrap()));
    heme_iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, heme_iron_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_a_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_a_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_b_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_b_quantity_entity_id.clone().try_into().unwrap()));


    // Save nutrient quantity records
    iron_quantity_record.save_to_database(&pool).await.unwrap();
    heme_iron_quantity_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_quantity_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_a_quantity_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_b_quantity_record.save_to_database(&pool).await.unwrap();


    // Add nutrients to nutrient quantity list
    nutrient_quantity_list.push(iron_quantity_entity);
    nutrient_quantity_list.push(heme_iron_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_a_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_b_quantity_entity);


    // Create nutrient quantity list items record
    let nutrient_quantity_list_item_record = NutrientQuantityListItemRecord::from_value(nutrient_quantity_list.get_id().as_bytes().to_vec(), iron_quantity_entity_id);
    let transformed_nutrient = nutrient_quantity_list_item_record.to_nutrient_quantity(&pool).await.unwrap();
    assert_eq!(transformed_nutrient.get_value(), value);
    assert_eq!(transformed_nutrient.get_output_unit(), NutrientUnit::Mass(MassUnit::Kilogram));
    assert_eq!(transformed_nutrient.get_nutrient().borrow().clone().get_name(), iron.borrow().get_name());
}

#[tokio::test]
async fn test_database_operations() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let mut nutrient_quantity_list_a = NutrientQuantityList::new();
    let mut nutrient_quantity_list_b = NutrientQuantityList::new();
    let mut nutrient_quantity_list_c = NutrientQuantityList::new();
    
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


    // create nutrient quantities
    let value = 5f64;
    let mut iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_a_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let mut non_heme_iron_b_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();


    // Create entities from nutrient quantities
    let mut iron_quantity_entity = Entity::new(iron_quantity.clone());
    let mut heme_iron_quantity_entity = Entity::new(heme_iron_quantity.clone());
    let mut non_heme_iron_quantity_entity = Entity::new(non_heme_iron_quantity.clone());
    let mut non_heme_iron_a_quantity_entity = Entity::new(non_heme_iron_a_quantity.clone());
    let mut non_heme_iron_b_quantity_entity = Entity::new(non_heme_iron_b_quantity.clone());


    // Get real entity ids
    let iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(iron_quantity_entity.clone(), &pool).await.unwrap();
    let iron_quantity_entity_id = iron_quantity_record.save_to_database(&pool).await.unwrap();

    let heme_iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(heme_iron_quantity_entity.clone(), &pool).await.unwrap();
    let heme_iron_quantity_entity_id = heme_iron_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_quantity_entity_id = non_heme_iron_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_a_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_a_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_a_quantity_entity_id = non_heme_iron_a_quantity_record.save_to_database(&pool).await.unwrap();

    let non_heme_iron_b_quantity_record = NutrientQuantityRecord::from_nutrient_quantity_entity(non_heme_iron_b_quantity_entity.clone(), &pool).await.unwrap();
    let non_heme_iron_b_quantity_entity_id = non_heme_iron_b_quantity_record.save_to_database(&pool).await.unwrap();


    // update entity ids
    iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, iron_quantity_entity_id.clone().try_into().unwrap()));
    heme_iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, heme_iron_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_a_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_a_quantity_entity_id.clone().try_into().unwrap()));
    non_heme_iron_b_quantity_entity.set_id(Id::from_bytes(InnerIdType::Uuid, non_heme_iron_b_quantity_entity_id.clone().try_into().unwrap()));


    // Add nutrients to nutrient list
    nutrient_quantity_list_a.push(iron_quantity_entity.clone());
    nutrient_quantity_list_a.push(heme_iron_quantity_entity.clone());
    nutrient_quantity_list_a.push(non_heme_iron_quantity_entity.clone());
    nutrient_quantity_list_a.push(non_heme_iron_a_quantity_entity.clone());
    nutrient_quantity_list_a.push(non_heme_iron_b_quantity_entity.clone());

    nutrient_quantity_list_b.push(iron_quantity_entity.clone());
    nutrient_quantity_list_b.push(heme_iron_quantity_entity.clone());
    nutrient_quantity_list_b.push(non_heme_iron_quantity_entity.clone());


    // Create nutrient list record
    let nutrient_quantity_list_a_record = NutrientQuantityListRecord::from_nutrient_quantity_list(nutrient_quantity_list_a.clone());
    let nutrient_quantity_list_b_record = NutrientQuantityListRecord::from_nutrient_quantity_list(nutrient_quantity_list_b.clone());
    let nutrient_quantity_list_c_record = NutrientQuantityListRecord::from_nutrient_quantity_list(nutrient_quantity_list_c.clone());


    // save db
    nutrient_quantity_list_a_record.save_or_update_to_database(&pool).await.unwrap();
    nutrient_quantity_list_b_record.save_or_update_to_database(&pool).await.unwrap();
    nutrient_quantity_list_c_record.save_or_update_to_database(&pool).await.unwrap();


    // Create nutrient list items record
    let nutrient_quantity_list_a_item_record_vec = NutrientQuantityListItemRecord::from_nutrient_quantity_list(nutrient_quantity_list_a.clone()).await;
    let nutrient_quantity_list_b_item_record_vec = NutrientQuantityListItemRecord::from_nutrient_quantity_list(nutrient_quantity_list_b.clone()).await;
    let nutrient_quantity_list_c_item_record_vec = NutrientQuantityListItemRecord::from_nutrient_quantity_list(nutrient_quantity_list_c.clone()).await;


    // save nutrient list items record
    let mut refs: Vec<&NutrientQuantityListItemRecord> = nutrient_quantity_list_a_item_record_vec.iter().collect();
    NutrientQuantityListItemRecord::save_vec_to_database(refs, &pool).await.unwrap();
    
    for item_record in &nutrient_quantity_list_b_item_record_vec {
        item_record.save_to_database(&pool).await.unwrap();
    }

    refs = nutrient_quantity_list_c_item_record_vec.iter().collect();
    NutrientQuantityListItemRecord::save_vec_to_database(refs, &pool).await.unwrap();


    // load items from sqlite
    let retrieved_nutrient_quantity_list_a_items = NutrientQuantityListItemRecord::load_all_from_sqlite(
        Uuid::from_slice(&nutrient_quantity_list_a_record.get_id()).unwrap(), &pool
    ).await.unwrap();
    let retrieved_nutrient_quantity_list_b_items = NutrientQuantityListItemRecord::load_all_from_sqlite(
        Uuid::from_slice(&nutrient_quantity_list_b_record.get_id()).unwrap(), &pool
    ).await.unwrap();
    let retrieved_nutrient_quantity_list_c_items = NutrientQuantityListItemRecord::load_all_from_sqlite(
        Uuid::from_slice(&nutrient_quantity_list_c_record.get_id()).unwrap(), &pool
    ).await.unwrap();

    assert_eq!(retrieved_nutrient_quantity_list_a_items.len(), nutrient_quantity_list_a_item_record_vec.len());
    assert!(retrieved_nutrient_quantity_list_a_items.iter().all(|item| nutrient_quantity_list_a_item_record_vec.contains(item)));
    assert_eq!(retrieved_nutrient_quantity_list_b_items.len(), nutrient_quantity_list_b_item_record_vec.len());
    assert!(retrieved_nutrient_quantity_list_b_items.iter().all(|item| nutrient_quantity_list_b_item_record_vec.contains(item)));
    assert_eq!(retrieved_nutrient_quantity_list_c_items.len(), nutrient_quantity_list_c_item_record_vec.len());
    assert!(retrieved_nutrient_quantity_list_c_items.iter().all(|item| nutrient_quantity_list_c_item_record_vec.contains(item)));


    // delete from list
    nutrient_quantity_list_b_item_record_vec[0].delete_item_from_sqlite(&pool).await.unwrap();
    let retrieved_nutrient_quantity_list_b_items_2 = NutrientQuantityListItemRecord::load_all_from_sqlite(
        Uuid::from_slice(&nutrient_quantity_list_b_record.get_id()).unwrap(), &pool
    ).await.unwrap();
    assert!(retrieved_nutrient_quantity_list_b_items_2.len() == retrieved_nutrient_quantity_list_b_items.len() - 1);

    refs = retrieved_nutrient_quantity_list_b_items_2.iter().collect();
    NutrientQuantityListItemRecord::delete_item_vec_from_sqlite(refs, &pool).await.unwrap();
    let retrieved_nutrient_list_b_items_3 = NutrientQuantityListItemRecord::load_all_from_sqlite(
        Uuid::from_slice(&nutrient_quantity_list_b_record.get_id()).unwrap(), &pool
    ).await.unwrap();
    assert!(retrieved_nutrient_list_b_items_3.len() == 0);

    // delete from db
    nutrient_quantity_list_a_record.delete_from_database(&pool).await.unwrap();
    nutrient_quantity_list_b_record.delete_from_database(&pool).await.unwrap();
    nutrient_quantity_list_c_record.delete_from_database(&pool).await.unwrap();
}

