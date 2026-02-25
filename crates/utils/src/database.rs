use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
struct Migration {
    file: String,
    environments: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct MigrationList {
    migration: Vec<Migration>,
}

pub struct DatabaseService {
    pub pool: Pool<Sqlite>,
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

    pub async fn run_migrations(&self, current_env: &str) -> Result<(), sqlx::Error> {
        let workspace_root = std::env::var("WORKSPACE_ROOT").expect("WORKSPACE_ROOT must be set");
        let workspace_pathbuf = PathBuf::from(workspace_root);
        let content_path = workspace_pathbuf.join("migrations.toml");
        println!("content path: {:#?}", content_path);
        let content = fs::read_to_string(content_path).expect("Failed to read migrations.toml");
        let migrations: MigrationList = toml::from_str(&content).expect("Failed to parse TOML");

        for migration in migrations.migration {
            if let Some(envs) = &migration.environments {
                if !envs.contains(&current_env.to_string()) {
                    continue;
                }
            }
            let sql_path = workspace_pathbuf.join(&migration.file);
            let sql = fs::read_to_string(sql_path)
                .unwrap_or_else(|_| panic!("Failed to read migration file: {}", migration.file));
            sqlx::query(&sql)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}
