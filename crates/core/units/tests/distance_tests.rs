use units::{
    distance::{quantity::DistanceQuantity, unit::DistanceUnit},
    measurement_system::MeasurementSystem,
    record::{GetFromDatabaseUsingId, Record},
};
use utils::database::DatabaseService;

#[test]
fn test_new_distance() {
    let value = 10 as f64;

    let distance_new_km = DistanceQuantity::new(value, DistanceUnit::Kilometer);
    let distance_from_km = DistanceQuantity::from_km(value);
    assert_eq!(distance_new_km, distance_from_km);

    let distance_new_m = DistanceQuantity::new(value, DistanceUnit::Meter);
    let distance_from_m = DistanceQuantity::from_m(value);
    assert_eq!(distance_new_m, distance_from_m);

    let distance_new_cm = DistanceQuantity::new(value, DistanceUnit::Centimeter);
    let distance_from_cm = DistanceQuantity::from_cm(value);
    assert_eq!(distance_new_cm, distance_from_cm);

    let distance_new_mm = DistanceQuantity::new(value, DistanceUnit::Millimeter);
    let distance_from_mm = DistanceQuantity::from_mm(value);
    assert_eq!(distance_new_mm, distance_from_mm);

    let distance_new_ft = DistanceQuantity::new(value, DistanceUnit::Foot);
    let distance_from_ft = DistanceQuantity::from_ft(value);
    assert_eq!(distance_new_ft, distance_from_ft);

    let distance_new_in = DistanceQuantity::new(value, DistanceUnit::Inch);
    let distance_from_in = DistanceQuantity::from_in(value);
    assert_eq!(distance_new_in, distance_from_in);
}

#[test]
fn test_distance_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut distance_new = DistanceQuantity::new(value, DistanceUnit::Meter);
    let distance_rounded = distance_new.round(5);
    let distance_coded = DistanceQuantity::new(5.68033, DistanceUnit::Meter);
    assert_eq!(distance_rounded, distance_coded);

    let mut distance_new_2 = DistanceQuantity::new(value_2, DistanceUnit::Meter);
    let distance_rounded_2 = distance_new_2.round(5);
    let distance_coded_2 = DistanceQuantity::new(147.20473, DistanceUnit::Meter);
    assert_eq!(distance_rounded_2, distance_coded_2);
}

#[test]
fn test_distance_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let distance_km = DistanceQuantity::from_km(value);
    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(value);
    let distance_mm = DistanceQuantity::from_mm(value);
    let distance_ft = DistanceQuantity::from_ft(value);
    let distance_in = DistanceQuantity::from_in(value);

    // percentage error calculations
    assert!((distance_km.as_m() - value * 1000f64).abs() / distance_km.as_m() < percentage_err);
    assert!(
        (distance_km.as_cm() - value * 100_000f64).abs() / distance_km.as_cm() < percentage_err
    );
    assert!(
        (distance_km.as_mm() - value * 1_000_000f64).abs() / distance_km.as_mm() < percentage_err
    );
    assert!((distance_km.as_ft() - value * 3280.84).abs() / distance_km.as_ft() < percentage_err);
    assert!((distance_km.as_in() - value * 39_370.08).abs() / distance_km.as_in() < percentage_err);

    assert!((distance_m.as_km() - value * 0.001).abs() / distance_m.as_km() < percentage_err);
    assert!((distance_m.as_cm() - value * 100f64).abs() / distance_m.as_cm() < percentage_err);
    assert!((distance_m.as_mm() - value * 1000f64).abs() / distance_m.as_mm() < percentage_err);
    assert!((distance_m.as_ft() - value * 3.28084).abs() / distance_m.as_ft() < percentage_err);
    assert!((distance_m.as_in() - value * 39.37008).abs() / distance_m.as_in() < percentage_err);

    assert!((distance_cm.as_km() - value * 0.00001).abs() / distance_cm.as_km() < percentage_err);
    assert!((distance_cm.as_m() - value * 0.01).abs() / distance_cm.as_m() < percentage_err);
    assert!((distance_cm.as_mm() - value * 10f64).abs() / distance_cm.as_mm() < percentage_err);
    assert!((distance_cm.as_ft() - value * 0.0328084).abs() / distance_cm.as_ft() < percentage_err);
    assert!((distance_cm.as_in() - value * 0.3937008).abs() / distance_cm.as_in() < percentage_err);

    assert!((distance_mm.as_km() - value * 0.000001).abs() / distance_mm.as_km() < percentage_err);
    assert!((distance_mm.as_m() - value * 0.001).abs() / distance_mm.as_m() < percentage_err);
    assert!((distance_mm.as_cm() - value * 0.1f64).abs() / distance_mm.as_cm() < percentage_err);
    assert!(
        (distance_mm.as_ft() - value * 0.00328084).abs() / distance_mm.as_ft() < percentage_err
    );
    assert!(
        (distance_mm.as_in() - value * 0.03937008).abs() / distance_mm.as_in() < percentage_err
    );

    assert!((distance_ft.as_km() - value * 0.0003048).abs() / distance_cm.as_km() < percentage_err);
    assert!((distance_ft.as_m() - value * 0.3048).abs() / distance_cm.as_m() < percentage_err);
    assert!((distance_ft.as_cm() - value * 30.48).abs() / distance_cm.as_cm() < percentage_err);
    assert!((distance_ft.as_mm() - value * 304.8).abs() / distance_cm.as_mm() < percentage_err);
    assert!((distance_ft.as_in() - value * 12f64).abs() / distance_cm.as_in() < percentage_err);

    assert!((distance_in.as_km() - value * 0.0000254).abs() / distance_in.as_km() < percentage_err);
    assert!((distance_in.as_m() - value * 0.0254).abs() / distance_in.as_m() < percentage_err);
    assert!((distance_in.as_cm() - value * 2.54).abs() / distance_in.as_cm() < percentage_err);
    assert!((distance_in.as_mm() - value * 25.4).abs() / distance_in.as_mm() < percentage_err);
    assert!(
        (distance_in.as_ft() - value * 0.08333333).abs() / distance_in.as_ft() < percentage_err
    );
}

