use base_types::Angle;

#[test]
fn test_angle_one_revolution() {
    let angle_1 = Angle::new(360f64);
    assert_eq!(angle_1.get_value(), 0f64);
}

#[test]
fn test_angle_negative_angle() {
    let angle_1 = Angle::new(-10.5);
    let err = (angle_1.get_value() + Angle::new(10.5).get_value())
        .abs()
        .rem_euclid(360f64);
    println!("{}", err);
    assert!(err < 0.0001);
}

#[test]
fn test_angle_over_one_revolution() {
    let angle_1 = Angle::new(400.52);
    let err = (angle_1.get_value() - Angle::new(40.52).get_value()).abs();
    assert!(err < 0.0001);
}

#[test]
fn test_angle_over_two_revolution() {
    let angle_1 = Angle::new(760f64);
    let err = (angle_1.get_value() - Angle::new(40f64).get_value()).abs();
    assert!(err < 0.0001);
}