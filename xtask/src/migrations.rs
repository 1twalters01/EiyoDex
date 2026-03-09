use std::fs;

use anyhow::Result;
use config::{config::get_workspace_pathbuf, database_config::DatabaseConfig};
use utils::database::DatabaseService;

#[derive(::serde::Deserialize)]
struct Migration {
    file: String,
    environments: Option<Vec<String>>,
}

#[derive(::serde::Deserialize)]
struct MigrationList {
    migration: Vec<Migration>,
}

pub async fn run_database_migrations(current_env: &str) -> Result<(), sqlx::Error> {
    let database_config = DatabaseConfig::default();

    let content = fs::read_to_string(database_config.get_migrations_path())
        .expect("Failed to read migrations.toml");
    let migrations: MigrationList = toml::from_str(&content).expect("Failed to parse TOML");

    for migration in migrations.migration {
        if let Some(envs) = &migration.environments {
            if !envs.contains(&current_env.to_string()) {
                continue;
            }
        }
        let sql_path = get_workspace_pathbuf().unwrap().join(&migration.file);
        let sql = fs::read_to_string(sql_path)
            .unwrap_or_else(|_| panic!("Failed to read migration file: {}", migration.file));

        let pool = DatabaseService::new().await.unwrap().pool;
        sqlx::query(&sql).execute(&pool).await?;
    }

    Ok(())
}
