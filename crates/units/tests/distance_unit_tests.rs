use std::{collections::BTreeSet, str::FromStr};
use units::{distance::unit::DistanceUnit, measurement_system::MeasurementSystem};
use utils::database::DatabaseService;

#[test]
fn test_get_distance_unit_enumerations() {
    let function_enumerations = DistanceUnit::get_enumerations();
    let manual_enumerations = &vec![
        DistanceUnit::Kilometer,
        DistanceUnit::Meter,
        DistanceUnit::Centimeter,
        DistanceUnit::Millimeter,
        DistanceUnit::Foot,
        DistanceUnit::Inch,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(DistanceUnit::Kilometer.as_symbol(), "km");
    assert_eq!(DistanceUnit::Meter.as_symbol(), "m");
    assert_eq!(DistanceUnit::Centimeter.as_symbol(), "cm");
    assert_eq!(DistanceUnit::Millimeter.as_symbol(), "mm");
    assert_eq!(DistanceUnit::Foot.as_symbol(), "ft");
    assert_eq!(DistanceUnit::Inch.as_symbol(), "in");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(DistanceUnit::Kilometer.as_unit_type(), "kilometer");
    assert_eq!(DistanceUnit::Meter.as_unit_type(), "meter");
    assert_eq!(DistanceUnit::Centimeter.as_unit_type(), "centimeter");
    assert_eq!(DistanceUnit::Millimeter.as_unit_type(), "millimeter");
    assert_eq!(DistanceUnit::Foot.as_unit_type(), "foot");
    assert_eq!(DistanceUnit::Inch.as_unit_type(), "inch");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(DistanceUnit::Kilometer.as_unit_type_plural(), "kilometers");
    assert_eq!(DistanceUnit::Meter.as_unit_type_plural(), "meters");
    assert_eq!(
        DistanceUnit::Centimeter.as_unit_type_plural(),
        "centimeters"
    );
    assert_eq!(
        DistanceUnit::Millimeter.as_unit_type_plural(),
        "millimeters"
    );
    assert_eq!(DistanceUnit::Foot.as_unit_type_plural(), "feet");
    assert_eq!(DistanceUnit::Inch.as_unit_type_plural(), "inches");
}

#[test]
fn test_get_measurement_systems() {
    assert_eq!(
        DistanceUnit::Kilometer.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        DistanceUnit::Meter.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        DistanceUnit::Centimeter.get_measurement_system(),
        MeasurementSystem::Metric,
    );
    assert_eq!(
        DistanceUnit::Millimeter.get_measurement_system(),
        MeasurementSystem::Metric,
    );
    assert_eq!(
        DistanceUnit::Foot.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        DistanceUnit::Inch.get_measurement_system(),
        MeasurementSystem::Imperial
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(DistanceUnit::Kilometer.si_factor(), 1000 as f64);
    assert_eq!(DistanceUnit::Meter.si_factor(), 1 as f64);
    assert_eq!(DistanceUnit::Centimeter.si_factor(), 0.01);
    assert_eq!(DistanceUnit::Millimeter.si_factor(), 0.001 as f64);
    assert_eq!(DistanceUnit::Foot.si_factor(), 0.3048 as f64);
    assert_eq!(DistanceUnit::Inch.si_factor(), 0.0254);
}

#[test]
fn test_from_str() {
    assert_eq!(
        DistanceUnit::from_str("km").unwrap(),
        DistanceUnit::Kilometer
    );
    assert_eq!(
        DistanceUnit::from_str("kilometer").unwrap(),
        DistanceUnit::Kilometer
    );
    assert_eq!(
        DistanceUnit::from_str("kilometers").unwrap(),
        DistanceUnit::Kilometer
    );

    assert_eq!(DistanceUnit::from_str("m").unwrap(), DistanceUnit::Meter);
    assert_eq!(
        DistanceUnit::from_str("meter").unwrap(),
        DistanceUnit::Meter
    );
    assert_eq!(
        DistanceUnit::from_str("Meter").unwrap(),
        DistanceUnit::Meter
    );
    assert_eq!(
        DistanceUnit::from_str("meters").unwrap(),
        DistanceUnit::Meter
    );
    assert_eq!(
        DistanceUnit::from_str("mEtErS").unwrap(),
        DistanceUnit::Meter
    );
    assert_ne!(DistanceUnit::from_str("cm").unwrap(), DistanceUnit::Meter);

    assert_eq!(
        DistanceUnit::from_str("cm").unwrap(),
        DistanceUnit::Centimeter
    );
    assert_eq!(
        DistanceUnit::from_str("centimeter").unwrap(),
        DistanceUnit::Centimeter
    );
    assert_eq!(
        DistanceUnit::from_str("centimeters").unwrap(),
        DistanceUnit::Centimeter
    );

    assert_eq!(
        DistanceUnit::from_str("mm").unwrap(),
        DistanceUnit::Millimeter
    );
    assert_eq!(
        DistanceUnit::from_str("millimeter").unwrap(),
        DistanceUnit::Millimeter
    );
    assert_eq!(
        DistanceUnit::from_str("millimeters").unwrap(),
        DistanceUnit::Millimeter
    );

    assert_eq!(DistanceUnit::from_str("ft").unwrap(), DistanceUnit::Foot);
    assert_eq!(DistanceUnit::from_str("foot").unwrap(), DistanceUnit::Foot);
    assert_eq!(DistanceUnit::from_str("feet").unwrap(), DistanceUnit::Foot);

    assert_eq!(DistanceUnit::from_str("in").unwrap(), DistanceUnit::Inch);
    assert_eq!(DistanceUnit::from_str("inch").unwrap(), DistanceUnit::Inch);
    assert_eq!(
        DistanceUnit::from_str("inches").unwrap(),
        DistanceUnit::Inch
    );
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let res = DistanceUnit::save_to_database().await;
    assert!(res.is_ok());

    let distance = DistanceUnit::Kilometer.as_unit_type();
    let rows = sqlx::query!(
        r#"
            SELECT id, unit_type
            FROM units_distance_types
            WHERE unit_type = ?
        "#,
        distance
    )
    .fetch_one(&database_service.pool)
    .await
    .unwrap();
    let unit_type = rows.unit_type;

    assert_eq!(unit_type, "kilometer");
}

#[tokio::test]
async fn test_get_database_id() {
    let res = DistanceUnit::save_to_database().await;
    assert!(res.is_ok());

    let pint = DistanceUnit::Millimeter;
    let id = pint.get_database_id().await;
    assert!(id.is_ok());
}

