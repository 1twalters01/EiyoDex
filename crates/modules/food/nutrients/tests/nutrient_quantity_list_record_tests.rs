use identity::{entity::Entity, inner_id::InnerIdType, Id};
use nutrients::{
    nutrient::{link_parent_child, Nutrient},
    nutrient_quantity::{self, NutrientQuantity},
    nutrient_quantity_list::NutrientQuantityList,
    nutrient_units::NutrientUnit,
    records::{
        nutrient_quantity_list_record::{NutrientQuantityListItemRecord, NutrientQuantityListRecord},
        nutrient_quantity_record::NutrientQuantityRecord,
        nutrient_type_record::NutrientTypeRecord,
        nutrient_unit_record::NutrientUnitRecord
    },
    schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

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


    // Add nutrients to nutrient quantity list
    nutrient_quantity_list.push(iron_quantity_entity);
    nutrient_quantity_list.push(heme_iron_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_a_quantity_entity);
    nutrient_quantity_list.push(non_heme_iron_b_quantity_entity);


    // Create nutrient quantity list entity
    let nutrient_quantity_list_entity = Entity::new(nutrient_quantity_list.clone());


    // Create nutrient list record
    let nutrient_quantity_list_record = NutrientQuantityListRecord::from_nutrient_quantity_list_entity(nutrient_quantity_list_entity.clone());
    let manual_list_record = NutrientQuantityListRecord::from_value(
        nutrient_quantity_list_entity.get_id().to_bytes().to_vec(),
        nutrient_quantity_list.get_name(),
        nutrient_quantity_list.get_description()
    );

    assert_eq!(nutrient_quantity_list_record, manual_list_record);
}

#[tokio::test]
async fn test_to_nutrient_quantity_list() {
    let nutrient_quantity_list = NutrientQuantityList::new();
    let nutrient_quantity_list_entity = Entity::new(nutrient_quantity_list);
    let nutrient_quantity_list_record = NutrientQuantityListRecord::from_nutrient_quantity_list_entity(nutrient_quantity_list_entity.clone());
    let converted_list_entity = nutrient_quantity_list_record.to_nutrient_quantity_list_entity().unwrap();

    assert_eq!(nutrient_quantity_list_entity.get_id(), converted_list_entity.get_id())
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


    // Add nutrient quantities to nutrient quantity list
    nutrient_quantity_list_a.push(iron_quantity_entity.clone());
    nutrient_quantity_list_a.push(heme_iron_quantity_entity.clone());
    nutrient_quantity_list_a.push(non_heme_iron_quantity_entity.clone());
    nutrient_quantity_list_a.push(non_heme_iron_a_quantity_entity.clone());
    nutrient_quantity_list_a.push(non_heme_iron_b_quantity_entity.clone());

    nutrient_quantity_list_b.push(iron_quantity_entity.clone());
    nutrient_quantity_list_b.push(heme_iron_quantity_entity.clone());
    nutrient_quantity_list_b.push(non_heme_iron_quantity_entity.clone());


    // Create nutrient quantity entities
    let nutrient_quantity_list_a_entity = Entity::new(nutrient_quantity_list_a);
    let nutrient_quantity_list_b_entity = Entity::new(nutrient_quantity_list_b);
    let nutrient_quantity_list_c_entity = Entity::new(nutrient_quantity_list_c);


    // Create nutrient quantity list record
    let nutrient_quantity_list_a_record = NutrientQuantityListRecord::from_nutrient_quantity_list_entity(nutrient_quantity_list_a_entity.clone());
    let nutrient_quantity_list_b_record = NutrientQuantityListRecord::from_nutrient_quantity_list_entity(nutrient_quantity_list_b_entity.clone());
    let nutrient_quantity_list_c_record = NutrientQuantityListRecord::from_nutrient_quantity_list_entity(nutrient_quantity_list_c_entity.clone());


    // save db
    nutrient_quantity_list_a_record.save_or_update_to_database(&pool).await.unwrap();
    nutrient_quantity_list_b_record.save_or_update_to_database(&pool).await.unwrap();
    nutrient_quantity_list_c_record.save_or_update_to_database(&pool).await.unwrap();

    let nutrient_quantity_list_a_item_record_vec = NutrientQuantityListItemRecord::from_nutrient_quantity_list_entity(nutrient_quantity_list_a_entity.clone()).await;
    let mut refs: Vec<&NutrientQuantityListItemRecord> = nutrient_quantity_list_a_item_record_vec.iter().collect();
    NutrientQuantityListItemRecord::save_vec_to_database(refs, &pool).await.unwrap();


    // Get all
    let retrieved_nutrient_quantity_list_records = NutrientQuantityListRecord::get_all_from_sqlite(&pool).await.unwrap();
    assert!(retrieved_nutrient_quantity_list_records.iter().any(|item| item == &nutrient_quantity_list_a_record || item == &nutrient_quantity_list_b_record || item == &nutrient_quantity_list_c_record));


    // delete from db
    nutrient_quantity_list_a_record.delete_from_database(&pool).await.unwrap();
    nutrient_quantity_list_b_record.delete_from_database(&pool).await.unwrap();
    nutrient_quantity_list_c_record.delete_from_database(&pool).await.unwrap();
}

