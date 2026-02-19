use units::{mass::Mass, mass_unit::MassUnit, measurement_system::MeasurementSystem};

#[test]
fn test_new_mass() {
    let value = 10 as f64;

    let mass_new_g = Mass::new(value, MassUnit::Gram);
    let mass_from_g = Mass::from_g(value);
    assert_eq!(mass_new_g, mass_from_g);

    let mass_new_mg = Mass::new(value, MassUnit::Milligram);
    let mass_from_mg = Mass::from_mg(value);
    assert_eq!(mass_new_mg, mass_from_mg);

    let mass_new_kg = Mass::new(value, MassUnit::Kilogram);
    let mass_from_kg = Mass::from_kg(value);
    assert_eq!(mass_new_kg, mass_from_kg);

    let mass_new_ug = Mass::new(value, MassUnit::Microgram);
    let mass_from_ug = Mass::from_ug(value);
    assert_eq!(mass_new_ug, mass_from_ug);

    let mass_new_oz = Mass::new(value, MassUnit::Ounce);
    let mass_from_oz = Mass::from_oz(value);
    assert_eq!(mass_new_oz, mass_from_oz);
}

#[test]
fn test_mass_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut mass_new = Mass::new(value, MassUnit::Gram);
    let mass_rounded = mass_new.round(5);
    let mass_coded = Mass::new(5.68033, MassUnit::Gram);
    assert_eq!(mass_rounded, mass_coded);

    let mut mass_new_2 = Mass::new(value_2, MassUnit::Gram);
    let mass_rounded_2 = mass_new_2.round(5);
    let mass_coded_2 = Mass::new(147.20473, MassUnit::Gram);
    assert_eq!(mass_rounded_2, mass_coded_2);
}

#[test]
fn test_mass_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let mass_g = Mass::from_g(value);
    let mass_mg = Mass::from_mg(value);
    let mass_kg = Mass::from_kg(value);
    let mass_ug = Mass::from_ug(value);
    let mass_oz = Mass::from_oz(value);

    // percentage error calculations
    assert!((mass_g.as_mg() - value * 1_000 as f64).abs() / mass_g.as_mg() < percentage_err);
    assert!((mass_g.as_kg() - value * 0.001).abs() / mass_g.as_kg() < percentage_err);
    assert!((mass_g.as_ug() - value * 1e6).abs() / mass_g.as_ug() < percentage_err);
    assert!((mass_g.as_oz() - value * 0.0352739).abs() / mass_g.as_oz() < percentage_err);

    assert!((mass_mg.as_g() - value * 0.001).abs() / mass_mg.as_g() < percentage_err);
    assert!((mass_mg.as_kg() - value * 1e-6).abs() / mass_mg.as_kg() < percentage_err);
    assert!((mass_mg.as_ug() - value * 1_000 as f64).abs() / mass_mg.as_ug() < percentage_err);
    assert!((mass_mg.as_oz() - value * 3.527396e-5).abs() / mass_mg.as_oz() < percentage_err);

    assert!((mass_kg.as_g() - value * 1000 as f64).abs() / mass_kg.as_g() < percentage_err);
    assert!((mass_kg.as_mg() - value * 1e6).abs() / mass_kg.as_mg() < percentage_err);
    assert!((mass_kg.as_ug() - value * 1e9).abs() / mass_kg.as_ug() < percentage_err);
    assert!((mass_kg.as_oz() - value * 35.27396).abs() / mass_kg.as_oz() < percentage_err);

    assert!((mass_ug.as_g() - value * 1e-6).abs() / mass_ug.as_g() < percentage_err);
    assert!((mass_ug.as_mg() - value * 0.001).abs() / mass_ug.as_mg() < percentage_err);
    assert!((mass_ug.as_kg() - value * 1e-9).abs() / mass_ug.as_kg() < percentage_err);
    assert!((mass_ug.as_oz() - value * 3.527396e-8).abs() / mass_ug.as_oz() < percentage_err);

    assert!((mass_oz.as_g() - value * 28.34952).abs() / mass_oz.as_g() < percentage_err);
    assert!((mass_oz.as_mg() - value * 28349.52).abs() / mass_oz.as_mg() < percentage_err);
    assert!((mass_oz.as_kg() - value * 0.02834952).abs() / mass_oz.as_kg() < percentage_err);
    assert!((mass_oz.as_ug() - value * 2.834952e+7).abs() / mass_oz.as_ug() < percentage_err);
}

