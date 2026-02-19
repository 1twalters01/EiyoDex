use std::{collections::BTreeSet, str::FromStr};
use units::{
    duration_unit::DurationUnit, energy_unit::EnergyUnit, measurement_system::MeasurementSystem,
    power_measurement_system::PowerMeasurementSystem, power_unit::PowerUnit,
};

#[test]
fn test_from_variants() {
    let kilocalorie = EnergyUnit::Kilocalorie;
    let kilojoule = EnergyUnit::Kilojoule;

    let second = DurationUnit::Second;
    let minute = DurationUnit::Minute;
    let hour = DurationUnit::Hour;
    let day = DurationUnit::Day;
    let week = DurationUnit::Week;

    assert_eq!(
        PowerUnit::from_variants(kilocalorie, second),
        PowerUnit::KilocaloriePerSecond,
    );
    assert_eq!(
        PowerUnit::from_variants(kilocalorie, minute),
        PowerUnit::KilocaloriePerMinute,
    );
    assert_eq!(
        PowerUnit::from_variants(kilocalorie, hour),
        PowerUnit::KilocaloriePerHour,
    );
    assert_eq!(
        PowerUnit::from_variants(kilocalorie, day),
        PowerUnit::KilocaloriePerDay,
    );
    assert_eq!(
        PowerUnit::from_variants(kilocalorie, week),
        PowerUnit::KilocaloriePerWeek,
    );

    assert_eq!(
        PowerUnit::from_variants(kilojoule, second),
        PowerUnit::KilojoulePerSecond,
    );
    assert_eq!(
        PowerUnit::from_variants(kilojoule, minute),
        PowerUnit::KilojoulePerMinute,
    );
    assert_eq!(
        PowerUnit::from_variants(kilojoule, hour),
        PowerUnit::KilojoulePerHour,
    );
    assert_eq!(
        PowerUnit::from_variants(kilojoule, day),
        PowerUnit::KilojoulePerDay,
    );
    assert_eq!(
        PowerUnit::from_variants(kilojoule, week),
        PowerUnit::KilojoulePerWeek,
    );
}

