use units::{
    duration::{quantity::DurationQuantity, unit::DurationUnit},
    measurement_system::MeasurementSystem,
    entity::{GetFromDatabaseUsingId, Entity},
};

#[test]
fn test_new_distance() {
    let value = 10 as f64;

    let duration_new_week = DurationQuantity::new(value, DurationUnit::Week);
    let duration_from_week = DurationQuantity::from_week(value);
    assert_eq!(duration_new_week, duration_from_week);

    let duration_new_day = DurationQuantity::new(value, DurationUnit::Day);
    let duration_from_day = DurationQuantity::from_day(value);
    assert_eq!(duration_new_day, duration_from_day);

    let duration_new_hr = DurationQuantity::new(value, DurationUnit::Hour);
    let distance_from_cm = DurationQuantity::from_hr(value);
    assert_eq!(duration_new_hr, distance_from_cm);

    let duration_new_min = DurationQuantity::new(value, DurationUnit::Minute);
    let duration_from_min = DurationQuantity::from_min(value);
    assert_eq!(duration_new_min, duration_from_min);

    let duration_new_s = DurationQuantity::new(value, DurationUnit::Second);
    let duration_from_s = DurationQuantity::from_s(value);
    assert_eq!(duration_new_s, duration_from_s);
}

#[test]
fn test_distance_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let duration_week = DurationQuantity::from_week(value);
    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(value);
    let duration_min = DurationQuantity::from_min(value);
    let duration_s = DurationQuantity::from_s(value);

    // percentage error calculations
    assert!(
        (duration_week.as_day() - value * 7f64).abs() / duration_week.as_day() < percentage_err
    );
    assert!(
        (duration_week.as_hr() - value * 168f64).abs() / duration_week.as_hr() < percentage_err
    );
    assert!(
        (duration_week.as_min() - value * 10_080f64).abs() / duration_week.as_min()
            < percentage_err
    );
    assert!(
        (duration_week.as_s() - value * 604_800f64).abs() / duration_week.as_s() < percentage_err
    );

    assert!(
        (duration_day.as_week() - value / 7f64).abs() / duration_day.as_week() < percentage_err
    );
    assert!((duration_day.as_hr() - value * 24f64).abs() / duration_day.as_hr() < percentage_err);
    assert!(
        (duration_day.as_min() - value * 1440f64).abs() / duration_day.as_min() < percentage_err
    );
    assert!((duration_day.as_s() - value * 86_400f64).abs() / duration_day.as_s() < percentage_err);

    assert!(
        (duration_hr.as_week() - value / 168f64).abs() / duration_hr.as_week() < percentage_err
    );
    assert!((duration_hr.as_day() - value / 24f64).abs() / duration_hr.as_day() < percentage_err);
    assert!((duration_hr.as_min() - value * 60f64).abs() / duration_hr.as_min() < percentage_err);
    assert!((duration_hr.as_s() - value * 3600f64).abs() / duration_hr.as_s() < percentage_err);

    assert!(
        (duration_min.as_week() - value / 10_080f64).abs() / duration_min.as_week()
            < percentage_err
    );
    assert!(
        (duration_min.as_day() - value / 1440f64).abs() / duration_min.as_day() < percentage_err
    );
    assert!((duration_min.as_hr() - value / 60f64).abs() / duration_min.as_hr() < percentage_err);
    assert!((duration_min.as_s() - value * 60f64).abs() / duration_min.as_s() < percentage_err);

    assert!(
        (duration_s.as_week() - value / 604_800f64).abs() / duration_hr.as_week() < percentage_err
    );
    assert!(
        (duration_s.as_day() - value / 86_400f64).abs() / duration_hr.as_day() < percentage_err
    );
    assert!((duration_s.as_hr() - value / 3600f64).abs() / duration_hr.as_hr() < percentage_err);
    assert!((duration_s.as_min() - value / 60f64).abs() / duration_hr.as_min() < percentage_err);
}

#[test]
fn test_distance_to_unit() {
    let value = 5.6;
    let new_value = value * 24f64;
    let new_value_2 = value / 7f64;

    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(new_value);
    let duration_week = DurationQuantity::from_week(new_value_2);
    let distance_day_to_hr = duration_day.to_unit(DurationUnit::Hour);
    let distance_day_to_week = duration_day.to_unit(DurationUnit::Week);

    println!("{}", duration_hr);
    assert_eq!(duration_hr, distance_day_to_hr);
    assert_eq!(duration_week, distance_day_to_week);
}

#[test]
fn test_distance_to_fn() {
    let value = 6.9;
    let new_value = value * 24f64;
    let new_value_2 = value / 7f64;

    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(new_value);
    let duration_week = DurationQuantity::from_week(new_value_2);
    let distance_day_to_hr = duration_day.to_hr();
    let distance_day_to_week = duration_day.to_week();

    assert_eq!(duration_hr, distance_day_to_hr);
    assert_eq!(duration_week, distance_day_to_week);
}

