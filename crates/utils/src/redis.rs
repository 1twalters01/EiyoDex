use redis::{cmd, Commands, Connection, RedisResult, ToRedisArgs};
// use dotenv::dotenv;
use std::{env, path::PathBuf};

pub struct CacheService {
    con: Connection,
}

impl CacheService {
    pub fn new() -> Self {
        let con = create_redis_client_connection();
        Self { con }
    }

    pub fn from_con(con: Connection) -> Self {
        Self { con }
    }

    pub fn store_key_and_value<K: ToRedisArgs, V: ToRedisArgs>(
        &mut self,
        key: K,
        value: K,
        expiry_in_seconds: Option<i64>,
    ) -> RedisResult<()> {
        let mut command = cmd("SET");
        command.arg(&key).arg(&value);

        if let Some(expiry) = expiry_in_seconds {
            command.arg("EX").arg(expiry);
        }

        command.query(&mut self.con)
    }

    pub fn get_value_as_string<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<String> {
        self.con.get(key)
    }

    pub fn delete_key<K: ToRedisArgs>(&mut self, key: K) -> RedisResult<()> {
        self.con.del(key)
    }
}

/// Create connection pool to redis
fn create_redis_client_connection() -> Connection {
    let workspace_root = std::env::var("WORKSPACE_ROOT").unwrap();
    let workspace_pathbuf = PathBuf::from(workspace_root);
    let env_location = workspace_pathbuf.join(".env.example");
    dotenv::from_path(env_location).ok();

    let url: String = env::var("REDIS_URL").unwrap();
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_create_redis_client_connection() {
        dotenv().ok();

        let mut con = create_redis_client_connection();
        let _: () = redis::cmd("PING").query(&mut con).unwrap();
    }

    #[test]
    fn test_new_cache_service() {
        let mut cache_service = CacheService::new();

        let key = "test_new_cache_service";
        let value = "value";
        let expiry_in_seconds = None;

        let _ = cache_service.store_key_and_value::<&str, &str>(key, value, expiry_in_seconds);
        let res = cache_service.get_value_as_string(key);
        assert_eq!(Some(value.to_string()), res.ok());

        let _ = cache_service.delete_key(key);
        let res2 = cache_service.get_value_as_string(key);
        assert_eq!(None, res2.ok());
    }

    #[test]
    fn test_from_cache_service() {
        let con = create_redis_client_connection();
        let mut cache_service = CacheService::from_con(con);

        let key = "test_from_cache_service";
        let value = "value";
        let expiry_in_seconds = Some(1);

        let _ = cache_service.store_key_and_value::<&str, &str>(key, value, expiry_in_seconds);
        let res = cache_service.get_value_as_string(key);
        assert_eq!(Some(value.to_string()), res.ok());

        sleep(Duration::from_secs(2));

        let res2 = cache_service.get_value_as_string(key);
        assert_eq!(None, res2.ok());
    }
}
