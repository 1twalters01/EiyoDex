use utils::base_types::angle::Angle;

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

#[test]
fn test_angle_multiplication() {
    let angle_1 = Angle::new(100f64);
    let angle_2 = Angle::new(200f64);
    let err = (angle_1.get_value() * 2f64 - angle_2.get_value()).abs();
    assert!(err < 0.0001);
}

#[test]
fn test_angle_reflex_multiplication() {
    let angle_1 = Angle::new(200f64);
    let angle_2 = Angle::new(400f64);
    let angle_3 = Angle::new(40f64);

    let err = (angle_1.get_value() * 2f64 - 360f64 - angle_2.get_value()).abs();
    assert!(err < 0.0001);

    let err = (angle_1.get_value() * 2f64 - 360f64 - angle_3.get_value()).abs();
    assert!(err < 0.0001);
}

#[test]
fn test_angle_division() {
    let angle_1 = Angle::new(200f64);
    let angle_2 = Angle::new(100f64);
    let err = (angle_1.get_value() / 2f64 - angle_2.get_value()).abs();
    assert!(err < 0.0001);
}

#[test]
fn test_angle_reflex_division() {
    let angle_1 = Angle::new(400f64);
    let angle_2 = Angle::new(20f64);
    let err = (angle_1.get_value() / 2f64 - angle_2.get_value()).abs();
    assert!(err < 0.0001);
}
