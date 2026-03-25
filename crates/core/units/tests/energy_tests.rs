use units::{
    energy::{quantity::EnergyQuantity, unit::EnergyUnit},
    measurement_system::MeasurementSystem,
    entity::{GetFromDatabaseUsingId, Entity},
};
use utils::database::DatabaseService;

#[test]
fn test_new_energy() {
    let value = 10 as f64;

    let energy_new_kcal = EnergyQuantity::new(value, EnergyUnit::Kilocalorie);
    let energy_from_kcal = EnergyQuantity::from_kcal(value);
    assert_eq!(energy_new_kcal, energy_from_kcal);

    let energy_new_kj = EnergyQuantity::new(value, EnergyUnit::Kilojoule);
    let energy_from_kj = EnergyQuantity::from_kj(value);
    assert_eq!(energy_new_kj, energy_from_kj);
}

#[test]
fn test_energy_rounding() {
    let value = 5.6803294822;
    let value_2 = 147.20473186;

    let mut energy_new = EnergyQuantity::new(value, EnergyUnit::Kilocalorie);
    let energy_rounded = energy_new.round(5);
    let energy_coded = EnergyQuantity::new(5.68033, EnergyUnit::Kilocalorie);
    assert_eq!(energy_rounded, energy_coded);

    let mut energy_new_2 = EnergyQuantity::new(value_2, EnergyUnit::Kilocalorie);
    let energy_rounded_2 = energy_new_2.round(5);
    let energy_coded_2 = EnergyQuantity::new(147.20473, EnergyUnit::Kilocalorie);
    assert_eq!(energy_rounded_2, energy_coded_2);
}

#[test]
fn test_energy_as_fn() {
    let value = 5.6;
    let percentage_err = 0.5;

    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(value);

    // percentage error calculations
    assert!(
        (energy_kcal.as_kj() - value * 4.184 as f64).abs() / energy_kcal.as_kcal() < percentage_err
    );
    assert!(
        (energy_kj.as_kcal() - value * 0.2390057 as f64).abs() / energy_kj.as_kcal()
            < percentage_err
    );
}

#[test]
fn test_energy_to_unit() {
    let value = 5.6;
    let new_value = value * 4.184;

    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(new_value);
    let energy_kcal_to_kj = energy_kcal.to_unit(EnergyUnit::Kilojoule);

    print!(
        "energy_kj1: {},\nenergy_kj2: {}",
        energy_kj, energy_kcal_to_kj
    );
    assert_eq!(energy_kj, energy_kcal_to_kj);
}

#[test]
fn test_energy_to_fn() {
    let value = 6.9;
    let new_value = value * 4.184;

    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(new_value);
    let energy_kcal_to_kj = energy_kcal.to_kj();

    print!(
        "energy_kj1: {},\nenergy_kj2: {}",
        energy_kj, energy_kcal_to_kj
    );
    assert_eq!(energy_kj, energy_kcal_to_kj);
}

#[test]
fn test_energy_is_zero() {
    let zero_energy = EnergyQuantity::from_kcal(0f64);
    let energy = EnergyQuantity::from_kcal(5.5);

    assert!(zero_energy.is_zero());
    assert!(!energy.is_zero());
}

#[test]
fn test_energy_is_negative() {
    let negative_energy = EnergyQuantity::from_kcal(-5.5);
    let energy = EnergyQuantity::from_kcal(5.5);

    assert!(negative_energy.is_negative());
    assert!(!energy.is_negative());
}

#[test]
fn test_energy_get_value() {
    let energy = EnergyQuantity::new(6.882, EnergyUnit::Kilojoule);
    assert_eq!(energy.get_value(), 6.882);
}

#[test]
fn test_energy_set_value() {
    let mut energy = EnergyQuantity::new(6.882, EnergyUnit::Kilojoule);
    energy.set_value(8.92);
    assert_eq!(energy.get_value(), 8.92);
}

#[test]
fn test_energy_get_unit() {
    let energy = EnergyQuantity::new(6.882, EnergyUnit::Kilojoule);
    assert_eq!(energy.get_unit(), EnergyUnit::Kilojoule);
}

#[test]
fn test_energy_set_unit() {
    let mut energy = EnergyQuantity::new(6.882, EnergyUnit::Kilojoule);
    energy.set_unit(EnergyUnit::Kilojoule);
    assert_eq!(energy.get_unit(), EnergyUnit::Kilojoule);
}

#[test]
fn test_energy_get_symbol() {
    let value = 4.2;
    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(value);

    assert_eq!(energy_kcal.get_symbol(), "kcal");
    assert_eq!(energy_kj.get_symbol(), "kj");
}

#[test]
fn test_energy_get_measurement_system() {
    let value = 4.2;

    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(value);

    assert_eq!(
        energy_kcal.get_measurement_system(),
        MeasurementSystem::Metric
    );
    assert_eq!(
        energy_kj.get_measurement_system(),
        MeasurementSystem::Metric
    );
}

#[test]
fn test_energy_unit_type() {
    let value = 4.2;
    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(value);

    assert_eq!(energy_kcal.get_unit_type(), "kilocalorie");
    assert_eq!(energy_kj.get_unit_type(), "kilojoule");
}

#[test]
fn test_energy_unit_type_plural() {
    let value = 8.52;
    let energy_kcal = EnergyQuantity::from_kcal(value);
    let energy_kj = EnergyQuantity::from_kj(value);

    assert_eq!(energy_kcal.get_unit_type_plural(), "kilocalories");
    assert_eq!(energy_kj.get_unit_type_plural(), "kilojoules");
}

