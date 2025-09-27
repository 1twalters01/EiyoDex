use nutrients::nutrient::{Nutrient, NutrientAmount, Unit};
use units::mass::MassUnit;

#[test]
fn test_new_nutrient_amount() {}

#[test]
fn test_rounding() {}

#[test]
fn test_multiply() {
    let id = None;
    let potassium = Nutrient::new(
        id,
        String::from("Potassium"),
        Unit::Mass(MassUnit::Milligram),
    );

    let value = 5.2;
    let mut nutrient: NutrientAmount =
        NutrientAmount::new(value, potassium, Unit::Mass(MassUnit::Milligram)).unwrap();

    nutrient = nutrient * 2f64;
    assert!(nutrient.get_value() == 10.4);
}

#[test]
fn test_division() {
    let id = None;
    let potassium = Nutrient::new(
        id,
        String::from("Potassium"),
        Unit::Mass(MassUnit::Milligram),
    );

    let value = 4.2;
    let mut nutrient: NutrientAmount =
        NutrientAmount::new(value, potassium, Unit::Mass(MassUnit::Milligram)).unwrap();

    nutrient = nutrient / 2f64;
    assert!(nutrient.get_value() == 2.1);
}

#[test]
fn test_add() {
    let id = None;
    let mut potassium = Nutrient::new(
        id,
        String::from("Potassium"),
        Unit::Mass(MassUnit::Milligram),
    );

    let value_1 = 5f64;
    let nutrient_1: NutrientAmount =
        NutrientAmount::new(value_1, potassium.clone(), Unit::Mass(MassUnit::Milligram)).unwrap();

    let value_2 = 2.3;
    let nutrient_2: NutrientAmount =
        NutrientAmount::new(value_2, potassium.clone(), Unit::Mass(MassUnit::Milligram)).unwrap();

    // potassium.set_description("Potassium is needed for fluid
    // balance".to_string());
    let value_3 = 7.3;
    let nutrient_3: NutrientAmount =
        NutrientAmount::new(value_3, potassium, Unit::Mass(MassUnit::Milligram)).unwrap();

    assert_eq!(nutrient_1 + nutrient_2, nutrient_3);
}

#[test]
fn test_subtract() {}

#[test]
fn test_ordering() {}
