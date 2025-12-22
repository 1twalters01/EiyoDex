use nutrients::{nutrient::{link_parent_child, Nutrient}, nutrient_amount::NutrientAmount, units::NutrientUnit};
use units::mass::MassUnit;

#[test]
fn test_nutrient() {
    let id = None;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let calcium = Nutrient::new_rc_refcell(
        id,
        String::from("Calcium"),
        NutrientUnit::Mass(MassUnit::Microgram),
    );

    let value = 5.2134;
    let mut nutrient: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    assert_eq!(nutrient.get_nutrient(), potassium);

    nutrient.set_nutrient_rc_refcell(calcium.clone());
    assert_eq!(nutrient.get_nutrient(), calcium);
}

#[test]
fn test_rounding() {
    let id = None;
    let potassium = Nutrient::new(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value_1 = 5.2134;
    let mut nutrient_1: NutrientAmount = NutrientAmount::new(
        value_1,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 5.21;
    let nutrient_2: NutrientAmount =
        NutrientAmount::new(value_2, potassium, NutrientUnit::Mass(MassUnit::Milligram)).unwrap();

    assert_eq!(nutrient_1.round(2), nutrient_2)
}

#[test]
fn test_multiply() {
    let id = None;
    let potassium = Nutrient::new(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value = 5.2;
    let mut nutrient: NutrientAmount =
        NutrientAmount::new(value, potassium, NutrientUnit::Mass(MassUnit::Milligram)).unwrap();

    nutrient = nutrient * 2f64;
    assert!(nutrient.get_value() == 10.4);
}

#[test]
fn test_division() {
    let id = None;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value = 4.2;
    let mut nutrient: NutrientAmount =
        NutrientAmount::from_rc_refcell(value, potassium, NutrientUnit::Mass(MassUnit::Milligram))
            .unwrap();

    nutrient = nutrient / 2f64;
    assert!(nutrient.get_value() == 2.1);
}

#[test]
fn test_add() {
    let id = None;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value_1 = 5f64;
    let nutrient_1: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 2.3;
    let nutrient_2: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    potassium
        .borrow_mut()
        .set_description("Some description".to_string());
    let value_3 = 7.3;
    let nutrient_3: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        potassium,
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    assert_eq!(nutrient_1 + nutrient_2, nutrient_3);
}

#[test]
fn test_subtract() {
    let id = None;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value_1 = 5.2f64;
    let nutrient_1: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 2.2;
    let nutrient_2: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    potassium
        .borrow_mut()
        .set_description("Some description".to_string());
    let value_3 = 3f64;
    let nutrient_3: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        potassium,
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    assert_eq!(nutrient_1 - nutrient_2, nutrient_3);
}

#[test]
fn test_ordering() {
    let id = None;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value_1 = 5f64;
    let nutrient_1: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 3600f64;
    let nutrient_2: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Microgram),
    )
    .unwrap();

    let value_3 = 2.3;
    let nutrient_3: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    assert!(nutrient_1 > nutrient_2);
    assert!(nutrient_2 > nutrient_3);
}

#[test]
fn test_parent_vec() {
    let id = None;

    // Create iron, heme iron and non-heme iron
    let value_1 = 15f64;
    let iron = Nutrient::new_rc_refcell(
        id,
        String::from("Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_2 = 7.5;
    let heme_iron = Nutrient::new_rc_refcell(
        id,
        String::from("Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let heme_iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_3 = 2.5;
    let non_heme_iron = Nutrient::new_rc_refcell(
        id,
        String::from("Iron"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let non_heme_iron_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        non_heme_iron.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    let value_4 = 50f64;
    let potassium = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );
    let potassium_amount: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_4,
        potassium.clone(),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    // Create a dedecated type (probably a wrapper arround Vec with some functions) as a holder of
    // different nutrient amounts?
    let minerals = Vec::from([iron_amount, heme_iron_amount, non_heme_iron_amount, potassium_amount]);


    // Link parents and children
    link_parent_child(&iron, &heme_iron);
    link_parent_child(&iron, &non_heme_iron);


    // Want to get total amount from iron amount which should be itself's value + value from children


    // Want to get child amount from iron amount which should be the sum of values from children
}
