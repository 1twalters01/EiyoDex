use units::{
    duration::{DurationUnit, DurationWrapper},
    measurement_system::MeasurementSystem,
};

#[test]
fn test_new_distance() {
    let value = 10 as f64;

    let duration_new_week = DurationWrapper::new(value, DurationUnit::Week);
    let duration_from_week = DurationWrapper::from_week(value);
    assert_eq!(duration_new_week, duration_from_week);

    let duration_new_day = DurationWrapper::new(value, DurationUnit::Day);
    let duration_from_day = DurationWrapper::from_day(value);
    assert_eq!(duration_new_day, duration_from_day);

    let duration_new_hr = DurationWrapper::new(value, DurationUnit::Hour);
    let distance_from_cm = DurationWrapper::from_hr(value);
    assert_eq!(duration_new_hr, distance_from_cm);

    let duration_new_min = DurationWrapper::new(value, DurationUnit::Minute);
    let duration_from_min = DurationWrapper::from_min(value);
    assert_eq!(duration_new_min, duration_from_min);

    let duration_new_s = DurationWrapper::new(value, DurationUnit::Second);
    let duration_from_s = DurationWrapper::from_s(value);
    assert_eq!(duration_new_s, duration_from_s);
}

#[test]
fn test_distance_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let duration_week = DurationWrapper::from_week(value);
    let duration_day = DurationWrapper::from_day(value);
    let duration_hr = DurationWrapper::from_hr(value);
    let duration_min = DurationWrapper::from_min(value);
    let duration_s = DurationWrapper::from_s(value);

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

