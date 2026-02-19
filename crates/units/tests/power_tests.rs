use units::{
    duration::DurationWrapper, duration_unit::DurationUnit, energy::Energy,
    energy_unit::EnergyUnit, measurement_system::MeasurementSystem, power::Power,
    power_measurement_system::PowerMeasurementSystem, power_unit::PowerUnit,
};

#[test]
fn test_power_from_variants() {
    let value = 5.6;

    let kilocalorie = EnergyUnit::Kilocalorie;
    let kilojoule = EnergyUnit::Kilojoule;

    let second = DurationUnit::Second;
    let minute = DurationUnit::Minute;
    let hour = DurationUnit::Hour;
    let day = DurationUnit::Day;
    let week = DurationUnit::Week;

    assert_eq!(
        Power::from_variants(value, kilocalorie, second),
        Power::new(value, PowerUnit::KilocaloriePerSecond)
    );
    assert_eq!(
        Power::from_variants(value, kilocalorie, minute),
        Power::new(value, PowerUnit::KilocaloriePerMinute)
    );
    assert_eq!(
        Power::from_variants(value, kilocalorie, hour),
        Power::new(value, PowerUnit::KilocaloriePerHour)
    );
    assert_eq!(
        Power::from_variants(value, kilocalorie, day),
        Power::new(value, PowerUnit::KilocaloriePerDay)
    );
    assert_eq!(
        Power::from_variants(value, kilocalorie, week),
        Power::new(value, PowerUnit::KilocaloriePerWeek)
    );

    assert_eq!(
        Power::from_variants(value, kilojoule, second),
        Power::new(value, PowerUnit::KilojoulePerSecond)
    );
    assert_eq!(
        Power::from_variants(value, kilojoule, minute),
        Power::new(value, PowerUnit::KilojoulePerMinute)
    );
    assert_eq!(
        Power::from_variants(value, kilojoule, hour),
        Power::new(value, PowerUnit::KilojoulePerHour)
    );
    assert_eq!(
        Power::from_variants(value, kilojoule, day),
        Power::new(value, PowerUnit::KilojoulePerDay)
    );
    assert_eq!(
        Power::from_variants(value, kilojoule, week),
        Power::new(value, PowerUnit::KilojoulePerWeek)
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

    let mut power_new = Power::new(value, PowerUnit::KilocaloriePerMinute);
    let power_rounded = power_new.round(5);
    let power_manual = Power::new(5.68033, PowerUnit::KilocaloriePerMinute);
    assert_eq!(power_rounded, power_manual);

    let mut power_new_2 = Power::new(value_2, PowerUnit::KilojoulePerDay);
    let power_rounded_2 = power_new_2.round(5);
    let power_coded_2 = Power::new(147.20473, PowerUnit::KilojoulePerDay);
    assert_eq!(power_rounded_2, power_coded_2);
}

#[test]
fn test_power_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let power_kcal_per_hr = Power::from_kcal_per_hr(value);
    let power_kcal_per_min = Power::from_kcal_per_min(value);
    let power_kj_per_hr = Power::from_kj_per_hr(value);

    // percentage error calculations
    assert!(
        (power_kcal_per_hr.as_kcal_per_min() - value / 60f64).abs()
            / power_kcal_per_hr.as_kcal_per_min()
            < percentage_err
    );
    assert!(
        (power_kcal_per_hr.as_kj_per_hr() - value * 4.184).abs() / power_kcal_per_hr.as_kj_per_hr()
            < percentage_err
    );
    assert!(
        (power_kcal_per_min.as_kj_per_hr() - value * 4.184 * 60f64).abs()
            / power_kcal_per_min.as_kj_per_hr()
            < percentage_err
    );
    assert!(
        (power_kj_per_hr.as_kcal_per_min() - value / (4.184 * 60f64)).abs()
            / power_kj_per_hr.as_kcal_per_min()
            < percentage_err
    );
}

#[test]
fn test_power_to_unit() {
    let value = 6.9;
    let new_value = value / (24f64 * 4.184);

    let power_kj_per_day = Power::from_kj_per_day(value);
    let power_kcal_per_hr = Power::from_kcal_per_hr(new_value).round(8);
    let power_kj_per_day_to_kcal_per_hr = power_kj_per_day
        .to_unit(PowerUnit::KilocaloriePerHour)
        .round(8);
    assert_eq!(power_kcal_per_hr, power_kj_per_day_to_kcal_per_hr);
}

