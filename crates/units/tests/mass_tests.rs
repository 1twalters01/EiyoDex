use units::mass::{Mass, MassUnit};

#[test]
fn test_new_mass() {
    let value = 10 as f64;

    let mass_new_grams = Mass::new(value, MassUnit::Gram);
    let mass_from_grams = Mass::from_g(value);
    assert_eq!(mass_new_grams, mass_from_grams);

    let mass_new_milligrams = Mass::new(value, MassUnit::Milligram);
    let mass_from_milligrams = Mass::from_mg(value);
    assert_eq!(mass_new_milligrams, mass_from_milligrams);

    let mass_new_kilograms = Mass::new(value, MassUnit::Kilogram);
    let mass_from_kilograms = Mass::from_kg(value);
    assert_eq!(mass_new_kilograms, mass_from_kilograms);

    let mass_new_micrograms = Mass::new(value, MassUnit::Microgram);
    let mass_from_micrograms = Mass::from_ug(value);
    assert_eq!(mass_new_micrograms, mass_from_micrograms);

    let mass_new_ounces = Mass::new(value, MassUnit::Ounce);
    let mass_from_ounces = Mass::from_oz(value);
    assert_eq!(mass_new_ounces, mass_from_ounces);
}

#[test]
fn test_mass_rounding() {
    let value = 5.6803294822;

    let mut mass_new = Mass::new(value, MassUnit::Gram);
    let mass_rounded = mass_new.round(5);
    let mass_coded = Mass::new(5.68033, MassUnit::Gram);
    assert_eq!(mass_rounded, mass_coded);
}

#[test]
fn test_mass_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let mass_grams = Mass::from_g(value);
    let mass_milligrams = Mass::from_mg(value);
    let mass_kilograms = Mass::from_kg(value);
    let mass_micrograms = Mass::from_ug(value);
    let mass_ounces = Mass::from_oz(value);

    // percentage error calculations
    assert!((mass_grams.as_mg() - value * 1_000 as f64).abs() / mass_grams.as_g() < percentage_err);
    assert!((mass_grams.as_kg() - value * 0.001).abs() / mass_grams.as_kg() < percentage_err);
    assert!((mass_grams.as_ug() - value * 1e6).abs() / mass_grams.as_ug() < percentage_err);
    assert!((mass_grams.as_oz() - value * 0.0352739).abs() / mass_grams.as_oz() < percentage_err);

    assert!(
        (mass_milligrams.as_g() - value * 0.001 as f64).abs() / mass_milligrams.as_g()
            < percentage_err
    );
    assert!(
        (mass_milligrams.as_kg() - value * 1e-6).abs() / mass_milligrams.as_kg() < percentage_err
    );
    assert!(
        (mass_milligrams.as_ug() - value * 1_000 as f64).abs() / mass_milligrams.as_ug()
            < percentage_err
    );
    assert!(
        (mass_milligrams.as_oz() - value * 3.527396e-5).abs() / mass_milligrams.as_oz()
            < percentage_err
    );

    assert!(
        (mass_kilograms.as_g() - value * 1000 as f64).abs() / mass_kilograms.as_g()
            < percentage_err
    );
    assert!((mass_kilograms.as_mg() - value * 1e6).abs() / mass_kilograms.as_mg() < percentage_err);
    assert!((mass_kilograms.as_ug() - value * 1e9).abs() / mass_kilograms.as_ug() < percentage_err);
    assert!(
        (mass_kilograms.as_oz() - value * 35.27396).abs() / mass_kilograms.as_oz() < percentage_err
    );

    assert!(
        (mass_micrograms.as_g() - value * 1e-6).abs() / mass_micrograms.as_g() < percentage_err
    );
    assert!(
        (mass_micrograms.as_mg() - value * 0.001).abs() / mass_micrograms.as_mg() < percentage_err
    );
    assert!(
        (mass_micrograms.as_kg() - value * 1e-9).abs() / mass_micrograms.as_kg() < percentage_err
    );
    assert!(
        (mass_micrograms.as_oz() - value * 3.527396e-8).abs() / mass_micrograms.as_oz()
            < percentage_err
    );

    assert!((mass_ounces.as_g() - value * 28.34952).abs() / mass_ounces.as_g() < percentage_err);
    assert!((mass_ounces.as_mg() - value * 28349.52).abs() / mass_ounces.as_mg() < percentage_err);
    assert!(
        (mass_ounces.as_kg() - value * 0.02834952).abs() / mass_ounces.as_kg() < percentage_err
    );
    assert!(
        (mass_ounces.as_ug() - value * 2.834952e+7).abs() / mass_ounces.as_ug() < percentage_err
    );
}