#[test]
fn test_distance_is_zero() {
    let zero_duration = DurationQuantity::from_day(0f64);
    let duration = DurationQuantity::from_day(5.5);

    assert!(zero_duration.is_zero());
    assert!(!duration.is_zero());
}

#[test]
fn test_get_duration() {
    let days = 6.882;
    let duration = DurationQuantity::new(days, DurationUnit::Day);
    let percentage_err = 0.5;

    assert!((duration.get_duration() - days) / days < percentage_err);
}

use chrono::Duration;
use utils::database::DatabaseService;
#[test]
fn test_distance_get_value() {
    let days = 6.882;
    let duration = DurationQuantity::new(days, DurationUnit::Day);
    let nanoseconds = (days * 24f64 * 60f64 * 60f64 * 1e9).round() as i64;
    assert_eq!(duration.get_value(), Duration::nanoseconds(nanoseconds));
}

#[test]
fn test_distance_set_value() {
    let mut duration = DurationQuantity::new(6.882, DurationUnit::Hour);

    let days = 3;
    let new_duration = Duration::days(days);
    duration.set_value(new_duration);
    assert_eq!(duration.get_value(), new_duration);
}

#[test]
fn test_distance_get_unit() {
    let duration = DurationQuantity::new(6.882, DurationUnit::Day);
    assert_eq!(duration.get_unit(), DurationUnit::Day);
}

#[test]
fn test_distance_set_unit() {
    let mut duration = DurationQuantity::new(6.882, DurationUnit::Day);
    duration.set_unit(DurationUnit::Week);
    assert_eq!(duration.get_unit(), DurationUnit::Week);
}

#[test]
fn test_distance_get_symbol() {
    let value = 4.2;
    let duration_week = DurationQuantity::from_week(value);
    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(value);
    let duration_min = DurationQuantity::from_min(value);
    let duration_s = DurationQuantity::from_s(value);

    assert_eq!(duration_week.get_symbol(), "week");
    assert_eq!(duration_day.get_symbol(), "day");
    assert_eq!(duration_hr.get_symbol(), "hr");
    assert_eq!(duration_min.get_symbol(), "min");
    assert_eq!(duration_s.get_symbol(), "s");
}

#[test]
fn test_distance_get_measurement_system() {
    let value = 4.2;

    let duration_week = DurationQuantity::from_week(value);
    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(value);
    let duration_min = DurationQuantity::from_min(value);
    let duration_s = DurationQuantity::from_s(value);

    assert_eq!(
        duration_week.get_measurement_system(),
        MeasurementSystem::SI
    );
    assert_eq!(duration_day.get_measurement_system(), MeasurementSystem::SI);
    assert_eq!(duration_hr.get_measurement_system(), MeasurementSystem::SI);
    assert_eq!(duration_min.get_measurement_system(), MeasurementSystem::SI);
    assert_eq!(duration_s.get_measurement_system(), MeasurementSystem::SI);
}

#[test]
fn test_mass_get_unit_type() {
    let value = 4.2;
    let duration_week = DurationQuantity::from_week(value);
    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(value);
    let duration_min = DurationQuantity::from_min(value);
    let duration_s = DurationQuantity::from_s(value);

    assert_eq!(duration_week.get_unit_type(), "week");
    assert_eq!(duration_day.get_unit_type(), "day");
    assert_eq!(duration_hr.get_unit_type(), "hour");
    assert_eq!(duration_min.get_unit_type(), "minute");
    assert_eq!(duration_s.get_unit_type(), "second");
}

#[test]
fn test_mass_get_unit_type_plural() {
    let value = 8.52;
    let duration_week = DurationQuantity::from_week(value);
    let duration_day = DurationQuantity::from_day(value);
    let duration_hr = DurationQuantity::from_hr(value);
    let duration_min = DurationQuantity::from_min(value);
    let duration_s = DurationQuantity::from_s(value);

    assert_eq!(duration_week.get_unit_type_plural(), "weeks");
    assert_eq!(duration_day.get_unit_type_plural(), "days");
    assert_eq!(duration_hr.get_unit_type_plural(), "hours");
    assert_eq!(duration_min.get_unit_type_plural(), "minutes");
    assert_eq!(duration_s.get_unit_type_plural(), "seconds");
}