#[test]
fn test_power_to_fn() {
    let value = 6.9;
    let new_value = value / (24f64 * 4.184);

    let power_kj_per_day = Power::from_kj_per_day(value);
    let mut power_kcal_per_hr = Power::from_kcal_per_hr(new_value);
    let mut power_kj_per_day_to_kcal_per_hr = power_kj_per_day.to_kcal_per_hr();
    assert_eq!(
        power_kcal_per_hr.round(8),
        power_kj_per_day_to_kcal_per_hr.round(8)
    );
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

    assert!(negative_power.is_negative());
    assert!(!power.is_negative());
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
    power.set_unit(PowerUnit::KilocaloriePerDay);
    assert_eq!(power.get_unit(), PowerUnit::KilocaloriePerDay);
}

#[test]
fn test_power_get_symbol() {
    let value = 4.86;

    assert_eq!(Power::from_kcal_per_s(value).get_symbol(), "kcal/s");
    assert_eq!(Power::from_kcal_per_min(value).get_symbol(), "kcal/min");
    assert_eq!(Power::from_kcal_per_hr(value).get_symbol(), "kcal/hr");
    assert_eq!(Power::from_kcal_per_day(value).get_symbol(), "kcal/day");
    assert_eq!(Power::from_kcal_per_week(value).get_symbol(), "kcal/week");

    assert_eq!(Power::from_kj_per_s(value).get_symbol(), "kj/s");
    assert_eq!(Power::from_kj_per_min(value).get_symbol(), "kj/min");
    assert_eq!(Power::from_kj_per_hr(value).get_symbol(), "kj/hr");
    assert_eq!(Power::from_kj_per_day(value).get_symbol(), "kj/day");
    assert_eq!(Power::from_kj_per_week(value).get_symbol(), "kj/week");
}

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
fn test_power_get_unit_type() {
    let value = 4.86;

    assert_eq!(
        Power::from_kcal_per_s(value).get_unit_type(),
        "kilocalorie per second"
    );
    assert_eq!(
        Power::from_kcal_per_min(value).get_unit_type(),
        "kilocalorie per minute"
    );
    assert_eq!(
        Power::from_kcal_per_hr(value).get_unit_type(),
        "kilocalorie per hour"
    );
    assert_eq!(
        Power::from_kcal_per_day(value).get_unit_type(),
        "kilocalorie per day"
    );
    assert_eq!(
        Power::from_kcal_per_week(value).get_unit_type(),
        "kilocalorie per week"
    );

    assert_eq!(
        Power::from_kj_per_s(value).get_unit_type(),
        "kilojoule per second"
    );
    assert_eq!(
        Power::from_kj_per_min(value).get_unit_type(),
        "kilojoule per minute"
    );
    assert_eq!(
        Power::from_kj_per_hr(value).get_unit_type(),
        "kilojoule per hour"
    );
    assert_eq!(
        Power::from_kj_per_day(value).get_unit_type(),
        "kilojoule per day"
    );
    assert_eq!(
        Power::from_kj_per_week(value).get_unit_type(),
        "kilojoule per week"
    );
}

#[test]
fn test_power_get_unit_type_plural() {
    let value = 4.86;

    assert_eq!(
        Power::from_kcal_per_s(value).get_unit_type_plural(),
        "kilocalories per second"
    );
    assert_eq!(
        Power::from_kcal_per_min(value).get_unit_type_plural(),
        "kilocalories per minute"
    );
    assert_eq!(
        Power::from_kcal_per_hr(value).get_unit_type_plural(),
        "kilocalories per hour"
    );
    assert_eq!(
        Power::from_kcal_per_day(value).get_unit_type_plural(),
        "kilocalories per day"
    );
    assert_eq!(
        Power::from_kcal_per_week(value).get_unit_type_plural(),
        "kilocalories per week"
    );

    assert_eq!(
        Power::from_kj_per_s(value).get_unit_type_plural(),
        "kilojoules per second"
    );
    assert_eq!(
        Power::from_kj_per_min(value).get_unit_type_plural(),
        "kilojoules per minute"
    );
    assert_eq!(
        Power::from_kj_per_hr(value).get_unit_type_plural(),
        "kilojoules per hour"
    );
    assert_eq!(
        Power::from_kj_per_day(value).get_unit_type_plural(),
        "kilojoules per day"
    );
    assert_eq!(
        Power::from_kj_per_week(value).get_unit_type_plural(),
        "kilojoules per week"
    );
}

