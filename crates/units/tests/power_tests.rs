use units::{
    measurement_system::MeasurementSystem,
    power::{Power, PowerUnit},
};

#[test]
fn test_power_from_variants() {
    let kilocalorie = EnergyUnit::Kilocalorie;
    let kilojoule = EnergyUnit::Kilojoule;

    let second = DurationUnit::Second;
    let minute = DurationUnit::Minute;
    let hour = DurationUnit::Hour;
    let day = DurationUnit::Day;
    let week = DurationUnit::Week;

    assert_eq!(
        Power::from_variants(value, kilocalorie, second),
        Power::new(value, PowerUnit::KilocaloriePerSecond);
    );
    assert_eq!(
        Power::from_variants(kilocalorie, minute),
        Power::new(value, PowerUnit::KilocaloriePerMinute);
    );
    assert_eq!(
        Power::from_variants(kilocalorie, hour),
        Power::new(value, PowerUnit::KilocaloriePerHour);
    );
    assert_eq!(
        Power::from_variants(kilocalorie, day),
        Power::new(value, PowerUnit::KilocaloriePerDay);
    );
    assert_eq!(
        Power::from_variants(kilocalorie, week),
        Power::new(value, PowerUnit::KilocaloriePerWeek);
    );

    assert_eq!(
        Power::from_variants(kilojoule, second),
        Power::new(value, PowerUnit::KilojoulePerSecond);
    );
    assert_eq!(
        Power::from_variants(kilojoule, minute),
        Power::new(value, PowerUnit::KilojoulePerMinute);
    );
    assert_eq!(
        Power::from_variants(kilojoule, hour),
        Power::new(value, PowerUnit::KilojoulePerHour);
    );
    assert_eq!(
        Power::from_variants(kilojoule, day),
        Power::new(value, PowerUnit::KilojoulePerDay);
    );
    assert_eq!(
        Power::from_variants(kilojoule, week),
        Power::new(value, PowerUnit::KilojoulePerWeek);
    );
}

#[test]
fn test_power_new() {
    let value = 10 as f64;
    
    let power_new_kj_per_min = Power::new(value, PowerUnit::KilojoulePerMinute);
    let power_from_kj_per_min = Power::from_kj_per_min(value);
    assert_eq!(power_new_kj_per_min, power_from_kj_per_min);

    let power_new_kcal_per_hr = Power::new(value, PowerUnit::KilocaloriePerHour);
    let power_from_kcal_per_hr = Power::from_kcal_per_hr(value);
    assert_eq!(power_new_kcal_per_hr, power_from_kcal_per_hr);
}

#[test]
fn test_power_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20472986;

    let mut power_new = Density::new(value, DensityUnit::KilocaloriePerMinute);
    let power_rounded = density_new.round(5);
    let power_manual = Density::new(5.68033, DensityUnit::KilocaloriePerMinute);
    assert_eq!(power_rounded, power_manual);

    let mut power_new_2 = Density::new(value_2, PowerUnit::KilojoulePerDay);
    let power_rounded_2 = power_new_2.round(5);
    let power_coded_2 = Density::new(147.20473, PowerUnit::KilojoulePerDay);
    assert_eq!(power_rounded_2, power_coded_2);
}

#[test]
fn test_power_as_fn() {}

#[test]
fn test_power_to_unit() {
    let value = 6.9;
    let new_value = value / 1e6f64;

    let power_kj_per_day = Power::from_kj_per_day(value);
    let power_kcal_per_hr = Power::from_kcal_per_hr(new_value);
    let power_kj_per_day_to_kg_per_ml = power_kj_per_day.to_unit(PowerUnit::KilocaloriePerHour);
    let power_kj_per_day_to_kg_per_ml = power_kj_per_day.to_kcal_per_hr();
    assert_eq!(power_kcal_per_hr, power_kj_per_day_to_kcal_per_hr);
}

#[test]
fn test_power_to_fn() {
    let value = 6.9;
    let new_value = value / 4.184;

    let power_kj_per_day = Power::from_kj_per_day(value);
    let power_kcal_per_hr = Power::from_kcal_per_hr(new_value);
    let power_kj_per_day_to_kg_per_ml = power_kj_per_day.to_kcal_per_hr();
    assert_eq!(power_kcal_per_hr, power_kj_per_day_to_kcal_per_hr);
}

#[test]
fn test_power_is_zero() {
    let zero_power = Power::from_kj_per_s(0f64);
    let power = Power::from_kj_per_s(5.5);

    assert!(zero_power.is_zero());
    assert!(!power.is_zero());
}

#[test]
fn test_power_is_negative() {
    let negative_power = Power::from_kj_per_s(-5.5f64);
    let power = Power::from_kj_per_s(5.5);

    assert!(negative_power.is_zero());
    assert!(!power.is_zero());
}

#[test]
fn test_power_get_value() {
    let power = Power::new(6.882, PowerUnit::KilojoulePerHour);
    assert_eq!(power.get_value(), 6.882);
}

#[test]
fn test_power_set_value() {
    let mut power = Power::new(6.882, PowerUnit::KilojoulePerHour);
    assert_eq!(power.get_value(), 6.882);
    power.set_value(8.92);
    assert_eq!(power.get_value(), 8.92);
}

#[test]
fn test_power_get_unit() {
    let power = Power::new(6.882, PowerUnit::KilojoulePerHour);
    assert_eq!(power.get_unit(), PowerUnit::KilojoulePerHour);
}

#[test]
fn test_power_set_unit() {
    let mut power = Power::new(6.882, PowerUnit::KilojoulePerHour);
    assert_eq!(power.get_unit(), PowerUnit::KilojoulePerHour);
    power.set_unit(PowerUnit::KilocaloriesPerDay);
    assert_eq!(power.get_unit(), PowerUnit::KilocaloriesPerDay);
}

#[test]
fn test_power_get_symbol() {}

#[test]
fn test_power_get_measurement_system() {
    let value = 4.86;
    let metric_energy_measurement_system = MeasurementSystem::Metric;
    let si_duration_measurement_system = MeasurementSystem::SI;

    assert_eq!(
        Power::from_kcal_per_s(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kcal_per_min(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kcal_per_hr(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kcal_per_day(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kcal_per_week(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );

    assert_eq!(
        Power::from_kj_per_s(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kj_per_min(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kj_per_hr(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kj_per_day(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
    assert_eq!(
        Power::from_kj_per_week(value).get_measurement_system(),
        PowerMeasurementSystem::new(
            metric_energy_measurement_system,
            si_duration_measurement_system
        )
    );
}

#[test]
fn test_power_get_unit_type() {}

#[test]
fn test_power_get_unit_type_plural() {}

#[test]
fn test_power_to_string() {}

#[test]
fn test_power_add() {}

#[test]
fn test_power_subtract() {}

#[test]
fn test_power_div_f64() {}

#[test]
fn test_power_div_energy_by_duration() {}

#[test]
fn test_power_mul_f64() {}

#[test]
fn test_power_mul_power_by_duration() {}

#[test]
fn test_power_sum() {}

#[test]
fn test_power_partial_order() {}