#[test]
fn test_mass_to_string() {
    let value_1 = 5f64;
    let value_2 = 1.624;

    let precision_1 = None;
    let distance_week_1 = DurationQuantity::from_week(value_1);
    assert_eq!(distance_week_1.to_string(precision_1), "5 weeks");
    let duration_day_1 = DurationQuantity::from_day(value_1);
    assert_eq!(duration_day_1.to_string(precision_1), "5 days");
    let distance_hour_1 = DurationQuantity::from_hr(value_1);
    assert_eq!(distance_hour_1.to_string(precision_1), "5 hours");
    let distance_minute_1 = DurationQuantity::from_min(value_1);
    assert_eq!(distance_minute_1.to_string(precision_1), "5 minutes");
    let distance_second_1 = DurationQuantity::from_s(value_1);
    assert_eq!(distance_second_1.to_string(precision_1), "5 seconds");

    let precision_2 = Some(3);
    let distance_week_2 = DurationQuantity::from_week(value_2);
    assert_eq!(distance_week_2.to_string(precision_2), "1.624 weeks");
    let duration_day_2 = DurationQuantity::from_day(value_2);
    assert_eq!(duration_day_2.to_string(precision_2), "1.624 days");
    let distance_hour_2 = DurationQuantity::from_hr(value_2);
    assert_eq!(distance_hour_2.to_string(precision_2), "1.624 hours");
    let distance_minute_2 = DurationQuantity::from_min(value_2);
    assert_eq!(distance_minute_2.to_string(precision_2), "1.624 minutes");
    let distance_second_2 = DurationQuantity::from_s(value_2);
    assert_eq!(distance_second_2.to_string(precision_2), "1.624 seconds");
}

#[test]
fn test_mass_add() {
    let duration_day_1 = DurationQuantity::from_day(1f64);
    let duration_day_2 = DurationQuantity::from_day(5f64);
    let duration_hr = DurationQuantity::from_hr(6f64);

    let duration_day_1_plus_day_2 = DurationQuantity::from_day(6f64);
    let duration_hr_plus_day_1 = DurationQuantity::from_hr(30f64);
    let duration_day_2_plus_hr = DurationQuantity::from_day(5.25f64);

    assert_eq!(duration_day_1 + duration_day_2, duration_day_1_plus_day_2);
    assert_eq!(duration_hr + duration_day_1, duration_hr_plus_day_1);
    assert_eq!(duration_day_2 + duration_hr, duration_day_2_plus_hr);
}

#[test]
fn test_mass_subtract() {
    let duration_day_1 = DurationQuantity::from_day(2f64);
    let duration_day_2 = DurationQuantity::from_day(1f64);
    let duration_hr = DurationQuantity::from_hr(12f64);

    let duration_day_1_minus_day_2 = DurationQuantity::from_day(1f64);
    let duration_hour_minus_day_1 = DurationQuantity::from_hr(-36f64);
    let duration_day_2_minus_hour = DurationQuantity::from_day(0.5f64);

    assert_eq!(duration_day_1 - duration_day_2, duration_day_1_minus_day_2);
    assert_eq!(duration_hr - duration_day_1, duration_hour_minus_day_1);
    assert_eq!(duration_day_2 - duration_hr, duration_day_2_minus_hour);
}

#[test]
fn test_mass_multiply() {
    let duration_day_1 = DurationQuantity::from_day(70f64);
    let duration_day_2 = DurationQuantity::from_day(350f64);
    let duration_day_3 = DurationQuantity::from_day(267.4f64);

    assert_eq!(duration_day_1 * 5, duration_day_2);
    assert_eq!(duration_day_1 * 3.82, duration_day_3);
}

#[test]
fn test_distance_divide() {
    let duration_day_1 = DurationQuantity::from_day(350f64);
    let duration_day_2 = DurationQuantity::from_day(70f64);
    assert_eq!(duration_day_1 / 5, duration_day_2);
}

#[test]
fn test_energy_sum() {
    let duration_1 = DurationQuantity::from_day(30f64);
    let duration_2 = DurationQuantity::from_day(20f64);
    let duration_3 = DurationQuantity::from_day(50f64).to_hr();
    let duration_4 = DurationQuantity::from_hr(480f64);
    let duration_5 = DurationQuantity::from_day(130f64).to_week();
    let duration_total = DurationQuantity::from_day(250f64);

    let distances = vec![duration_1, duration_2, duration_3, duration_4, duration_5];

    let sum: DurationQuantity = distances.iter().map(|duration| *duration * 2).sum();
    assert_eq!(sum.get_unit(), duration_5.get_unit());
    assert_eq!(sum, (duration_total * 2).to_unit(duration_5.get_unit()));
}

#[test]
fn test_mass_partial_order() {
    let duration_day_1 = DurationQuantity::from_day(6f64);
    let duration_day_2 = DurationQuantity::from_day(4f64);
    let duration_hr = DurationQuantity::from_hr(120f64);
    assert!(duration_day_1 > duration_day_2);
    assert!(duration_day_1 > duration_hr);
    assert!(duration_hr > duration_day_2);
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = DurationUnit::save_enumerations_to_database(&pool).await;

    let duration_hr = DurationQuantity::from_hr(6.78f64);
    let duration_record = Entity::new(duration_hr);

    let res = duration_record.save_to_database(&pool).await;
    assert!(res.is_ok());

    let duration_saved =
        DurationQuantity::get_from_database_using_id(duration_record.get_id(), &pool).await;
    assert!(duration_saved.is_ok());
    assert_eq!(duration_saved.unwrap(), duration_record);

    let res = duration_record.delete_from_database_using_id(&pool).await;
    assert!(res.is_ok());

    let duration_saved_2 =
        DurationQuantity::get_from_database_using_id(duration_record.get_id(), &pool).await;
    assert!(duration_saved_2.is_err());
}
