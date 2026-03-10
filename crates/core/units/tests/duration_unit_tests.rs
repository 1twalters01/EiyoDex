use std::{collections::BTreeSet, str::FromStr};
use units::{duration::unit::DurationUnit, measurement_system::MeasurementSystem};
use utils::database::DatabaseService;

#[test]
fn test_get_distance_unit_enumerations() {
    let function_enumerations = DurationUnit::get_enumerations();
    let manual_enumerations = &vec![
        DurationUnit::Week,
        DurationUnit::Day,
        DurationUnit::Hour,
        DurationUnit::Minute,
        DurationUnit::Second,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(DurationUnit::Week.as_symbol(), "week");
    assert_eq!(DurationUnit::Day.as_symbol(), "day");
    assert_eq!(DurationUnit::Hour.as_symbol(), "hr");
    assert_eq!(DurationUnit::Minute.as_symbol(), "min");
    assert_eq!(DurationUnit::Second.as_symbol(), "s");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(DurationUnit::Week.as_unit_type(), "week");
    assert_eq!(DurationUnit::Day.as_unit_type(), "day");
    assert_eq!(DurationUnit::Hour.as_unit_type(), "hour");
    assert_eq!(DurationUnit::Minute.as_unit_type(), "minute");
    assert_eq!(DurationUnit::Second.as_unit_type(), "second");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(DurationUnit::Week.as_unit_type_plural(), "weeks");
    assert_eq!(DurationUnit::Day.as_unit_type_plural(), "days");
    assert_eq!(DurationUnit::Hour.as_unit_type_plural(), "hours");
    assert_eq!(DurationUnit::Minute.as_unit_type_plural(), "minutes");
    assert_eq!(DurationUnit::Second.as_unit_type_plural(), "seconds");
}

#[test]
fn test_get_measurement_systems() {
    assert_eq!(
        DurationUnit::Week.get_measurement_system(),
        MeasurementSystem::SI
    );
    assert_eq!(
        DurationUnit::Day.get_measurement_system(),
        MeasurementSystem::SI
    );
    assert_eq!(
        DurationUnit::Hour.get_measurement_system(),
        MeasurementSystem::SI,
    );
    assert_eq!(
        DurationUnit::Minute.get_measurement_system(),
        MeasurementSystem::SI,
    );
    assert_eq!(
        DurationUnit::Second.get_measurement_system(),
        MeasurementSystem::SI
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(DurationUnit::Week.si_factor(), 604_800 as f64);
    assert_eq!(DurationUnit::Day.si_factor(), 86_400f64);
    assert_eq!(DurationUnit::Hour.si_factor(), 3600f64);
    assert_eq!(DurationUnit::Minute.si_factor(), 60f64);
    assert_eq!(DurationUnit::Second.si_factor(), 1f64);
}

#[test]
fn test_from_str() {
    assert_eq!(DurationUnit::from_str("week").unwrap(), DurationUnit::Week);
    assert_eq!(DurationUnit::from_str("weeks").unwrap(), DurationUnit::Week);

    assert_eq!(DurationUnit::from_str("day").unwrap(), DurationUnit::Day);
    assert_eq!(DurationUnit::from_str("dAy").unwrap(), DurationUnit::Day);
    assert_eq!(DurationUnit::from_str("Day").unwrap(), DurationUnit::Day);
    assert_ne!(DurationUnit::from_str("week").unwrap(), DurationUnit::Day);

    assert_eq!(DurationUnit::from_str("hr").unwrap(), DurationUnit::Hour);
    assert_eq!(DurationUnit::from_str("hour").unwrap(), DurationUnit::Hour);
    assert_eq!(DurationUnit::from_str("hours").unwrap(), DurationUnit::Hour);

    assert_eq!(
        DurationUnit::from_str("minute").unwrap(),
        DurationUnit::Minute
    );
    assert_eq!(
        DurationUnit::from_str("minutes").unwrap(),
        DurationUnit::Minute
    );
    assert_eq!(DurationUnit::from_str("min").unwrap(), DurationUnit::Minute);

    assert_eq!(DurationUnit::from_str("s").unwrap(), DurationUnit::Second);
    assert_eq!(
        DurationUnit::from_str("second").unwrap(),
        DurationUnit::Second
    );
    assert_eq!(
        DurationUnit::from_str("seconds").unwrap(),
        DurationUnit::Second
    );
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = DurationUnit::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let duration = DurationUnit::Second.as_unit_type();
    let rows = sqlx::query!(
        r#"
            SELECT id, unit_type
            FROM units_duration_types
            WHERE unit_type = ?
        "#,
        duration
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let unit_type = rows.unit_type;

    assert_eq!(unit_type, "second");
}

#[tokio::test]
async fn test_get_database_id() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = DurationUnit::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let pint = DurationUnit::Minute;
    let id = pint.get_database_id(&pool).await;
    assert!(id.is_ok());
}