#[test]
fn test_get_all_density_unit_enumerations() {
    let function_enumerations = PowerUnit::get_all_enumerations();
    let manual_enumerations = &vec![
        PowerUnit::KilocaloriePerSecond,
        PowerUnit::KilocaloriePerMinute,
        PowerUnit::KilocaloriePerHour,
        PowerUnit::KilocaloriePerDay,
        PowerUnit::KilocaloriePerWeek,
        PowerUnit::KilojoulePerSecond,
        PowerUnit::KilojoulePerMinute,
        PowerUnit::KilojoulePerHour,
        PowerUnit::KilojoulePerDay,
        PowerUnit::KilojoulePerWeek,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_selected_power_unit_enumerations() {
    let function_enumerations = PowerUnit::get_selected_enumerations();
    let manual_enumerations = &vec![PowerUnit::KilocaloriePerHour];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(PowerUnit::KilocaloriePerSecond.as_symbol(), "kcal/s");
    assert_eq!(PowerUnit::KilocaloriePerMinute.as_symbol(), "kcal/min");
    assert_eq!(PowerUnit::KilocaloriePerHour.as_symbol(), "kcal/hr");
    assert_eq!(PowerUnit::KilocaloriePerDay.as_symbol(), "kcal/day");
    assert_eq!(PowerUnit::KilocaloriePerWeek.as_symbol(), "kcal/week");

    assert_eq!(PowerUnit::KilojoulePerSecond.as_symbol(), "kj/s");
    assert_eq!(PowerUnit::KilojoulePerMinute.as_symbol(), "kj/min");
    assert_eq!(PowerUnit::KilojoulePerHour.as_symbol(), "kj/hr");
    assert_eq!(PowerUnit::KilojoulePerDay.as_symbol(), "kj/day");
    assert_eq!(PowerUnit::KilojoulePerWeek.as_symbol(), "kj/week");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(
        PowerUnit::KilocaloriePerSecond.as_unit_type(),
        "kilocalorie per second"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerMinute.as_unit_type(),
        "kilocalorie per minute"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerHour.as_unit_type(),
        "kilocalorie per hour"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerDay.as_unit_type(),
        "kilocalorie per day"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerWeek.as_unit_type(),
        "kilocalorie per week"
    );

    assert_eq!(
        PowerUnit::KilojoulePerSecond.as_unit_type(),
        "kilojoule per second"
    );
    assert_eq!(
        PowerUnit::KilojoulePerMinute.as_unit_type(),
        "kilojoule per minute"
    );
    assert_eq!(
        PowerUnit::KilojoulePerHour.as_unit_type(),
        "kilojoule per hour"
    );
    assert_eq!(
        PowerUnit::KilojoulePerDay.as_unit_type(),
        "kilojoule per day"
    );
    assert_eq!(
        PowerUnit::KilojoulePerWeek.as_unit_type(),
        "kilojoule per week"
    );
}

#[test]
fn test_get_unit_types_plural() {
    assert_eq!(
        PowerUnit::KilocaloriePerSecond.as_unit_type_plural(),
        "kilocalories per second"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerMinute.as_unit_type_plural(),
        "kilocalories per minute"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerHour.as_unit_type_plural(),
        "kilocalories per hour"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerDay.as_unit_type_plural(),
        "kilocalories per day"
    );
    assert_eq!(
        PowerUnit::KilocaloriePerWeek.as_unit_type_plural(),
        "kilocalories per week"
    );

    assert_eq!(
        PowerUnit::KilojoulePerSecond.as_unit_type_plural(),
        "kilojoules per second"
    );
    assert_eq!(
        PowerUnit::KilojoulePerMinute.as_unit_type_plural(),
        "kilojoules per minute"
    );
    assert_eq!(
        PowerUnit::KilojoulePerHour.as_unit_type_plural(),
        "kilojoules per hour"
    );
    assert_eq!(
        PowerUnit::KilojoulePerDay.as_unit_type_plural(),
        "kilojoules per day"
    );
    assert_eq!(
        PowerUnit::KilojoulePerWeek.as_unit_type_plural(),
        "kilojoules per week"
    );
}

#[test]
fn test_get_measurement_system() {
    let metric_energy_measurement_system = MeasurementSystem::Metric;
    let si_duration_measurement_system = MeasurementSystem::SI;

    assert_eq!(
        PowerUnit::KilocaloriePerSecond.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilocaloriePerMinute.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilocaloriePerHour.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilocaloriePerDay.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilocaloriePerWeek.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );

    assert_eq!(
        PowerUnit::KilojoulePerSecond.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilojoulePerMinute.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilojoulePerHour.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilojoulePerDay.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        PowerUnit::KilojoulePerWeek.get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
}

#[test]
fn test_get_energy_variant() {
    assert_eq!(
        PowerUnit::KilocaloriePerSecond.get_energy_variant(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        PowerUnit::KilocaloriePerMinute.get_energy_variant(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        PowerUnit::KilocaloriePerHour.get_energy_variant(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        PowerUnit::KilocaloriePerDay.get_energy_variant(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        PowerUnit::KilocaloriePerWeek.get_energy_variant(),
        EnergyUnit::Kilocalorie
    );

    assert_eq!(
        PowerUnit::KilojoulePerSecond.get_energy_variant(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        PowerUnit::KilojoulePerMinute.get_energy_variant(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        PowerUnit::KilojoulePerHour.get_energy_variant(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        PowerUnit::KilojoulePerDay.get_energy_variant(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        PowerUnit::KilojoulePerWeek.get_energy_variant(),
        EnergyUnit::Kilojoule
    );
}

#[test]
fn test_get_duration_variant() {
    assert_eq!(
        PowerUnit::KilocaloriePerSecond.get_duration_variant(),
        DurationUnit::Second
    );
    assert_eq!(
        PowerUnit::KilocaloriePerMinute.get_duration_variant(),
        DurationUnit::Minute
    );
    assert_eq!(
        PowerUnit::KilocaloriePerHour.get_duration_variant(),
        DurationUnit::Hour
    );
    assert_eq!(
        PowerUnit::KilocaloriePerDay.get_duration_variant(),
        DurationUnit::Day
    );
    assert_eq!(
        PowerUnit::KilocaloriePerWeek.get_duration_variant(),
        DurationUnit::Week
    );

    assert_eq!(
        PowerUnit::KilojoulePerSecond.get_duration_variant(),
        DurationUnit::Second
    );
    assert_eq!(
        PowerUnit::KilojoulePerMinute.get_duration_variant(),
        DurationUnit::Minute
    );
    assert_eq!(
        PowerUnit::KilojoulePerHour.get_duration_variant(),
        DurationUnit::Hour
    );
    assert_eq!(
        PowerUnit::KilojoulePerDay.get_duration_variant(),
        DurationUnit::Day
    );
    assert_eq!(
        PowerUnit::KilojoulePerWeek.get_duration_variant(),
        DurationUnit::Week
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(PowerUnit::KilocaloriePerSecond.si_factor(), 4184f64);
    assert_eq!(PowerUnit::KilocaloriePerMinute.si_factor(), 4184f64 / 60f64);
    assert_eq!(PowerUnit::KilocaloriePerHour.si_factor(), 4184f64 / 3600f64);
    assert_eq!(PowerUnit::KilocaloriePerDay.si_factor(), 4184f64 / 86400f64);
    assert_eq!(
        PowerUnit::KilocaloriePerWeek.si_factor(),
        4184f64 / 604800f64
    );

    assert_eq!(PowerUnit::KilojoulePerSecond.si_factor(), 1000f64);
    assert_eq!(PowerUnit::KilojoulePerMinute.si_factor(), 1000f64 / 60f64);
    assert_eq!(PowerUnit::KilojoulePerHour.si_factor(), 1000f64 / 3600f64);
    assert_eq!(PowerUnit::KilojoulePerDay.si_factor(), 1000f64 / 86400f64);
    assert_eq!(PowerUnit::KilojoulePerWeek.si_factor(), 1000f64 / 604800f64);
}

#[test]
fn test_from_str() {
    assert_eq!(
        PowerUnit::from_str("kilocalorie per day").unwrap(),
        PowerUnit::KilocaloriePerDay
    );
    assert_eq!(
        PowerUnit::from_str("kilojoules per hour").unwrap(),
        PowerUnit::KilojoulePerHour
    );
    assert_eq!(
        PowerUnit::from_str("kj/s").unwrap(),
        PowerUnit::KilojoulePerSecond
    );
}
