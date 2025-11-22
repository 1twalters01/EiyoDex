use nutrients::{nutrient::Nutrient, schema::nutrients::NutrientType, units::NutrientUnit};
use units::{mass::MassUnit, volume::VolumeUnit};
use uuid::Uuid;
use std::collections::HashSet;

#[test]
fn test_nutrient_new_nutrient_unit() {
    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(id, name.clone(), main_unit);
    let mut nutrient_2 = nutrient.clone();
    assert_eq!(nutrient, nutrient_2);
    
    nutrient_2.set_description(description.clone());
    assert_ne!(nutrient, nutrient_2);
    
    nutrient.set_description(description);
    assert_eq!(nutrient, nutrient_2);
}

#[test]
fn test_nutrient_new_rc_refcell_nutrient_unit() {
    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let nutrient = Nutrient::new_rc_refcell(id, name, main_unit);
    let nutrient_2 = nutrient.clone();
    assert_eq!(nutrient, nutrient_2);

    nutrient.borrow_mut().set_description(description);
    assert_eq!(nutrient, nutrient_2);
}

#[test]
fn test_nutrient_unit_id() {
    let id: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128);
    let id2: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2c3c4d1d2d3d4e1e2u128);
    let name = String::from("Potassium");
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(Some(id), name.clone(), main_unit);

    assert_eq!(nutrient.get_id(), id);
    nutrient.set_id(id2);
    assert_eq!(nutrient.get_id(), id2);
}

#[test]
fn test_nutrient_unit_name() {
    let id: Uuid = Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128);
    let name = String::from("Potassium");
    let name2 = String::from("Calcium");
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(Some(id), name.clone(), main_unit);

    assert_eq!(nutrient.get_name(), name);
    nutrient.set_name(name2.clone());
    assert_eq!(nutrient.get_name(), name2);
}

#[test]
fn test_nutrient_unit_description() {
    let id = None;
    let name = String::from("Potassium");
    let description = "Test description".to_string();
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let mut nutrient = Nutrient::new(id, name.clone(), main_unit);

    assert_eq!(nutrient.get_description(), String::new());
    nutrient.set_description(description.clone());
    assert_eq!(nutrient.get_description(), description);
}

#[test]
fn test_nutrient_unit_categories() {
    let id = None;
    let name = String::from("Potassium");
    let main_unit = NutrientUnit::Mass(MassUnit::Milligram);

    let nutrient = Nutrient::new_rc_refcell(id, name.clone(), main_unit);

    assert_eq!(nutrient.borrow().get_categories(), HashSet::new());

    let mineral_category = NutrientType::Mineral;
    nutrient.borrow_mut().insert_category(mineral_category.clone());
    assert_eq!(nutrient.borrow().get_categories(), HashSet::from([mineral_category]));

    nutrient.borrow_mut().remove_category(mineral_category);
    assert_eq!(nutrient.borrow().get_categories(), HashSet::new());

    let other_category = NutrientType::Other;
    nutrient.borrow_mut().extend_category(Vec::from([mineral_category, other_category]));
    assert_eq!(nutrient.borrow().get_categories(), HashSet::from([other_category, mineral_category]));
}

#[test]
fn test_nutrient_unit_main_unit() {
    // let id = None;
    // let name = String::from("Potassium");
    // let main_unit = NutrientUnit::Mass(MassUnit::Milligram);
    // let main_unit_2 = NutrientUnit::Mass(MassUnit::Microgram);
    // let main_unit_3 = NutrientUnit::Volume(VolumeUnit::Pint);
    //
    // let mut nutrient = Nutrient::new(id, name.clone(), main_unit);
    // assert_eq!(nutrient.get_main_unit(), main_unit);
    //
    // let _ = nutrient.set_main_unit(main_unit_2);
    // assert_eq!(nutrient.get_main_unit(), main_unit_2);
    // assert!(nutrient.get_accepted_units().contains(&main_unit));
    // assert!(nutrient.get_accepted_units().contains(&main_unit_2));
    //
    // let mass_enums = MassUnit::get_enumerations();
    // let mass_units: Vec<NutrientUnit> = mass_enums
    //     .iter()
    //     .map(|mass_enum| NutrientUnit::Mass(*mass_enum))
    //     .collect();
    // assert!(mass_units
    //     .iter()
    //     .all(|mass_unit| nutrient.get_accepted_units().contains(mass_unit)));
    //
    // let res = nutrient.set_main_unit(main_unit_3);
    // assert_eq!(res.err(), Some("New main unit not in accepted units"));
    //
    // nutrient.push_to_accepted_units(main_unit_3);
    // let _ = nutrient.set_main_unit(main_unit_3);
    // assert_eq!(nutrient.get_main_unit(), main_unit_3);
    // assert!(nutrient.get_accepted_units().contains(&main_unit));
    // assert!(nutrient.get_accepted_units().contains(&main_unit_2));
    // assert!(nutrient.get_accepted_units().contains(&main_unit_3));
}

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