#[test]
fn test_energy_to_string() {
    let value_1 = 5f64;
    let value_2 = 8.642;

    let energy_kcal_1 = EnergyQuantity::from_kcal(value_1);
    assert_eq!(energy_kcal_1.to_string(), "5kcal");
    let energy_kj_1 = EnergyQuantity::from_kj(value_1);
    assert_eq!(energy_kj_1.to_string(), "5kj");

    let energy_kcal_2 = EnergyQuantity::from_kcal(value_2);
    assert_eq!(energy_kcal_2.to_string(), "8.642kcal");
    let energy_kj_2 = EnergyQuantity::from_kj(value_2);
    assert_eq!(energy_kj_2.to_string(), "8.642kj");
}

#[test]
fn test_energy_add() {
    let energy_kcal_1 = EnergyQuantity::from_kcal(100f64);
    let energy_kcal_2 = EnergyQuantity::from_kcal(500f64);
    let energy_kj = EnergyQuantity::from_kj(200f64);

    let energy_kcal_1_plus_kcal_2 = EnergyQuantity::from_kcal(600f64);
    let energy_kj_plus_kcal_1 = EnergyQuantity::from_kj(618.4);
    let energy_kcal_2_plus_kj = EnergyQuantity::from_kcal(547.8011472275334);

    assert_eq!((energy_kcal_1 + energy_kcal_2), energy_kcal_1_plus_kcal_2);
    assert_eq!((energy_kj + energy_kcal_1), energy_kj_plus_kcal_1);
    assert_eq!((energy_kcal_2 + energy_kj), energy_kcal_2_plus_kj);
}

#[test]
fn test_energy_subtract() {
    let energy_kcal_1 = EnergyQuantity::from_kcal(6f64);
    let energy_kcal_2 = EnergyQuantity::from_kcal(4f64);
    let energy_kj = EnergyQuantity::from_kj(1f64);

    let energy_g_1_minus_g_2 = EnergyQuantity::from_kcal(2f64);
    let energy_kj_minus_kcal_1 = EnergyQuantity::from_kj(-24.104);
    let energy_kcal_2_minus_kj = EnergyQuantity::from_kcal(3.7609942638623326);

    assert_eq!((energy_kcal_1 - energy_kcal_2), energy_g_1_minus_g_2);
    assert_eq!((energy_kj - energy_kcal_1), energy_kj_minus_kcal_1);
    assert_eq!((energy_kcal_2 - energy_kj), energy_kcal_2_minus_kj);
}

#[test]
fn test_energy_multiply() {
    let energy_kcal_1 = EnergyQuantity::from_kcal(70f64);
    let energy_kcal_2 = EnergyQuantity::from_kcal(350f64);
    let energy_g_3 = EnergyQuantity::from_kcal(267.4f64);

    assert_eq!((energy_kcal_1 * 5), energy_kcal_2);
    assert_eq!((energy_kcal_1 * 3.82), energy_g_3);
}

#[test]
fn test_energy_divide() {
    let energy_kcal_1 = EnergyQuantity::from_kcal(350f64);
    let energy_kcal_2 = EnergyQuantity::from_kcal(70f64);

    assert_eq!((energy_kcal_1 / 5), energy_kcal_2);
}

#[test]
fn test_energy_sum() {
    let energy_1 = EnergyQuantity::from_kcal(30f64);
    let energy_2 = EnergyQuantity::from_kcal(20f64);
    let energy_3 = EnergyQuantity::from_kcal(50f64).to_kj();
    let energy_4 = EnergyQuantity::from_kcal(20f64).to_kj();
    let energy_5 = EnergyQuantity::from_kcal(130f64).to_kj();
    let energy_total = EnergyQuantity::from_kcal(250f64);

    let energies = vec![energy_1, energy_2, energy_3, energy_4, energy_5];

    let sum: EnergyQuantity = energies.iter().map(|energy| *energy * 2).sum();
    assert_eq!(sum.get_unit(), energy_5.get_unit());
    assert_eq!(sum, (energy_total * 2).to_unit(energy_5.get_unit()));
}

#[test]
fn test_energy_partial_order() {
    let energy_kcal_1 = EnergyQuantity::from_kcal(6700f64);
    let energy_kcal_2 = EnergyQuantity::from_kcal(4700f64);
    let energy_kj = EnergyQuantity::from_kj(20920f64);
    assert!(energy_kcal_1 > energy_kcal_2);
    assert!(energy_kcal_1 > energy_kj);
    assert!(energy_kj > energy_kcal_2);
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();

    let _ = EnergyUnit::save_enumerations_to_database(&pool).await;

    let energy_kcal = EnergyQuantity::from_kcal(6700f64);
    let energy_record = Entity::new(energy_kcal);

    let res = energy_record.save_to_database(&pool).await;
    assert!(res.is_ok());

    let energy_saved =
        EnergyQuantity::get_from_database_using_id(energy_record.get_id(), &pool).await;
    assert!(energy_saved.is_ok());
    assert_eq!(energy_saved.unwrap(), energy_record);

    let res = energy_record.delete_from_database_using_id(&pool).await;
    assert!(res.is_ok());

    let energy_saved_2 =
        EnergyQuantity::get_from_database_using_id(energy_record.get_id(), &pool).await;
    assert!(energy_saved_2.is_err());
}
