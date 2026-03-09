use std::{env, fs, path::PathBuf};

use crate::config::{get_workspace_pathbuf, init_env};

pub struct DatabaseConfig {
    database_url: String,
    database_pool_size: u32,
    database_migrations_path: PathBuf,
}

impl DatabaseConfig {
    pub fn get_database_url(&self) -> String {
        self.database_url.clone()
    }

    pub fn get_pool_size(&self) -> u32 {
        self.database_pool_size
    }

    pub fn get_migrations_path(&self) -> PathBuf {
        self.database_migrations_path.clone()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        init_env();

        let workspace_pathbuf = get_workspace_pathbuf().expect("Invalid workspace path buf");

        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => {
                // Prepend workspace root if it's SQLite with a relative path
                if url.starts_with("sqlite://") && !url.starts_with("sqlite:///") {
                    let relative_path = &url["sqlite://".len()..];
                    let absolute_path = workspace_pathbuf.join(relative_path);
                    format!("sqlite://{}", absolute_path.to_str().unwrap())
                } else {
                    url
                }
            }
            Err(_) => {
                let sqlite_file_path = workspace_pathbuf.join("eiyodex.sqlite");
                if !sqlite_file_path.exists() {
                    fs::File::create(&sqlite_file_path).expect("Failed to create sqlite file");
                }
                format!("sqlite://{}", sqlite_file_path.to_str().unwrap())
            }
        };

        let database_pool_size = env::var("DATABASE_POOL_SIZE")
            .unwrap_or_else(|_| String::from("5"))
            .parse::<u32>()
            .expect("Database pool size must be a number");

        let database_migrations_path = workspace_pathbuf.join("migrations.toml");

        Self {
            database_url,
            database_pool_size,
            database_migrations_path,
        }
    }
}
