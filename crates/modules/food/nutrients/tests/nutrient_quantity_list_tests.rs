use std::rc::Rc;

use nutrients::{
    nutrient::{link_parent_child, Nutrient}, nutrient_list::NutrientList, nutrient_quantity::NutrientQuantity, nutrient_quantity_list::NutrientQuantityList, nutrient_units::NutrientUnit, records::{nutrient_quantity_record::NutrientQuantityRecord, nutrient_record::{NutrientLinkRecord, NutrientRecord}, nutrient_unit_record::NutrientUnitRecord}, schema::{
        nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
        nutrient_type::NutrientType,
    }
};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;
use uuid::Uuid;

#[tokio::test]
async fn test_id_funcs() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let nutrient_list_id = Uuid::from_u128(15u128);
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    let value = 15f64;

    let iron = Nutrient::new_rc_refcell(
        String::from("Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let potassium = Nutrient::new_rc_refcell(
        String::from("Potassium"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let main_nutrient_list: NutrientList = NutrientList::from_vec([iron, potassium].to_vec());

    let iron_record = NutrientQuantityRecord::from_nutrient_quantity(iron_quantity, &pool).await.unwrap();
    let potassium_record = NutrientQuantityRecord::from_nutrient_quantity(potassium_quantity, &pool).await.unwrap();
    iron_record.save_to_database(&pool);
    potassium_record.save_to_database(&pool);
    let iron_entity = iron_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();
    let potassium_entity = potassium_record.to_nutrient_quantity_entity(&pool, main_nutrient_list).await.unwrap();

    let mut nutrient_amount_list = NutrientQuantityList::from_vec(Vec::from([iron_entity, potassium_entity]));

    assert_ne!(nutrient_amount_list.get_id(), nutrient_list_id);
    nutrient_amount_list.set_id(nutrient_list_id);
    assert_eq!(nutrient_amount_list.get_id(), nutrient_list_id);
}

#[tokio::test]
async fn test_push_and_remove_from_nutrient_list() {
    let pool = DatabaseService::new().await.unwrap().get_pool();
    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let _ = VolumeUnit::save_enumerations_to_database(&pool).await;
    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;
    let _ = NutrientUnitRecord::save_enumerations_to_database(&pool).await;

    let nutrient_list_id = Uuid::from_u128(15u128);
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };

    let value = 15f64;

    let iron = Nutrient::new_rc_refcell(
        String::from("Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let potassium = Nutrient::new_rc_refcell(
        String::from("Potassium"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let potassium_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let main_nutrient_list: NutrientList = NutrientList::from_vec([iron, potassium].to_vec());

    let iron_record = NutrientQuantityRecord::from_nutrient_quantity(iron_quantity, &pool).await.unwrap();
    let potassium_record = NutrientQuantityRecord::from_nutrient_quantity(potassium_quantity, &pool).await.unwrap();
    iron_record.save_to_database(&pool);
    potassium_record.save_to_database(&pool);
    let iron_entity = iron_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();
    let potassium_entity = potassium_record.to_nutrient_quantity_entity(&pool, main_nutrient_list).await.unwrap();

    let mut nutrient_amount_list_iron = NutrientQuantityList::from_vec(Vec::from([iron_entity.clone()]));
    nutrient_amount_list_iron.set_id(nutrient_list_id);
    let mut nutrient_amount_list_potassium =
        NutrientQuantityList::from_vec(Vec::from([potassium_entity.clone()]));
    nutrient_amount_list_potassium.set_id(nutrient_list_id);
    let mut nutrient_amount_list_iron_and_potassium =
        NutrientQuantityList::from_vec(Vec::from([iron_entity.clone(), potassium_entity.clone()]));
    nutrient_amount_list_iron_and_potassium.set_id(nutrient_list_id);
    let mut nutrient_amount_list_potassium_and_iron =
        NutrientQuantityList::from_vec(Vec::from([potassium_entity.clone(), iron_entity.clone()]));
    nutrient_amount_list_potassium_and_iron.set_id(nutrient_list_id);

    let mut nutrient_amount_list = NutrientQuantityList::from_vec(Vec::from([iron_entity.clone()]));
    nutrient_amount_list.set_id(nutrient_list_id);

    nutrient_amount_list.push(iron_entity.clone());
    assert_eq!(nutrient_amount_list, nutrient_amount_list_iron);

    nutrient_amount_list.push(potassium_entity);
    assert_eq!(
        nutrient_amount_list,
        nutrient_amount_list_iron_and_potassium
    );
    assert_eq!(
        nutrient_amount_list,
        nutrient_amount_list_potassium_and_iron
    );

    nutrient_amount_list.remove(&iron_entity);
    assert_eq!(nutrient_amount_list, nutrient_amount_list_potassium);
}

#[test]
fn test_sum_ascendants_vec() {}

#[tokio::test]
async fn test_sum_descendants_vec() {
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

    // Create iron, heme iron and non-heme iron
    let iron = Nutrient::new_rc_refcell(
        String::from("Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let heme_iron = Nutrient::new_rc_refcell(
        String::from("Heme Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron = Nutrient::new_rc_refcell(
        String::from("Non-heme Iron"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_a = Nutrient::new_rc_refcell(
        String::from("Non-heme Iron A"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_b = Nutrient::new_rc_refcell(
        String::from("Non-heme Iron B"),
        nutrient_type.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let potassium = Nutrient::new_rc_refcell(
        String::from("Potassium"),
        nutrient_type,
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let main_nutrient_list: NutrientList = NutrientList::from_vec([
        iron.clone(),
        heme_iron.clone(),
        non_heme_iron.clone(),
        non_heme_iron_a.clone(),
        non_heme_iron_b.clone(),
        potassium.clone(),
    ].to_vec());

    // Link parents and children
    let _ = link_parent_child(&iron, &heme_iron);
    let _ = link_parent_child(&iron, &non_heme_iron);
    let _ = link_parent_child(&non_heme_iron, &non_heme_iron_a);
    let _ = link_parent_child(&non_heme_iron, &non_heme_iron_b);

    let iron_record = NutrientRecord::from_nutrient_rc_refcell(iron.clone(), &pool).await.unwrap();
    let heme_iron_record = NutrientRecord::from_nutrient_rc_refcell(heme_iron.clone(), &pool).await.unwrap();
    let non_heme_iron_record = NutrientRecord::from_nutrient_rc_refcell(non_heme_iron.clone(), &pool).await.unwrap();
    let non_heme_iron_a_record = NutrientRecord::from_nutrient_rc_refcell(non_heme_iron_a.clone(), &pool).await.unwrap();
    let non_heme_iron_b_record = NutrientRecord::from_nutrient_rc_refcell(non_heme_iron_b.clone(), &pool).await.unwrap();
    let potassium_record = NutrientRecord::from_nutrient_rc_refcell(potassium.clone(), &pool).await.unwrap();

    iron_record.save_to_database(&pool).await.unwrap();
    heme_iron_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_a_record.save_to_database(&pool).await.unwrap();
    non_heme_iron_b_record.save_to_database(&pool).await.unwrap();
    potassium_record.save_to_database(&pool).await.unwrap();

    let iron_link_record = NutrientLinkRecord::from_nutrient_rc_refcell(iron.clone(), &pool).await.unwrap();
    let heme_iron_link_record = NutrientLinkRecord::from_nutrient_rc_refcell(heme_iron.clone(), &pool).await.unwrap();
    let non_heme_iron_link_record = NutrientLinkRecord::from_nutrient_rc_refcell(non_heme_iron.clone(), &pool).await.unwrap();
    let non_heme_iron_a_link_record = NutrientLinkRecord::from_nutrient_rc_refcell(non_heme_iron_a.clone(), &pool).await.unwrap();
    let non_heme_iron_b_link_record = NutrientLinkRecord::from_nutrient_rc_refcell(non_heme_iron_b.clone(), &pool).await.unwrap();
    let potassium_link_record = NutrientLinkRecord::from_nutrient_rc_refcell(potassium.clone(), &pool).await.unwrap();

    let value_1 = 1f64;
    let iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value_1,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let iron_record = NutrientQuantityRecord::from_nutrient_quantity(iron_quantity, &pool).await.unwrap();
    iron_record.save_to_database(&pool);
    let iron_entity = iron_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();

    let value_2 = 10f64;
    let heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value_2,
        heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let heme_iron_record = NutrientQuantityRecord::from_nutrient_quantity(heme_iron_quantity, &pool).await.unwrap();
    heme_iron_record.save_to_database(&pool);
    let heme_iron_entity = heme_iron_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();

    let value_3 = 100f64;
    let non_heme_iron_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value_3,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let non_heme_iron_record = NutrientQuantityRecord::from_nutrient_quantity(non_heme_iron_quantity, &pool).await.unwrap();
    non_heme_iron_record.save_to_database(&pool);
    let non_heme_iron_entity = non_heme_iron_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();

    let value_4 = 1_000f64;
    let non_heme_iron_a_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value_4,
        non_heme_iron_a.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let non_heme_iron_a_record = NutrientQuantityRecord::from_nutrient_quantity(non_heme_iron_a_quantity, &pool).await.unwrap();
    non_heme_iron_a_record.save_to_database(&pool);
    let non_heme_iron_a_entity = non_heme_iron_a_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();

    let value_5 = 10_000f64;
    let non_heme_iron_b_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value_5,
        non_heme_iron_b.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let non_heme_iron_b_record = NutrientQuantityRecord::from_nutrient_quantity(non_heme_iron_b_quantity, &pool).await.unwrap();
    non_heme_iron_b_record.save_to_database(&pool);
    let non_heme_iron_b_entity = non_heme_iron_b_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();

    let value_6 = 100_000f64;
    let potassium_quantity: NutrientQuantity = NutrientQuantity::from_rc_refcell(
        value_6,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();
    let potassium_record = NutrientQuantityRecord::from_nutrient_quantity(potassium_quantity, &pool).await.unwrap();
    potassium_record.save_to_database(&pool);
    let potassium_entity = potassium_record.to_nutrient_quantity_entity(&pool, main_nutrient_list.clone()).await.unwrap();


    let mineral_vec = Vec::from([
        iron_entity.clone(),
        heme_iron_entity,
        non_heme_iron_entity,
        non_heme_iron_a_entity,
        non_heme_iron_b_entity,
        potassium_entity,
    ]);
    println!("{:#?}", iron_entity.clone().get_inner().get_nutrient());
    println!("{:#?}", iron.clone());
    assert!(Rc::ptr_eq(&iron, &main_nutrient_list.get_nutrients()[0]));
    assert!(Rc::ptr_eq(&iron, &iron_entity.get_inner().get_nutrient()));

    let minerals = NutrientQuantityList::from_vec(mineral_vec);
    // println!("{:#?}", minerals);
    println!("{:#?}", minerals.get_nutrient_names());

    let iron_sum = minerals.sum_amounts_from_descendants_rc_refcell(iron);
    println!("iron_sum: {:#?}", iron_sum);
    println!("manual sum: {:#?}", value_2 + value_3 + value_4 + value_5);
    assert_eq!(iron_sum.get_value(), value_2 + value_3 + value_4 + value_5);
}