#[test]
fn test_mass_to_unit() {
    let value = 5.6;
    let new_value = value / 28.34952;

    let mass_g = Mass::from_g(value);
    let mass_oz = Mass::from_oz(new_value);
    let mass_g_to_oz = mass_g.to_unit(MassUnit::Ounce);

    print!("mass_ounces1: {},\nmass_ounces2: {}", mass_oz, mass_g_to_oz);
    assert_eq!(mass_oz, mass_g_to_oz);
}

#[test]
fn test_mass_to_fn() {
    let value = 6.9;
    let new_value = value / 28.34952;

    let mass_g = Mass::from_g(value);
    let mass_oz = Mass::from_oz(new_value);
    let mass_g_to_oz = mass_g.to_oz();

    print!("mass_ounces1: {},\nmass_ounces2: {}", mass_oz, mass_g_to_oz);
    assert_eq!(mass_oz, mass_g_to_oz);
}

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
fn test_mass_get_value() {
    let mass = Mass::new(6.882, MassUnit::Milligram);
    assert_eq!(mass.get_value(), 6.882);
}

#[test]
fn test_mass_set_value() {
    let mut mass = Mass::new(6.882, MassUnit::Milligram);
    mass.set_value(8.92);
    assert_eq!(mass.get_value(), 8.92);
}

#[test]
fn test_mass_get_unit() {
    let mass = Mass::new(6.882, MassUnit::Milligram);
    assert_eq!(mass.get_unit(), MassUnit::Milligram);
}

#[test]
fn test_mass_set_unit() {
    let mut mass = Mass::new(6.882, MassUnit::Milligram);
    mass.set_unit(MassUnit::Ounce);
    assert_eq!(mass.get_unit(), MassUnit::Ounce);
}

#[test]
fn test_mass_get_symbol() {
    let value = 4.2;
    let mass_g = Mass::from_g(value);
    let mass_mg = Mass::from_mg(value);
    let mass_kg = Mass::from_kg(value);
    let mass_ug = Mass::from_ug(value);
    let mass_oz = Mass::from_oz(value);

    assert_eq!(mass_g.get_symbol(), "g");
    assert_eq!(mass_mg.get_symbol(), "mg");
    assert_eq!(mass_kg.get_symbol(), "kg");
    assert_eq!(mass_ug.get_symbol(), "ug");
    assert_eq!(mass_oz.get_symbol(), "oz");
}

#[test]
fn test_mass_get_measurement_system() {
    let value = 4.2;

    let mass_g = Mass::from_g(value);
    let mass_mg = Mass::from_mg(value);
    let mass_kg = Mass::from_kg(value);
    let mass_ug = Mass::from_ug(value);
    let mass_oz = Mass::from_oz(value);

    assert_eq!(mass_g.get_measurement_system(), MeasurementSystem::Metric);
    assert_eq!(mass_mg.get_measurement_system(), MeasurementSystem::Metric);
    assert_eq!(mass_kg.get_measurement_system(), MeasurementSystem::Metric);
    assert_eq!(mass_ug.get_measurement_system(), MeasurementSystem::Metric);
    assert_eq!(
        mass_oz.get_measurement_system(),
        MeasurementSystem::Imperial
    );
}

#[test]
fn test_mass_get_unit_type() {
    let value = 4.2;
    let mass_g = Mass::from_g(value);
    let mass_mg = Mass::from_mg(value);
    let mass_kg = Mass::from_kg(value);
    let mass_ug = Mass::from_ug(value);
    let mass_oz = Mass::from_oz(value);

    assert_eq!(mass_g.get_unit_type(), "gram");
    assert_eq!(mass_mg.get_unit_type(), "milligram");
    assert_eq!(mass_kg.get_unit_type(), "kilogram");
    assert_eq!(mass_ug.get_unit_type(), "microgram");
    assert_eq!(mass_oz.get_unit_type(), "ounce");
}

#[test]
fn test_mass_get_unit_type_plural() {
    let value = 8.52;
    let mass_g = Mass::from_g(value);
    let mass_mg = Mass::from_mg(value);
    let mass_kg = Mass::from_kg(value);
    let mass_ug = Mass::from_ug(value);
    let mass_oz = Mass::from_oz(value);

    assert_eq!(mass_g.get_unit_type_plural(), "grams");
    assert_eq!(mass_mg.get_unit_type_plural(), "milligrams");
    assert_eq!(mass_kg.get_unit_type_plural(), "kilograms");
    assert_eq!(mass_ug.get_unit_type_plural(), "micrograms");
    assert_eq!(mass_oz.get_unit_type_plural(), "ounces");
}

