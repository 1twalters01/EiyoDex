use std::{collections::BTreeSet, str::FromStr};
use units::density::DensityUnit;

#[test]
fn test_get_all_density_unit_enumerations() {}

#[test]
fn test_get_selected_density_unit_enumerations() {
    let function_enumerations = DensityUnit::get_selected_enumerations();
    let manual_enumerations = vec![
        &DensityUnit::GramPerMilliliter,
        &DensityUnit::MilligramPerMilliliter,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {}

#[test]
fn test_get_unit_types() {}

#[test]
fn test_get_plural_unit_types() {}

#[test]
fn test_get_measurement_system() {}

#[test]
fn test_get_si_factor() {}

#[test]
fn test_from_str() {}