#[test]
fn test_distance_to_unit() {
    let value = 5.6;
    let new_value = value * 100f64;

    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(new_value);
    let distance_m_to_cm = distance_m.to_unit(DistanceUnit::Centimeter);

    print!(
        "distance_cm1: {},\ndistance_cm2: {}",
        distance_cm, distance_m_to_cm
    );
    assert_eq!(distance_cm, distance_m_to_cm);
}

#[test]
fn test_distance_to_fn() {
    let value = 6.9;
    let new_value = value * 100f64;

    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(new_value);
    let distance_m_to_cm = distance_m.to_cm();
    assert_eq!(distance_cm, distance_m_to_cm);
}

#[test]
fn test_distance_is_zero() {
    let zero_distance = DistanceQuantity::from_m(0f64);
    let distance = DistanceQuantity::from_m(5.5);

    assert!(zero_distance.is_zero());
    assert!(!distance.is_zero());
}

#[test]
fn test_distance_is_negative() {
    let negative_distance = DistanceQuantity::from_m(-5.5);
    let distance = DistanceQuantity::from_m(5.5);

    assert!(negative_distance.is_negative());
    assert!(!distance.is_negative());
}

#[test]
fn test_distance_get_value() {
    let distance = DistanceQuantity::new(6.882, DistanceUnit::Meter);
    assert_eq!(distance.get_value(), 6.882);
}

#[test]
fn test_distance_set_value() {
    let mut distance = DistanceQuantity::new(6.882, DistanceUnit::Meter);
    distance.set_value(8.92);
    assert_eq!(distance.get_value(), 8.92);
}

#[test]
fn test_distance_get_unit() {
    let distance = DistanceQuantity::new(6.882, DistanceUnit::Meter);
    assert_eq!(distance.get_unit(), DistanceUnit::Meter);
}

#[test]
fn test_distance_set_unit() {
    let mut distance = DistanceQuantity::new(6.882, DistanceUnit::Meter);
    distance.set_unit(DistanceUnit::Foot);
    assert_eq!(distance.get_unit(), DistanceUnit::Foot);
}

#[test]
fn test_distance_get_symbol() {
    let value = 4.2;
    let distance_km = DistanceQuantity::from_km(value);
    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(value);
    let distance_mm = DistanceQuantity::from_mm(value);
    let distance_ft = DistanceQuantity::from_ft(value);
    let distance_in = DistanceQuantity::from_in(value);

    assert_eq!(distance_km.get_symbol(), "km");
    assert_eq!(distance_m.get_symbol(), "m");
    assert_eq!(distance_cm.get_symbol(), "cm");
    assert_eq!(distance_mm.get_symbol(), "mm");
    assert_eq!(distance_ft.get_symbol(), "ft");
    assert_eq!(distance_in.get_symbol(), "in");
}