#[test]
fn test_mass_to_string() {
    let value_1 = 5f64;
    let value_2 = 8.642;

    let mass_g_1 = Mass::from_g(value_1);
    assert_eq!(mass_g_1.to_string(), "5g");
    let mass_mg_1 = Mass::from_mg(value_1);
    assert_eq!(mass_mg_1.to_string(), "5mg");
    let mass_kg_1 = Mass::from_kg(value_1);
    assert_eq!(mass_kg_1.to_string(), "5kg");
    let mass_ug_1 = Mass::from_ug(value_1);
    assert_eq!(mass_ug_1.to_string(), "5ug");
    let mass_oz_1 = Mass::from_oz(value_1);
    assert_eq!(mass_oz_1.to_string(), "5oz");

    let mass_g_2 = Mass::from_g(value_2);
    assert_eq!(mass_g_2.to_string(), "8.642g");
    let mass_mg_2 = Mass::from_mg(value_2);
    assert_eq!(mass_mg_2.to_string(), "8.642mg");
    let mass_kg_2 = Mass::from_kg(value_2);
    assert_eq!(mass_kg_2.to_string(), "8.642kg");
    let mass_ug_2 = Mass::from_ug(value_2);
    assert_eq!(mass_ug_2.to_string(), "8.642ug");
    let mass_oz_2 = Mass::from_oz(value_2);
    assert_eq!(mass_oz_2.to_string(), "8.642oz");
}

#[test]
fn test_mass_add() {
    let mass_g_1 = Mass::from_g(100f64);
    let mass_g_2 = Mass::from_g(500f64);
    let mass_kg = Mass::from_kg(2f64);

    let mass_g_1_plus_g_2 = Mass::from_g(600f64);
    let mass_kg_plus_g_1 = Mass::from_kg(2.1);
    let mass_g_2_plus_kg = Mass::from_g(2500f64);

    assert_eq!(mass_g_1 + mass_g_2, mass_g_1_plus_g_2);
    assert_eq!(mass_kg + mass_g_1, mass_kg_plus_g_1);
    assert_eq!(mass_g_2 + mass_kg, mass_g_2_plus_kg);
}

#[test]
fn test_mass_subtract() {
    let mass_g_1 = Mass::from_g(6700f64);
    let mass_g_2 = Mass::from_g(4700f64);
    let mass_kg = Mass::from_kg(1.2f64);

    let mass_g_1_minus_g_2 = Mass::from_g(2000f64);
    let mass_kg_minus_g_1 = Mass::from_kg(-5.5);
    let mass_g_2_minus_kg = Mass::from_g(3500f64);

    assert_eq!(mass_g_1 - mass_g_2, mass_g_1_minus_g_2);
    assert_eq!(mass_kg - mass_g_1, mass_kg_minus_g_1);
    assert_eq!(mass_g_2 - mass_kg, mass_g_2_minus_kg);
}

#[test]
fn test_mass_multiply() {
    let mass_g_1 = Mass::from_g(70f64);
    let mass_g_2 = Mass::from_g(350f64);
    let mass_g_3 = Mass::from_g(267.4f64);

    assert_eq!(mass_g_1 * 5, mass_g_2);
    assert_eq!(mass_g_1 * 3.82, mass_g_3);
}

#[test]
fn test_mass_divide() {
    let mass_g_1 = Mass::from_g(350f64);
    let mass_g_2 = Mass::from_g(70f64);

    assert_eq!(mass_g_1 / 5, mass_g_2);
}

#[test]
fn test_energy_sum() {
    let mass_1 = Mass::from_kg(30f64);
    let mass_2 = Mass::from_kg(20f64);
    let mass_3 = Mass::from_kg(50f64).to_oz();
    let mass_4 = Mass::from_kg(20f64).to_oz();
    let mass_5 = Mass::from_kg(130f64);
    let mass_total = Mass::from_kg(250f64);

    let masses = vec![mass_1, mass_2, mass_3, mass_4, mass_5];

    let sum: Mass = masses.iter().map(|mass| *mass * 2).sum();
    assert_eq!(sum.get_unit(), mass_5.get_unit());
    assert_eq!(sum, (mass_total * 2).to_unit(mass_5.get_unit()));
}

#[test]
fn test_mass_partial_order() {
    let mass_g_1 = Mass::from_g(6700f64);
    let mass_g_2 = Mass::from_g(4700f64);
    let mass_kg = Mass::from_kg(5.2f64);
    assert!(mass_g_1 > mass_g_2);
    assert!(mass_g_1 > mass_kg);
    assert!(mass_kg > mass_g_2);
}
