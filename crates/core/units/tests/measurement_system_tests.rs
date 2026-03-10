use std::collections::BTreeSet;
use units::measurement_system::MeasurementSystem;
use utils::database::DatabaseService;

#[test]
fn test_get_measurement_system_enumerations() {
    let function_enumerations = MeasurementSystem::get_enumerations();
    let manual_enumerations = &[
        MeasurementSystem::Metric,
        MeasurementSystem::Imperial,
        MeasurementSystem::SI,
    ];
    assert_eq!(
        BTreeSet::from_iter(function_enumerations),
        BTreeSet::from_iter(manual_enumerations)
    );
}

#[tokio::test]
async fn test_save_to_database() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = MeasurementSystem::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let measurement_system = MeasurementSystem::Metric.as_string();
    let rows = sqlx::query!(
        r#"
            SELECT name
            FROM units_measurement_systems
            WHERE name = ?
        "#,
        measurement_system
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let name = rows.name;

    assert!(name == "Metric")
}

#[tokio::test]
async fn test_get_database_id() {
    let database_service = DatabaseService::new().await.unwrap();
    let pool = database_service.get_pool();
    let res = MeasurementSystem::save_enumerations_to_database(&pool).await;
    assert!(res.is_ok());

    let metric = MeasurementSystem::Metric;
    let id = metric.get_database_id(&pool).await;
    assert!(id.is_ok());
}