// #[test]
// fn test_distance_to_unit() {
//     let value = 5.6;
//     let new_value = value * 100f64;
//
//     let duration_day = DurationWrapper::from_day(value);
//     let duration_hr = DurationWrapper::from_hr(new_value);
//     let distance_m_to_cm = duration_day.to_unit(DurationUnit::Centimeter);
//
//     print!(
//         "mass_ounces1: {},\nmass_ounces2: {}",
//         duration_hr, distance_m_to_cm
//     );
//     assert_eq!(duration_hr, distance_m_to_cm);
// }
//
// #[test]
// fn test_distance_to_fn() {
//     let value = 6.9;
//     let new_value = value * 100f64;
//
//     let duration_day = DurationWrapper::from_day(value);
//     let duration_hr = DurationWrapper::from_hr(new_value);
//     let distance_m_to_cm = duration_day.to_cm();
//
//     print!(
//         "distance_centimeters1: {},\ndistance_centimeters2: {}",
//         duration_hr, distance_m_to_cm
//     );
//     assert_eq!(duration_hr, distance_m_to_cm);
// }
//
// #[test]
// fn test_get_duration() {}
//
// #[test]
// fn test_distance_is_zero() {
//     let zero_distance = DurationWrapper::from_day(0f64);
//     let distance = DurationWrapper::from_day(5.5);
//
//     assert!(zero_distance.is_zero());
//     assert!(!distance.is_zero());
// }
//
// #[test]
// fn test_distance_is_negative() {
//     let negative_distance = DurationWrapper::from_day(-5.5);
//     let distance = DurationWrapper::from_day(5.5);
//
//     assert!(negative_distance.is_negative());
//     assert!(!distance.is_negative());
// }
//
// #[test]
// fn test_distance_get_value() {
//     let distance = DurationWrapper::new(6.882, DurationUnit::Meter);
//     assert_eq!(distance.get_value(), 6.882);
// }
//
// #[test]
// fn test_distance_set_value() {
//     let mut distance = DurationWrapper::new(6.882, DurationUnit::Meter);
//     distance.set_value(8.92);
//     assert_eq!(distance.get_value(), 8.92);
// }
//
// #[test]
// fn test_distance_get_unit() {
//     let distance = DurationWrapper::new(6.882, DurationUnit::Meter);
//     assert_eq!(distance.get_unit(), DurationUnit::Meter);
// }
//
// #[test]
// fn test_distance_set_unit() {
//     let mut distance = DurationWrapper::new(6.882, DurationUnit::Meter);
//     distance.set_unit(DurationUnit::Foot);
//     assert_eq!(distance.get_unit(), DurationUnit::Foot);
// }
//
// #[test]
// fn test_distance_get_symbol() {
//     let value = 4.2;
//     let duration_week = DurationWrapper::from_week(value);
//     let duration_day = DurationWrapper::from_day(value);
//     let duration_hr = DurationWrapper::from_hr(value);
//     let duration_min = DurationWrapper::from_min(value);
//     let duration_s = DurationWrapper::from_s(value);
//     let distance_in = DurationWrapper::from_in(value);
//
//     assert_eq!(duration_week.get_symbol(), "km");
//     assert_eq!(duration_day.get_symbol(), "m");
//     assert_eq!(duration_hr.get_symbol(), "cm");
//     assert_eq!(duration_min.get_symbol(), "mm");
//     assert_eq!(duration_s.get_symbol(), "ft");
//     assert_eq!(distance_in.get_symbol(), "in");
// }
//
// #[test]
// fn test_distance_get_measurement_system() {
//     let value = 4.2;
//
//     let duration_week = DurationWrapper::from_week(value);
//     let duration_day = DurationWrapper::from_day(value);
//     let duration_hr = DurationWrapper::from_hr(value);
//     let duration_min = DurationWrapper::from_min(value);
//     let duration_s = DurationWrapper::from_s(value);
//     let distance_in = DurationWrapper::from_in(value);
//
//     assert_eq!(
//         duration_week.get_measurement_system(),
//         MeasurementSystem::Metric
//     );
//     assert_eq!(
//         duration_day.get_measurement_system(),
//         MeasurementSystem::Metric
//     );
//     assert_eq!(
//         duration_hr.get_measurement_system(),
//         MeasurementSystem::Metric
//     );
//     assert_eq!(
//         duration_min.get_measurement_system(),
//         MeasurementSystem::Metric
//     );
//     assert_eq!(
//         duration_s.get_measurement_system(),
//         MeasurementSystem::Imperial
//     );
//     assert_eq!(
//         distance_in.get_measurement_system(),
//         MeasurementSystem::Imperial
//     );
// }
//
// #[test]
// fn test_mass_get_unit_type() {
//     let value = 4.2;
//     let duration_week = DurationWrapper::from_week(value);
//     let duration_day = DurationWrapper::from_day(value);
//     let duration_hr = DurationWrapper::from_hr(value);
//     let duration_min = DurationWrapper::from_min(value);
//     let duration_s = DurationWrapper::from_s(value);
//     let distance_in = DurationWrapper::from_in(value);
//
//     assert_eq!(duration_week.get_unit_type(), "kilometer");
//     assert_eq!(duration_day.get_unit_type(), "meter");
//     assert_eq!(duration_hr.get_unit_type(), "centimeter");
//     assert_eq!(duration_min.get_unit_type(), "millimeter");
//     assert_eq!(duration_s.get_unit_type(), "foot");
//     assert_eq!(distance_in.get_unit_type(), "inch");
// }
//
// #[test]
// fn test_mass_get_unit_type_plural() {
//     let value = 8.52;
//     let duration_week = DurationWrapper::from_week(value);
//     let duration_day = DurationWrapper::from_day(value);
//     let duration_hr = DurationWrapper::from_hr(value);
//     let duration_min = DurationWrapper::from_min(value);
//     let duration_s = DurationWrapper::from_s(value);
//     let distance_in = DurationWrapper::from_in(value);
//
//     assert_eq!(duration_week.get_unit_type_plural(), "kilometers");
//     assert_eq!(duration_day.get_unit_type_plural(), "meters");
//     assert_eq!(duration_hr.get_unit_type_plural(), "centimeters");
//     assert_eq!(duration_min.get_unit_type_plural(), "millimeters");
//     assert_eq!(duration_s.get_unit_type_plural(), "feet");
//     assert_eq!(distance_in.get_unit_type_plural(), "inches");
// }
//
// #[test]
// fn test_mass_to_string() {
//     let value_1 = 5f64;
//     let value_2 = 8.642;
//
//     let distance_km_1 = DurationWrapper::from_week(value_1);
//     assert_eq!(distance_km_1.to_string(), "5km");
//     let distance_m_1 = DurationWrapper::from_day(value_1);
//     assert_eq!(distance_m_1.to_string(), "5m");
//     let distance_cm_1 = DurationWrapper::from_hr(value_1);
//     assert_eq!(distance_cm_1.to_string(), "5cm");
//     let distance_mm_1 = DurationWrapper::from_min(value_1);
//     assert_eq!(distance_mm_1.to_string(), "5mm");
//     let distance_ft_1 = DurationWrapper::from_s(value_1);
//     assert_eq!(distance_ft_1.to_string(), "5ft");
//     let distance_in_1 = DurationWrapper::from_in(value_1);
//     assert_eq!(distance_in_1.to_string(), "5in");
//
//     let distance_km_2 = DurationWrapper::from_week(value_2);
//     assert_eq!(distance_km_2.to_string(), "8.642km");
//     let distance_m_2 = DurationWrapper::from_day(value_2);
//     assert_eq!(distance_m_2.to_string(), "8.642m");
//     let distance_cm_2 = DurationWrapper::from_hr(value_2);
//     assert_eq!(distance_cm_2.to_string(), "8.642cm");
//     let distance_mm_2 = DurationWrapper::from_min(value_2);
//     assert_eq!(distance_mm_2.to_string(), "8.642mm");
//     let distance_ft_2 = DurationWrapper::from_s(value_2);
//     assert_eq!(distance_ft_2.to_string(), "8.642ft");
//     let distance_in_2 = DurationWrapper::from_in(value_2);
//     assert_eq!(distance_in_2.to_string(), "8.642in");
// }
//
// #[test]
// fn test_mass_add() {
//     let distance_m_1 = DurationWrapper::from_day(1f64);
//     let distance_m_2 = DurationWrapper::from_day(5f64);
//     let duration_hr = DurationWrapper::from_hr(200f64);
//
//     let mass_m_1_plus_m_2 = DurationWrapper::from_day(6f64);
//     let mass_cm_plus_m_1 = DurationWrapper::from_hr(300f64);
//     let mass_m_2_plus_cm = DurationWrapper::from_day(7f64);
//
//     assert_eq!(distance_m_1 + distance_m_2, mass_m_1_plus_m_2);
//     assert_eq!(duration_hr + distance_m_1, mass_cm_plus_m_1);
//     assert_eq!(distance_m_2 + duration_hr, mass_m_2_plus_cm);
// }
//
// #[test]
// fn test_mass_subtract() {
//     let distance_m_1 = DurationWrapper::from_day(67f64);
//     let distance_m_2 = DurationWrapper::from_day(47f64);
//     let duration_hr = DurationWrapper::from_hr(1000f64);
//
//     let distance_m_1_minus_m_2 = DurationWrapper::from_day(20f64);
//     let distance_cm_minus_m_1 = DurationWrapper::from_hr(-5700f64);
//     let distance_m_2_minus_cm = DurationWrapper::from_day(37f64);
//
//     assert_eq!(distance_m_1 - distance_m_2, distance_m_1_minus_m_2);
//     assert_eq!(duration_hr - distance_m_1, distance_cm_minus_m_1);
//     assert_eq!(distance_m_2 - duration_hr, distance_m_2_minus_cm);
// }
//
// #[test]
// fn test_mass_multiply() {
//     let distance_m_1 = DurationWrapper::from_day(70f64);
//     let distance_m_2 = DurationWrapper::from_day(350f64);
//     let mass_g_3 = DurationWrapper::from_day(267.4f64);
//
//     assert_eq!(distance_m_1 * 5, distance_m_2);
//     assert_eq!(distance_m_1 * 3.82, mass_g_3);
// }
//
// #[test]
// fn test_distance_divide() {
//     let distance_m_1 = DurationWrapper::from_day(350f64);
//     let distance_m_2 = DurationWrapper::from_day(70f64);
//     assert_eq!(distance_m_1 / 5, distance_m_2);
// }
//
// #[test]
// fn test_energy_sum() {
//     let distance_1 = DurationWrapper::from_day(30f64);
//     let distance_2 = DurationWrapper::from_day(20f64);
//     let distance_3 = DurationWrapper::from_day(50f64).to_ft();
//     let distance_4 = DurationWrapper::from_day(20f64).to_ft();
//     let distance_5 = DurationWrapper::from_day(130f64).to_ft();
//     let distance_total = DurationWrapper::from_day(250f64);
//
//     let distances = vec![distance_1, distance_2, distance_3, distance_4,
// distance_5];
//
//     let sum: DurationWrapper = distances.iter().map(|distance| *distance *
// 2).sum();     assert_eq!(sum.get_unit(), distance_5.get_unit());
//     assert_eq!(sum, (distance_total * 2).to_unit(distance_5.get_unit()));
// }
//
// #[test]
// fn test_mass_partial_order() {
//     let distance_m_1 = DurationWrapper::from_day(6f64);
//     let distance_m_2 = DurationWrapper::from_day(4f64);
//     let duration_hr = DurationWrapper::from_hr(520f64);
//     assert!(distance_m_1 > distance_m_2);
//     assert!(distance_m_1 > duration_hr);
//     assert!(duration_hr > distance_m_2);
// }
