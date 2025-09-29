use redis::{Commands, Connection, FromRedisValue, RedisResult, ToRedisArgs};
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

    pub fn store_key_value<K: ToRedisArgs, V: ToRedisArgs>(
        &mut self,
        key: K,
        value: V,
        duration_in_seconds: Option<i64>,
    ) -> RedisResult<()> {
        let _: () = self.con.set(&key, &value)?;

        if let Some(expiry) = duration_in_seconds {
            let _: () = self.con.expire(key, expiry)?;
        }

        Ok(())
    }

    pub fn get_value<K: ToRedisArgs, V: FromRedisValue>(&mut self, key: K) -> RedisResult<V> {
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
        dotenv().ok();

        let mut cache_service = CacheService::new();

        let key = "test_new_cache_service";
        let value = 12.6f64;
        let duration_in_seconds = None;

        let _ = cache_service.store_key_value(key, value, duration_in_seconds);
        let mut res: Result<String, redis::RedisError> = cache_service.get_value(key);
        assert_eq!(Some(value.to_string()), res.ok());

        let _ = cache_service.delete_key(key);
        res = cache_service.get_value(key);
        assert_eq!(None, res.ok());
    }

    #[test]
    fn test_from_cache_service() {
        dotenv().ok();

        let con = create_redis_client_connection();
        let mut cache_service = CacheService::from_con(con);

        let key = "test_from_cache_service";
        let value = "value";
        let duration_in_seconds = Some(1);

        let _ = cache_service.store_key_value::<&str, &str>(key, value, duration_in_seconds);
        let mut res: Result<String, redis::RedisError> = cache_service.get_value(key);
        assert_eq!(Some(value.to_string()), res.ok());

        sleep(Duration::from_secs(2));

        res = cache_service.get_value(key);
        assert_eq!(None, res.ok());
    }
}
