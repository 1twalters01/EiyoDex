use std::{collections::BTreeSet, str::FromStr};
use units::{energy::unit::EnergyUnit, measurement_system::MeasurementSystem};
use utils::database::DatabaseService;

#[test]
fn test_get_energy_unit_enumerations() {
    let function_enumerations = EnergyUnit::get_enumerations();
    let manual_enumerations = &vec![EnergyUnit::Kilojoule, EnergyUnit::Kilocalorie];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[test]
fn test_get_symbols() {
    assert_eq!(EnergyUnit::Kilojoule.get_symbol(), "kj");
    assert_eq!(EnergyUnit::Kilocalorie.get_symbol(), "kcal");
}

#[test]
fn test_get_unit_types() {
    assert_eq!(EnergyUnit::Kilojoule.get_unit_type(), "kilojoule");
    assert_eq!(EnergyUnit::Kilocalorie.get_unit_type(), "kilocalorie");
}

#[test]
fn test_get_plural_unit_types() {
    assert_eq!(EnergyUnit::Kilojoule.get_unit_type_plural(), "kilojoules");
    assert_eq!(
        EnergyUnit::Kilocalorie.get_unit_type_plural(),
        "kilocalories"
    );
}

#[test]
fn test_get_measurement_systems() {
    assert_eq!(
        EnergyUnit::Kilojoule.get_measurement_system(),
        MeasurementSystem::Metric,
    );
    assert_eq!(
        EnergyUnit::Kilocalorie.get_measurement_system(),
        MeasurementSystem::Metric,
    );
}

#[test]
fn test_get_si_factor() {
    assert_eq!(EnergyUnit::Kilojoule.si_factor(), 1000 as f64);
    assert_eq!(EnergyUnit::Kilocalorie.si_factor(), 4184f64);
}

#[test]
fn test_from_str() {
    assert_eq!(EnergyUnit::from_str("kj").unwrap(), EnergyUnit::Kilojoule);
    assert_eq!(
        EnergyUnit::from_str("kilojoule").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        EnergyUnit::from_str("Kilojoule").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        EnergyUnit::from_str("kilojoules").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_eq!(
        EnergyUnit::from_str("KilOJOUleS").unwrap(),
        EnergyUnit::Kilojoule
    );
    assert_ne!(EnergyUnit::from_str("kcal").unwrap(), EnergyUnit::Kilojoule);

    assert_eq!(
        EnergyUnit::from_str("kcal").unwrap(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        EnergyUnit::from_str("kilocalorie").unwrap(),
        EnergyUnit::Kilocalorie
    );
    assert_eq!(
        EnergyUnit::from_str("kilocalories").unwrap(),
        EnergyUnit::Kilocalorie
    );
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = EnergyUnit::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let energy = EnergyUnit::Kilocalorie.get_unit_type();
    let rows = sqlx::query!(
        r#"
            SELECT id, unit_type
            FROM units_energy_types
            WHERE unit_type = ?
        "#,
        energy
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let unit_type = rows.unit_type;

    assert_eq!(unit_type, "kilocalorie");
}

#[tokio::test]
async fn test_get_database_id() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = EnergyUnit::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let kj = EnergyUnit::Kilojoule;
    let id = kj.get_database_id(&pool).await;
    assert!(id.is_ok());
    assert_ne!(
        EnergyUnit::Kilojoule.get_database_id(&pool).await.unwrap(),
        EnergyUnit::Kilocalorie
            .get_database_id(&pool)
            .await
            .unwrap()
    );
}
