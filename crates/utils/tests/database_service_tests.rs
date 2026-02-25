use utils::database::DatabaseService;

#[tokio::test]
async fn test_create_database_session() {
    let database_service = DatabaseService::new().await.expect("Failed to open sqlite");
    let res = database_service.ping().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_run_migrations_temporary() {
    let db = "sqlite://file::memory:?cache=shared";
    let database_service = DatabaseService::from_url(db).await.expect("failed to open sqlite");

    let current_env = "dev";
    database_service.run_migrations(current_env).await.expect("Unable to run migrations");

    let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM nutrients_essentiality_types")
        .fetch_all(&database_service.pool)
        .await.expect("Unable to run select query");

    assert_eq!(rows.len(), 3); // we expect 3 enum values
    assert!(rows.iter().any(|(_, name)| name == "essential"));
}

#[tokio::test]
async fn test_run_migrations() {
    let database_service = DatabaseService::new().await.expect("failed to open sqlite");

    let current_env = "dev";
    database_service.run_migrations(current_env).await.expect("Unable to run migrations");

    let rows: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM nutrients_quantity_types")
        .fetch_all(&database_service.pool)
        .await.expect("Unable to run select query");

    assert_eq!(rows.len(), 3); // we expect 3 enum values
    assert!(rows.iter().any(|(_, name)| name == "macronutrient"));
}
