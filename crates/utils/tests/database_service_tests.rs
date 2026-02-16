use utils::database::DatabaseService;

#[tokio::test]
async fn test_create_database_session() {
    let database = DatabaseService::new().await.expect("failed to open sqlite");
    let res = database.ping().await;
    assert!(res.is_ok());
}