#[test]
fn test_mass_to_unit() {
    let value = 5.6;
    let new_value = value / 28.34952;

    let mass_grams = Mass::from_g(value);
    let mass_ounces = Mass::from_oz(new_value);
    let mass_grams_to_ounces = mass_grams.to_unit(MassUnit::Ounce);

    print!(
        "mass_ounces1: {},\nmass_ounces2: {}",
        mass_ounces, mass_grams_to_ounces
    );
    assert_eq!(mass_ounces, mass_grams_to_ounces);
}

#[test]
fn test_mass_to_fn() {}

#[test]
fn test_mass_is_zero() {
    let zero_mass = Mass::from_g(0f64);
    let mass = Mass::from_g(5.5);

    assert!(zero_mass.is_zero());
    assert!(!mass.is_zero());
}

#[test]
fn test_mass_is_negative() {
    let negative_mass = Mass::from_g(-5.5);
    let mass = Mass::from_g(5.5);

    assert!(negative_mass.is_negative());
    assert!(!mass.is_negative());
}

#[test]
fn test_mass_get_value() {}

#[test]
fn test_mass_set_value() {}

#[test]
fn test_mass_get_units() {}

#[test]
fn test_mass_set_units() {}

#[test]
fn test_mass_get_symbol() {}

#[test]
fn test_mass_unit_type() {}

#[test]
fn test_mass_unit_type_plural() {}

#[test]
fn test_mass_to_string() {}

#[test]
fn test_mass_display() {}

#[test]
fn test_mass_add() {
    let mass_grams_1 = Mass::from_g(100f64);
    let mass_grams_2 = Mass::from_g(500f64);
    let mass_kilograms = Mass::from_kg(2f64);

    let mass_grams_1_plus_grams_2 = Mass::from_g(600f64);
    let mass_kilograms_plus_grams_1 = Mass::from_kg(2.1);
    let mass_grams_2_plus_kilograms = Mass::from_g(2500f64);

    assert_eq!(mass_grams_1 + mass_grams_2, mass_grams_1_plus_grams_2);
    assert_eq!(mass_kilograms + mass_grams_1, mass_kilograms_plus_grams_1);
    assert_eq!(mass_grams_2 + mass_kilograms, mass_grams_2_plus_kilograms);
}

#[test]
fn test_mass_subtract() {
    let mass_grams_1 = Mass::from_g(6700f64);
    let mass_grams_2 = Mass::from_g(4700f64);
    let mass_kilograms = Mass::from_kg(1.2f64);

    let mass_grams_1_minus_grams_2 = Mass::from_g(2000f64);
    let mass_kilograms_minus_grams_1 = Mass::from_kg(-5.5);
    let mass_grams_2_minus_kilograms = Mass::from_g(3500f64);

    assert_eq!(mass_grams_1 - mass_grams_2, mass_grams_1_minus_grams_2);
    assert_eq!(mass_kilograms - mass_grams_1, mass_kilograms_minus_grams_1);
    assert_eq!(mass_grams_2 - mass_kilograms, mass_grams_2_minus_kilograms);
}

#[test]
fn test_mass_multiply() {
    let mass_grams_1 = Mass::from_g(70f64);
    let mass_grams_2 = Mass::from_g(350f64);
    let mass_grams_3 = Mass::from_g(267.4f64);

    assert_eq!(mass_grams_1 * 5, mass_grams_2);
    assert_eq!(mass_grams_1 * 3.82, mass_grams_3);
}

#[test]
fn test_mass_divide() {}

#[test]
fn test_mass_partial_order() {}
