use std::{collections::BTreeSet, str::FromStr};
use units::{mass::MassUnit, measurement_system::MeasurementSystem};

#[test]
fn test_get_mass_unit_enumerations() {
    let function_enumerations = MassUnit::get_enumerations();
    let manual_enumerations = vec![
        &MassUnit::Gram,
        &MassUnit::Milligram,
        &MassUnit::Kilogram,
        &MassUnit::Microgram,
        &MassUnit::Ounce,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(MassUnit::Gram.as_symbol(), "g");
    assert_eq!(MassUnit::Milligram.as_symbol(), "mg");
    assert_eq!(MassUnit::Kilogram.as_symbol(), "kg");
    assert_eq!(MassUnit::Microgram.as_symbol(), "ug");
    assert_eq!(MassUnit::Ounce.as_symbol(), "oz");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(MassUnit::Gram.as_unit_type(), "gram");
    assert_eq!(MassUnit::Milligram.as_unit_type(), "milligram");
    assert_eq!(MassUnit::Kilogram.as_unit_type(), "kilogram");
    assert_eq!(MassUnit::Microgram.as_unit_type(), "microgram");
    assert_eq!(MassUnit::Ounce.as_unit_type(), "ounce");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(MassUnit::Gram.as_unit_type_plural(), "grams");
    assert_eq!(MassUnit::Milligram.as_unit_type_plural(), "milligrams");
    assert_eq!(MassUnit::Kilogram.as_unit_type_plural(), "kilograms");
    assert_eq!(MassUnit::Microgram.as_unit_type_plural(), "micrograms");
    assert_eq!(MassUnit::Ounce.as_unit_type_plural(), "ounces");
}

#[test]
fn test_get_measurement_systems() {
    assert_eq!(
        MassUnit::Gram.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        MassUnit::Milligram.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        MassUnit::Kilogram.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        MassUnit::Microgram.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        MassUnit::Ounce.get_measurement_system(),
        MeasurementSystem::Imperial
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(MassUnit::Gram.si_factor(), 0.001 as f64);
    assert_eq!(MassUnit::Milligram.si_factor(), 0.000001);
    assert_eq!(MassUnit::Kilogram.si_factor(), 1 as f64);
    assert_eq!(MassUnit::Microgram.si_factor(), 0.000000001);
    assert_eq!(MassUnit::Ounce.si_factor(), 0.02834952);
}

#[test]
fn test_from_str() {
    assert_eq!(MassUnit::from_str("g").unwrap(), MassUnit::Gram);
    assert_eq!(MassUnit::from_str("gram").unwrap(), MassUnit::Gram);
    assert_eq!(MassUnit::from_str("Gram").unwrap(), MassUnit::Gram);
    assert_eq!(MassUnit::from_str("grams").unwrap(), MassUnit::Gram);
    assert_eq!(MassUnit::from_str("gRAmS").unwrap(), MassUnit::Gram);
    assert_ne!(MassUnit::from_str("mg").unwrap(), MassUnit::Gram);

    assert_eq!(MassUnit::from_str("mg").unwrap(), MassUnit::Milligram);
    assert_eq!(
        MassUnit::from_str("milligram").unwrap(),
        MassUnit::Milligram
    );
    assert_eq!(
        MassUnit::from_str("milligrams").unwrap(),
        MassUnit::Milligram
    );

    assert_eq!(MassUnit::from_str("kg").unwrap(), MassUnit::Kilogram);
    assert_eq!(MassUnit::from_str("kilogram").unwrap(), MassUnit::Kilogram);
    assert_eq!(MassUnit::from_str("kilograms").unwrap(), MassUnit::Kilogram);

    assert_eq!(MassUnit::from_str("ug").unwrap(), MassUnit::Microgram);
    assert_eq!(
        MassUnit::from_str("microgram").unwrap(),
        MassUnit::Microgram
    );
    assert_eq!(
        MassUnit::from_str("micrograms").unwrap(),
        MassUnit::Microgram
    );

    assert_eq!(MassUnit::from_str("oz").unwrap(), MassUnit::Ounce);
    assert_eq!(MassUnit::from_str("ounce").unwrap(), MassUnit::Ounce);
    assert_eq!(MassUnit::from_str("ounces").unwrap(), MassUnit::Ounce);
}
