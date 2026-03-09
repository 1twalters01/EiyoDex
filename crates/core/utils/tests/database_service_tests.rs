use utils::database::DatabaseService;

#[tokio::test]
async fn test_create_database_session() {
    let database_service = DatabaseService::new().await.expect("Failed to open sqlite");
    let res = database_service.ping().await;
    assert!(res.is_ok());
}