#[test]
fn test_power_to_string() {
    let value_1 = 5f64;
    let value_2 = 8.642;

    let power_kcal_per_s = Power::from_kcal_per_s(value_1);
    assert_eq!(power_kcal_per_s.to_string(), "5kcal/s");
    let power_kj_per_week = Power::from_kj_per_week(value_1);
    assert_eq!(power_kj_per_week.to_string(), "5kj/week");

    let power_kcal_per_min = Power::from_kcal_per_min(value_2);
    assert_eq!(power_kcal_per_min.to_string(), "8.642kcal/min");
    let power_kj_per_hr = Power::from_kj_per_hr(value_2);
    assert_eq!(power_kj_per_hr.to_string(), "8.642kj/hr");
}

#[test]
fn test_power_add() {
    let power_1 = Power::from_kcal_per_day(100f64);
    let power_2 = Power::from_kcal_per_week(700f64);
    let power_3 = Power::from_kj_per_day(418.4f64);

    let power_1_plus_2 = Power::from_kcal_per_day(200f64);
    let power_3_plus_1 = Power::from_kj_per_day(836.8);
    let power_2_plus_3 = Power::from_kcal_per_week(1400f64);

    assert_eq!(power_1 + power_2, power_1_plus_2);
    assert_eq!((power_3 + power_1).round(2), power_3_plus_1);
    assert_eq!(power_2 + power_3, power_2_plus_3);
}

#[test]
fn test_power_subtract() {
    let power_1 = Power::from_kcal_per_day(200f64);
    let power_2 = Power::from_kcal_per_week(700f64);
    let power_3 = Power::from_kj_per_day(418.4f64);

    let power_1_minus_2 = Power::from_kcal_per_day(100f64);
    let power_3_minus_1 = Power::from_kj_per_day(-418.4);
    let power_2_minus_3 = Power::from_kcal_per_week(0f64);

    assert_eq!(power_1 - power_2, power_1_minus_2);
    assert_eq!((power_3 - power_1).round(2), power_3_minus_1);
    assert_eq!((power_2 - power_3).round(2), power_2_minus_3);
}

#[test]
fn test_power_div_f64() {
    let power_1 = Power::from_kcal_per_hr(350f64);
    let power_2 = Power::from_kcal_per_hr(70f64);

    assert_eq!(power_1 / 5, power_2);
}

#[test]
fn test_power_div_energy_by_duration() {
    let energy = Energy::from_kj(30.5f64);
    let duration = DurationWrapper::from_s(5f64);
    let power = Power::from_kj_per_s(6.1f64);
    assert_eq!(energy / duration, power);
}

#[test]
fn test_power_mul_f64() {
    let power_1 = Power::from_kcal_per_hr(70f64);
    let power_2 = Power::from_kcal_per_hr(350f64);
    let power_3 = Power::from_kcal_per_hr(267.4f64);

    assert_eq!(power_1 * 5, power_2);
    assert_eq!(power_1 * 3.82, power_3);
}

#[test]
fn test_power_mul_power_by_duration() {
    let power = Power::from_kcal_per_hr(4.2);
    let duration = DurationWrapper::from_day(1f64);
    let energy = Energy::from_kcal(100.8);
    assert_eq!((power * duration).round(2), energy);
    assert_eq!((duration * power).round(2), energy);
}

#[test]
fn test_power_sum() {
    let power_1 = Power::from_kcal_per_hr(30f64);
    let power_2 = Power::from_kcal_per_min(0.33333f64);
    let power_3 = Power::from_kcal_per_hr(50f64).to_kj_per_day();
    let power_4 = Power::from_kcal_per_hr(20f64).to_kj_per_min();
    let power_5 = Power::from_kcal_per_hr(130f64).to_kj_per_s();
    let power_total = Power::from_kcal_per_hr(250f64);

    let powers = vec![power_1, power_2, power_3, power_4, power_5];

    let mut sum: Power = powers.iter().map(|power| *power * 2).sum();
    assert_eq!(sum.get_unit(), power_5.get_unit());
    assert_eq!(
        sum.round(5),
        (power_total * 2).to_unit(power_5.get_unit()).round(5)
    );
}

#[test]
fn test_power_partial_order() {
    let power_1 = Power::from_kcal_per_hr(50f64);
    let power_2 = Power::from_kj_per_day(6000f64);
    let power_3 = Power::from_kcal_per_day(2000f64);
    assert!(power_1 < power_2);
    assert!(power_1 < power_3);
    assert!(power_2 < power_3);
}