#[test]
fn test_distance_get_measurement_system() {
    let value = 4.2;

    let distance_km = DistanceQuantity::from_km(value);
    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(value);
    let distance_mm = DistanceQuantity::from_mm(value);
    let distance_ft = DistanceQuantity::from_ft(value);
    let distance_in = DistanceQuantity::from_in(value);

    assert_eq!(
        distance_km.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        distance_m.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        distance_cm.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        distance_mm.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        distance_ft.get_measurement_system(),
        MeasurementSystem::Imperial
    );
    assert_eq!(
        distance_in.get_measurement_system(),
        MeasurementSystem::Imperial
    );
}

#[test]
fn test_distance_get_unit_type() {
    let value = 4.2;
    let distance_km = DistanceQuantity::from_km(value);
    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(value);
    let distance_mm = DistanceQuantity::from_mm(value);
    let distance_ft = DistanceQuantity::from_ft(value);
    let distance_in = DistanceQuantity::from_in(value);

    assert_eq!(distance_km.get_unit_type(), "kilometer");
    assert_eq!(distance_m.get_unit_type(), "meter");
    assert_eq!(distance_cm.get_unit_type(), "centimeter");
    assert_eq!(distance_mm.get_unit_type(), "millimeter");
    assert_eq!(distance_ft.get_unit_type(), "foot");
    assert_eq!(distance_in.get_unit_type(), "inch");
}

#[test]
fn test_distance_get_unit_type_plural() {
    let value = 8.52;
    let distance_km = DistanceQuantity::from_km(value);
    let distance_m = DistanceQuantity::from_m(value);
    let distance_cm = DistanceQuantity::from_cm(value);
    let distance_mm = DistanceQuantity::from_mm(value);
    let distance_ft = DistanceQuantity::from_ft(value);
    let distance_in = DistanceQuantity::from_in(value);

    assert_eq!(distance_km.get_unit_type_plural(), "kilometers");
    assert_eq!(distance_m.get_unit_type_plural(), "meters");
    assert_eq!(distance_cm.get_unit_type_plural(), "centimeters");
    assert_eq!(distance_mm.get_unit_type_plural(), "millimeters");
    assert_eq!(distance_ft.get_unit_type_plural(), "feet");
    assert_eq!(distance_in.get_unit_type_plural(), "inches");
}

#[test]
fn test_distance_to_string() {
    let value_1 = 5f64;
    let value_2 = 8.642;

    let distance_km_1 = DistanceQuantity::from_km(value_1);
    assert_eq!(distance_km_1.to_string(), "5km");
    let distance_m_1 = DistanceQuantity::from_m(value_1);
    assert_eq!(distance_m_1.to_string(), "5m");
    let distance_cm_1 = DistanceQuantity::from_cm(value_1);
    assert_eq!(distance_cm_1.to_string(), "5cm");
    let distance_mm_1 = DistanceQuantity::from_mm(value_1);
    assert_eq!(distance_mm_1.to_string(), "5mm");
    let distance_ft_1 = DistanceQuantity::from_ft(value_1);
    assert_eq!(distance_ft_1.to_string(), "5ft");
    let distance_in_1 = DistanceQuantity::from_in(value_1);
    assert_eq!(distance_in_1.to_string(), "5in");

    let distance_km_2 = DistanceQuantity::from_km(value_2);
    assert_eq!(distance_km_2.to_string(), "8.642km");
    let distance_m_2 = DistanceQuantity::from_m(value_2);
    assert_eq!(distance_m_2.to_string(), "8.642m");
    let distance_cm_2 = DistanceQuantity::from_cm(value_2);
    assert_eq!(distance_cm_2.to_string(), "8.642cm");
    let distance_mm_2 = DistanceQuantity::from_mm(value_2);
    assert_eq!(distance_mm_2.to_string(), "8.642mm");
    let distance_ft_2 = DistanceQuantity::from_ft(value_2);
    assert_eq!(distance_ft_2.to_string(), "8.642ft");
    let distance_in_2 = DistanceQuantity::from_in(value_2);
    assert_eq!(distance_in_2.to_string(), "8.642in");
}

