use units::{
    energy::{Energy, EnergyUnit},
    measurement_system::MeasurementSystem,
};

#[test]
fn test_new_energy() {
    let value = 10 as f64;

    let energy_new_kcal = Energy::new(value, EnergyUnit::Kilocalorie);
    let energy_from_kcal = Energy::from_kcal(value);
    assert_eq!(energy_new_kcal, energy_from_kcal);

    let energy_new_kj = Energy::new(value, EnergyUnit::Kilojoule);
    let energy_from_kj = Energy::from_kj(value);
    assert_eq!(energy_new_kj, energy_from_kj);
}

#[test]
fn test_energy_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut mass_new = Energy::new(value, EnergyUnit::Kilocalorie);
    let mass_rounded = mass_new.round(5);
    let mass_coded = Energy::new(5.68033, EnergyUnit::Kilocalorie);
    assert_eq!(mass_rounded, mass_coded);

    let mut mass_new_2 = Energy::new(value_2, EnergyUnit::Kilocalorie);
    let mass_rounded_2 = mass_new_2.round(5);
    let mass_coded_2 = Energy::new(147.20473, EnergyUnit::Kilocalorie);
    assert_eq!(mass_rounded_2, mass_coded_2);
}

#[test]
fn test_energy_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let energy_kcal = Energy::from_kcal(value);
    let energy_kj = Energy::from_kj(value);

    // percentage error calculations
    assert!(
        (energy_kcal.as_kj() - value * 4.184 as f64).abs() / energy_kcal.as_kcal() < percentage_err
    );
    assert!(
        (energy_kj.as_kcal() - value * 0.2390057 as f64).abs() / energy_kj.as_kcal()
            < percentage_err
    );
}

#[test]
fn test_energy_to_unit() {
    let value = 5.6;
    let new_value = value * 4.184;

    let energy_kcal = Energy::from_kcal(value);
    let energy_kj = Energy::from_kj(new_value);
    let energy_kcal_to_kj = energy_kcal.to_unit(EnergyUnit::Kilojoule);

    print!(
        "mass_ounces1: {},\nmass_ounces2: {}",
        energy_kj, energy_kcal_to_kj
    );
    assert_eq!(energy_kj, energy_kcal_to_kj);
}

#[test]
fn test_energy_to_fn() {
    let value = 6.9;
    let new_value = value * 4.184;

    let energy_kcal = Energy::from_kcal(value);
    let energy_kj = Energy::from_kj(new_value);
    let mass_kcal_to_kj = energy_kcal.to_kj();

    print!(
        "mass_ounces1: {},\nmass_ounces2: {}",
        energy_kj, mass_kcal_to_kj
    );
    assert_eq!(energy_kj, mass_kcal_to_kj);
}

#[test]
fn test_energy_is_zero() {
    let zero_mass = Energy::from_kcal(0f64);
    let energy = Energy::from_kcal(5.5);

    assert!(zero_mass.is_zero());
    assert!(!energy.is_zero());
}

#[test]
fn test_energy_is_negative() {
    let negative_mass = Energy::from_kcal(-5.5);
    let energy = Energy::from_kcal(5.5);

    assert!(negative_mass.is_negative());
    assert!(!energy.is_negative());
}

#[test]
fn test_energy_get_value() {
    let energy = Energy::new(6.882, EnergyUnit::Kilojoule);
    assert_eq!(energy.get_value(), 6.882);
}

#[test]
fn test_energy_set_value() {
    let mut energy = Energy::new(6.882, EnergyUnit::Kilojoule);
    energy.set_value(8.92);
    assert_eq!(energy.get_value(), 8.92);
}

#[test]
fn test_energy_get_unit() {
    let energy = Energy::new(6.882, EnergyUnit::Kilojoule);
    assert_eq!(energy.get_unit(), EnergyUnit::Kilojoule);
}

#[test]
fn test_energy_set_unit() {
    let mut energy = Energy::new(6.882, EnergyUnit::Kilojoule);
    energy.set_unit(EnergyUnit::Kilojoule);
    assert_eq!(energy.get_unit(), EnergyUnit::Kilojoule);
}

#[test]
fn test_energy_get_symbol() {
    let value = 4.2;
    let energy_kcal = Energy::from_kcal(value);
    let energy_kj = Energy::from_kj(value);

    assert_eq!(energy_kcal.get_symbol(), "kcal");
    assert_eq!(energy_kj.get_symbol(), "kj");
}

