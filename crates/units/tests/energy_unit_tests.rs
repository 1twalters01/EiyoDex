use std::{collections::BTreeSet, str::FromStr};
use units::energy::EnergyUnit;

#[test]
fn test_get_energy_unit_enumerations() {
    let function_enumerations = EnergyUnit::get_enumerations();
    let manual_enumerations = vec![&EnergyUnit::Kilojoule, &EnergyUnit::Kilocalorie];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(EnergyUnit::Kilojoule.as_symbol(), "kj");
    assert_eq!(EnergyUnit::Kilocalorie.as_symbol(), "kcal");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(EnergyUnit::Kilojoule.as_unit_type(), "kilojoule");
    assert_eq!(EnergyUnit::Kilocalorie.as_unit_type(), "kilocalorie");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(EnergyUnit::Kilojoule.as_unit_type_plural(), "kilojoules");
    assert_eq!(
        EnergyUnit::Kilocalorie.as_unit_type_plural(),
        "kilocalories"
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(EnergyUnit::Kilojoule.si_factor(), 1000 as f64);
    assert_eq!(EnergyUnit::Kilocalorie.si_factor(), 4184f64);
}

#[test]
fn test_from_str() {
    assert_eq!(EnergyUnit::from_str("kj").unwrap(), EnergyUnit::Kilojoule);
    assert_eq!(
        EnergyUnit::from_str("kilojoule").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        EnergyUnit::from_str("Kilojoule").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        EnergyUnit::from_str("kilojoules").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        EnergyUnit::from_str("KilOJOUleS").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_ne!(EnergyUnit::from_str("kcal").unwrap(), EnergyUnit::Kilojoule);

    assert_eq!(
        EnergyUnit::from_str("kcal").unwrap(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        EnergyUnit::from_str("kilocalorie").unwrap(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        EnergyUnit::from_str("kilocalories").unwrap(),
        EnergyUnit::Kilocalorie
    );
}
