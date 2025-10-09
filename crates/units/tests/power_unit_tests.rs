use std::{collections::BTreeSet, str::FromStr};
use units::power::PowerUnit;

#[test]
fn test_get_all_density_unit_enumerations() {
    let function_enumerations = PowerUnit::get_all_enumerations();
    let manual_enumerations = vec![
        &PowerUnit::KilocaloriePerSecond,
        &PowerUnit::KilocaloriePerMinute,
        &PowerUnit::KilocaloriePerHour,
        &PowerUnit::KilocaloriePerDay,
        &PowerUnit::KilocaloriePerWeek,
        &PowerUnit::KilojoulePerSecond,
        &PowerUnit::KilojoulePerMinute,
        &PowerUnit::KilojoulePerHour,
        &PowerUnit::KilojoulePerDay,
        &PowerUnit::KilojoulePerWeek,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_selected_density_unit_enumerations() {
    let function_enumerations = PowerUnit::get_selected_enumerations();
    let manual_enumerations = vec![&PowerUnit::KilocaloriePerHour];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}
