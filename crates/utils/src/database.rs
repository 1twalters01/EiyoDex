use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{fs, path::PathBuf};

pub struct DatabaseService {
    pub pool: Pool<Sqlite>,
}

impl DatabaseService {
    pub async fn from_url(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn new() -> Result<Self, sqlx::Error> {
        let workspace_root = std::env::var("WORKSPACE_ROOT").expect("WORKSPACE_ROOT must be set");
        let workspace_pathbuf = PathBuf::from(workspace_root);
        let database_path = workspace_pathbuf.join("eiyodex.sqlite");

        if !database_path.exists() {
            fs::File::create(&database_path).expect("failed to create sqlite file");
        }

        let database_url = format!("sqlite://{}", database_path.to_str().unwrap());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map(|_| ())
    }
}
