use std::collections::BTreeSet;
use units::measurement_system::MeasurementSystem;

#[test]
fn test_get_measurement_system_enumerations() {
    let function_enumerations = MeasurementSystem::get_enumerations();
    let manual_enumerations = vec![
        MeasurementSystem::Metric,
        MeasurementSystem::Imperial,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}