#[test]
fn test_distance_add() {
    let distance_m_1 = DistanceQuantity::from_m(1f64);
    let distance_m_2 = DistanceQuantity::from_m(5f64);
    let distance_cm = DistanceQuantity::from_cm(200f64);

    let distance_m_1_plus_m_2 = DistanceQuantity::from_m(6f64);
    let distance_cm_plus_m_1 = DistanceQuantity::from_cm(300f64);
    let distance_m_2_plus_cm = DistanceQuantity::from_m(7f64);

    assert_eq!((distance_m_1 + distance_m_2), distance_m_1_plus_m_2);
    assert_eq!((distance_cm + distance_m_1), distance_cm_plus_m_1);
    assert_eq!((distance_m_2 + distance_cm), distance_m_2_plus_cm);
}

#[test]
fn test_distance_subtract() {
    let distance_m_1 = DistanceQuantity::from_m(67f64);
    let distance_m_2 = DistanceQuantity::from_m(47f64);
    let distance_cm = DistanceQuantity::from_cm(1000f64);

    let distance_m_1_minus_m_2 = DistanceQuantity::from_m(20f64);
    let distance_cm_minus_m_1 = DistanceQuantity::from_cm(-5700f64);
    let distance_m_2_minus_cm = DistanceQuantity::from_m(37f64);

    assert_eq!((distance_m_1 - distance_m_2), distance_m_1_minus_m_2);
    assert_eq!((distance_cm - distance_m_1), distance_cm_minus_m_1);
    assert_eq!((distance_m_2 - distance_cm), distance_m_2_minus_cm);
}

#[test]
fn test_distance_multiply() {
    let distance_m_1 = DistanceQuantity::from_m(70f64);
    let distance_m_2 = DistanceQuantity::from_m(350f64);
    let distance_m_3 = DistanceQuantity::from_m(267.4f64);

    assert_eq!((distance_m_1 * 5), distance_m_2);
    assert_eq!((distance_m_1 * 3.82), distance_m_3);
}

#[test]
fn test_distance_divide() {
    let distance_m_1 = DistanceQuantity::from_m(350f64);
    let distance_m_2 = DistanceQuantity::from_m(70f64);
    assert_eq!((distance_m_1 / 5), distance_m_2);
}

#[test]
fn test_distance_sum() {
    let distance_1 = DistanceQuantity::from_m(30f64);
    let distance_2 = DistanceQuantity::from_m(20f64);
    let distance_3 = DistanceQuantity::from_m(50f64).to_ft();
    let distance_4 = DistanceQuantity::from_m(20f64).to_ft();
    let distance_5 = DistanceQuantity::from_m(130f64).to_ft();
    let distance_total = DistanceQuantity::from_m(250f64);

    let distances = vec![distance_1, distance_2, distance_3, distance_4, distance_5];

    let sum: DistanceQuantity = distances.iter().map(|distance| *distance * 2).sum();
    assert_eq!(sum.get_unit(), distance_5.get_unit());
    assert_eq!(sum, (distance_total * 2).to_unit(distance_5.get_unit()));
}

#[test]
fn test_distance_partial_order() {
    let distance_m_1 = DistanceQuantity::from_m(6f64);
    let distance_m_2 = DistanceQuantity::from_m(4f64);
    let distance_cm = DistanceQuantity::from_cm(520f64);
    assert!(distance_m_1 > distance_m_2);
    assert!(distance_m_1 > distance_cm);
    assert!(distance_cm > distance_m_2);
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = DistanceUnit::save_enumerations_to_database(&pool).await;

    let distance_m = DistanceQuantity::from_m(6700f64);
    let distance_record = Record::new(distance_m);

    let res = distance_record.save_to_database(&pool).await;
    assert!(res.is_ok());

    let distance_saved =
        DistanceQuantity::get_from_database_using_id(distance_record.get_id(), &pool).await;
    assert!(distance_saved.is_ok());
    assert_eq!(distance_saved.unwrap(), distance_record);

    let res = distance_record.delete_from_database_using_id(&pool).await;
    assert!(res.is_ok());

    let distance_saved_2 =
        DistanceQuantity::get_from_database_using_id(distance_record.get_id(), &pool).await;
    assert!(distance_saved_2.is_err());
}
