use std::{collections::BTreeSet, str::FromStr};
use units::distance::DistanceUnit;

#[test]
fn test_get_distance_unit_enumerations() {
    let function_enumerations = DistanceUnit::get_enumerations();
    let manual_enumerations = vec![
        &DistanceUnit::Meter,
        &DistanceUnit::Centimeter,
        &DistanceUnit::Foot,
        &DistanceUnit::Inch,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(DistanceUnit::Meter.as_symbol(), "m");
    assert_eq!(DistanceUnit::Centimeter.as_symbol(), "cm");
    assert_eq!(DistanceUnit::Foot.as_symbol(), "ft");
    assert_eq!(DistanceUnit::Inch.as_symbol(), "in");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(DistanceUnit::Meter.as_unit_type(), "meter");
    assert_eq!(DistanceUnit::Centimeter.as_unit_type(), "centimeter");
    assert_eq!(DistanceUnit::Foot.as_unit_type(), "foot");
    assert_eq!(DistanceUnit::Inch.as_unit_type(), "inch");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(DistanceUnit::Meter.as_unit_type_plural(), "meters");
    assert_eq!(
        DistanceUnit::Centimeter.as_unit_type_plural(),
        "centimeters"
    );
    assert_eq!(DistanceUnit::Foot.as_unit_type_plural(), "feet");
    assert_eq!(DistanceUnit::Inch.as_unit_type_plural(), "inches");
}

#[test]
fn test_get_si_factor() {
    assert_eq!(DistanceUnit::Meter.si_factor(), 1 as f64);
    assert_eq!(DistanceUnit::Centimeter.si_factor(), 0.01);
    assert_eq!(DistanceUnit::Foot.si_factor(), 0.3048 as f64);
    assert_eq!(DistanceUnit::Inch.si_factor(), 0.0254);
}

#[test]
fn test_from_str() {
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
