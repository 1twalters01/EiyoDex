use identity::{inner_id::InnerIdType, Id};
use nutrients::{
    entity::Entity, nutrient::{ link_parent_child, Nutrient }, nutrient_list::NutrientList, nutrient_quantity::NutrientQuantity, nutrient_units::NutrientUnit, records::{
        nutrient_list_record::{NutrientListItemRecord, NutrientListRecord}, nutrient_quantity_list_record::NutrientQuantityListRecord, nutrient_record::NutrientRecord, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient_quantity_list() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let mut nutrient_quantity_list = NutrientList::new();
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


    // Create nutrient quantities
    let value = 12.5;
    let value_2 = 1f64;
    let iron_quantity = NutrientQuantity::from_rc_refcell(value, iron, NutrientUnit::Mass(MassUnit::Gram));
    let iron_quantity_2 = NutrientQuantity::from_rc_refcell(value_2, iron, NutrientUnit::Mass(MassUnit::Gram));
    let heme_iron_quantity = NutrientQuantity::from_rc_refcell(value, heme_iron, NutrientUnit::Mass(MassUnit::Gram));
    let non_heme_iron_quantity = NutrientQuantity::from_rc_refcell(value, non_heme_iron, NutrientUnit::Mass(MassUnit::Gram));
    let non_heme_iron_a_quantity = NutrientQuantity::from_rc_refcell(value, non_heme_iron_a, NutrientUnit::Mass(MassUnit::Gram));
    let non_heme_iron_b_quantity = NutrientQuantity::from_rc_refcell(value, non_heme_iron_b, NutrientUnit::Mass(MassUnit::Gram));


    // Add nutrient quantities to nutrient quantity list
    nutrient_quantity_list.push(iron_quantity);
    nutrient_quantity_list.push(heme_iron_quantity);
    nutrient_quantity_list.push(non_heme_iron_quantity);
    nutrient_quantity_list.push(non_heme_iron_a_quantity);
    nutrient_quantity_list.push(non_heme_iron_b_quantity);


    // Create nutrient quantity list record
    let nutrient_quantity_list_record = NutrientQuantityListRecord::from_nutrient_quantity_list(nutrient_quantity_list.clone());
    let manual_list_record = NutrientQuantityListRecord::from_value(nutrient_quantity_list.get_id().as_bytes().to_vec(), nutrient_quantity_list.get_name(), nutrient_quantity_list.get_description());

    assert_eq!(nutrient_quantity_list_record, manual_list_record);
}

#[tokio::test]
async fn test_to_nutrient_quantity_list() {
}

#[tokio::test]
async fn test_database_operations() {
}

#[tokio::test]
async fn test_load_from_database() {
}

