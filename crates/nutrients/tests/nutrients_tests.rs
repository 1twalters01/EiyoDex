use nutrients::nutrient::{Nutrient, Unit};
use units::mass::MassUnit;
use uuid::Uuid;

#[test]
fn test_new_nutrient() {
    let id = None;
    let id2 = Uuid::new_v4();
    let name = String::from("Potassium");
    let name2 = String::from("Calcium");
    let description = "Test description".to_string();
    let main_unit = Unit::Mass(MassUnit::Milligram);
    let main_unit2 = Unit::Mass(MassUnit::Microgram);

    let mut nutrient = Nutrient::new(id, name.clone(), main_unit);

    nutrient.set_id(id2);
    assert_ne!(Some(nutrient.get_id()), id);
    assert_eq!(nutrient.get_id(), id2);

    nutrient.set_name(name2.clone());
    assert_ne!(nutrient.get_name(), name);
    assert_eq!(nutrient.get_name(), name2);

    assert_eq!(nutrient.get_description(), String::new());
    nutrient.set_description(description.clone());
    assert_eq!(nutrient.get_description(), description);

    assert_eq!(nutrient.get_main_unit(), main_unit);
    let _ = nutrient.set_main_unit(main_unit2);
    assert_eq!(nutrient.get_main_unit(), main_unit2);
    assert!(nutrient.get_accepted_units().contains(&main_unit));
    assert!(nutrient.get_accepted_units().contains(&main_unit2));
}

#[test]
fn test_from_nutrient() {
    let id: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128);
    let name = String::from("Potassium");
    let main_unit = Unit::Mass(MassUnit::Milligram);

    let nutrient = Nutrient::new(Some(id), name.clone(), main_unit);

    assert_eq!(nutrient.get_id(), id);
}

#[test]
fn test_categories() {}

#[test]
fn tst_accepted_units() {}

#[test]
fn test_convert() {}

#[test]
fn test_mutate_conversion() {}

#[test]
fn test_parents() {}

#[test]
fn test_children() {}
