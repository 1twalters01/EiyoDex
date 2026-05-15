use identity::{inner_id::InnerIdType, Id};
use nutrients::{entity::Entity, nutrient::{link_parent_child, Nutrient}, nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList, nutrient_units::NutrientUnit, records::{nutrient_quantity_list_record::NutrientQuantityListItemRecord, nutrient_quantity_record::NutrientQuantityRecord, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
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
}

#[tokio::test]
async fn test_database_operations() {
}
