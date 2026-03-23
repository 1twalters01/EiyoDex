use nutrients::{nutrient::Nutrient, nutrient_units::NutrientUnit, records::{nutrient_record::NutrientRecord, nutrient_type_record::NutrientTypeRecord, nutrient_unit_record::NutrientUnitRecord}, schema::{nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType}};
use units::mass::unit::MassUnit;
use utils::database::DatabaseService;

#[tokio::test]
async fn test_from_nutrient() {
    let pool = DatabaseService::new().await.unwrap().get_pool();

    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let nutrient_type = NutrientType {
        chemical_type: ChemicalType::Mineral,
        quantity_type: QuantityType::Micronutrient,
        essentiality_type: Some(EssentialityType::Essential),
    };
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let _ = MassUnit::save_enumerations_to_database(&pool).await;
    let nutrient_unit_record = NutrientUnitRecord::from_nutrient_unit(main_unit, &pool).await;
    let unit_res = nutrient_unit_record.save_to_database(&pool).await;
    assert!(unit_res.is_ok());
    let nutrient_type_record = NutrientTypeRecord::from_nutrient_type(nutrient_type.clone());
    let type_res = nutrient_type_record.save_to_database(&pool).await;
    assert!(type_res.is_ok());
    println!("{:#?}", type_res);
    let essentiality_type_id = nutrient_type_record.get_essentiality_type_id();
    let quantity_type_id = nutrient_type_record.get_quantity_type_id();
    let chemical_id = nutrient_type_record.get_chemical_id_from_database(&pool).await.unwrap();
    let main_unit_id = nutrient_unit_record.get_database_id(&pool).await.unwrap();

    let nutrient = Nutrient::new_rc_refcell(id, name.clone(), nutrient_type, main_unit);
    let nutrient_record = NutrientRecord::from_nutrient(nutrient.borrow().clone(), &pool).await.unwrap();
    let manual_record = NutrientRecord::from_values(
        nutrient.borrow().get_id().as_bytes().to_vec(),
        name,
        description,
        main_unit_id,
        essentiality_type_id,
        quantity_type_id,
        chemical_id
    );

    println!("nutrient_record: {:#?}", nutrient_record);
    println!("manual: {:#?}", manual_record);
    assert_eq!(nutrient_record, manual_record);
}

#[test]
fn test_to_nutrient() {
}

#[test]
fn test_save_to_database() {
}

#[test]
fn test_load_from_database() {
}

#[test]
fn test_delete_from_database() {
}
