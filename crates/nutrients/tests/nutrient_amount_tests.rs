use nutrients::{nutrient::Nutrient, nutrient_amount::NutrientAmount, units::NutrientUnit};
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
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Kilogram),
    )
    .unwrap();

    assert_eq!(nutrient.get_nutrient(), Some(potassium));

    nutrient.set_nutrient_rc_refcell(Some(calcium.clone()));
    assert_eq!(nutrient.get_nutrient(), Some(calcium));
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
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 5.21;
    let nutrient_2: NutrientAmount = NutrientAmount::new(
        value_2,
        Some(potassium),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

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
    let mut nutrient: NutrientAmount = NutrientAmount::new(
        value,
        Some(potassium),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

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
    let mut nutrient: NutrientAmount = NutrientAmount::from_rc_refcell(
        value,
        Some(potassium),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
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
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 2.3;
    let nutrient_2: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    potassium
        .borrow_mut()
        .set_description("Some description".to_string());
    let value_3 = 7.3;
    let nutrient_3: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        Some(potassium),
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
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 2.2;
    let nutrient_2: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    potassium
        .borrow_mut()
        .set_description("Some description".to_string());
    let value_3 = 3f64;
    let nutrient_3: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        Some(potassium),
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
    let iron = Nutrient::new_rc_refcell(
        id,
        String::from("Potassium"),
        NutrientUnit::Mass(MassUnit::Milligram),
    );

    let value_1 = 5f64;
    let mut nutrient_1: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_1,
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let value_2 = 3600f64;
    let mut nutrient_2: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_2,
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Microgram),
    )
    .unwrap();

    let value_3 = 2.3;
    let mut nutrient_3: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        Some(potassium.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    let nutrient_4: NutrientAmount = NutrientAmount::from_rc_refcell(
        value_3,
        Some(iron.clone()),
        NutrientUnit::Mass(MassUnit::Milligram),
    )
    .unwrap();

    
    assert!(
        nutrient_1
            .get_value_in(NutrientUnit::Mass(MassUnit::Milligram))
            > nutrient_2.get_value_in(NutrientUnit::Mass(MassUnit::Milligram))
    );

    assert!(
        nutrient_1
            .get_value_in(NutrientUnit::Mass(MassUnit::Milligram))
            > nutrient_3.get_value_in(NutrientUnit::Mass(MassUnit::Milligram))
    );

    assert!(
        nutrient_2
            .get_value_in(NutrientUnit::Mass(MassUnit::Milligram))
            > nutrient_3.get_value_in(NutrientUnit::Mass(MassUnit::Milligram))
    );
}
