use config::database_config::DatabaseConfig;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub struct DatabaseService {
    pool: Pool<Sqlite>,
}

impl DatabaseService {
    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    pub async fn from_url(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_config = DatabaseConfig::default();

        let pool = SqlitePoolOptions::new()
            .max_connections(database_config.get_pool_size())
            .connect(&database_config.get_database_url())
            .await?;

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> Pool<Sqlite> {
        self.pool.clone()
    }
}