#[test]
fn test_mass_get_measurement_system() {
    let value = 4.2;

    let energy_kcal = Energy::from_kcal(value);
    let energy_kj = Energy::from_kj(value);

    assert_eq!(
        energy_kcal.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        energy_kj.get_measurement_system(),
        MeasurementSystem::Metric
    );
}

#[test]
fn test_energy_unit_type() {
    let value = 4.2;
    let mass_g = Energy::from_kcal(value);
    let mass_mg = Energy::from_kj(value);

    assert_eq!(mass_g.get_unit_type(), "kilocalorie");
    assert_eq!(mass_mg.get_unit_type(), "kilojoule");
}

#[test]
fn test_energy_unit_type_plural() {
    let value = 8.52;
    let mass_g = Energy::from_kcal(value);
    let mass_mg = Energy::from_kj(value);

    assert_eq!(mass_g.get_unit_type_plural(), "kilocalories");
    assert_eq!(mass_mg.get_unit_type_plural(), "kilojoules");
}

#[test]
fn test_energy_to_string() {
    let value_1 = 5f64;
    let value_2 = 8.642;

    let energy_kcal_1 = Energy::from_kcal(value_1);
    assert_eq!(energy_kcal_1.to_string(), "5kcal");
    let energy_kj_1 = Energy::from_kj(value_1);
    assert_eq!(energy_kj_1.to_string(), "5kj");

    let energy_kcal_2 = Energy::from_kcal(value_2);
    assert_eq!(energy_kcal_2.to_string(), "8.642kcal");
    let energy_kj_2 = Energy::from_kj(value_2);
    assert_eq!(energy_kj_2.to_string(), "8.642kj");
}

#[test]
fn test_energy_add() {
    let energy_kcal_1 = Energy::from_kcal(100f64);
    let energy_kcal_2 = Energy::from_kcal(500f64);
    let energy_kj = Energy::from_kj(200f64);

    let energy_kcal_1_plus_kcal_2 = Energy::from_kcal(600f64);
    let energy_kj_plus_kcal_1 = Energy::from_kj(618.4);
    let energy_kcal_2_plus_kj = Energy::from_kcal(547.8011472275334);

    assert_eq!(energy_kcal_1 + energy_kcal_2, energy_kcal_1_plus_kcal_2);
    assert_eq!(energy_kj + energy_kcal_1, energy_kj_plus_kcal_1);
    assert_eq!(energy_kcal_2 + energy_kj, energy_kcal_2_plus_kj);
}

#[test]
fn test_energy_subtract() {
    let energy_kcal_1 = Energy::from_kcal(6f64);
    let energy_kcal_2 = Energy::from_kcal(4f64);
    let energy_kj = Energy::from_kj(1f64);

    let energy_g_1_minus_g_2 = Energy::from_kcal(2f64);
    let energy_kj_minus_kcal_1 = Energy::from_kj(-24.104);
    let energy_kcal_2_minus_kj = Energy::from_kcal(3.7609942638623326);

    assert_eq!(energy_kcal_1 - energy_kcal_2, energy_g_1_minus_g_2);
    assert_eq!(energy_kj - energy_kcal_1, energy_kj_minus_kcal_1);
    assert_eq!(energy_kcal_2 - energy_kj, energy_kcal_2_minus_kj);
}

#[test]
fn test_energy_multiply() {
    let energy_kcal_1 = Energy::from_kcal(70f64);
    let energy_kcal_2 = Energy::from_kcal(350f64);
    let energy_g_3 = Energy::from_kcal(267.4f64);

    assert_eq!(energy_kcal_1 * 5, energy_kcal_2);
    assert_eq!(energy_kcal_1 * 3.82, energy_g_3);
}

#[test]
fn test_energy_divide() {
    let energy_kcal_1 = Energy::from_kcal(350f64);
    let energy_kcal_2 = Energy::from_kcal(70f64);

    assert_eq!(energy_kcal_1 / 5, energy_kcal_2);
}

#[test]
fn test_energy_partial_order() {
    let energy_kcal_1 = Energy::from_kcal(6700f64);
    let energy_kcal_2 = Energy::from_kcal(4700f64);
    let energy_kj = Energy::from_kj(20920f64);
    assert!(energy_kcal_1 > energy_kcal_2);
    assert!(energy_kcal_1 > energy_kj);
    assert!(energy_kj > energy_kcal_2);
}
