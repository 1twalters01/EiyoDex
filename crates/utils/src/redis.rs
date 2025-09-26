use redis::{Commands, Connection, RedisResult, ToRedisArgs};
// use dotenv::dotenv;
use std::{env, path::PathBuf};

/// Create connection pool to redis
pub fn create_redis_client_connection() -> Connection {
    let workspace_root = std::env::var("WORKSPACE_ROOT").unwrap();
    let workspace_pathbuf = PathBuf::from(workspace_root);
    let env_location = workspace_pathbuf.join(".env.example");
    dotenv::from_path(env_location).ok();

    let url: String = env::var("REDIS_URL").unwrap();
    println!("url: {}", url);
    let client = redis::Client::open(url).unwrap();
    let con = client.get_connection().unwrap();
    return con;
}

pub fn set_key_value_in_redis<K: ToRedisArgs, V: ToRedisArgs>(
    con: &mut Connection,
    key: K,
    value: V,
    expiry_in_seconds: Option<i64>,
) -> RedisResult<()> {
    let _: () = con.set(&key, &value)?;

    if let Some(expiry) = expiry_in_seconds {
        let _: () = con.expire(key, expiry)?;
    }

    Ok(())
}

pub fn get_value_from_key_in_redis(con: &mut Connection, key: &str) -> RedisResult<String> {
    con.get(key)
}

pub fn delete_key_in_redis(con: &mut Connection, key: &str) -> RedisResult<()> {
    con.del(key)
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
    fn test_expiring_key_value_in_redis() {
        dotenv().ok();

        let mut con = create_redis_client_connection();
        let key = "test_expiring_key_value_in_redis";
        let value = 10.5f64;
        let expiry_in_seconds = 1;
        let _ = set_key_value_in_redis(&mut con, key, value, Some(expiry_in_seconds));

        let res = get_value_from_key_in_redis(&mut con, key);
        assert_eq!(Some(value.to_string()), res.ok());
        sleep(Duration::from_secs(2));

        let res = get_value_from_key_in_redis(&mut con, key);
        assert_eq!(None, res.ok());       
    }

    #[test]
    fn test_delete_key_in_redis() {
        dotenv().ok();

        let mut con = create_redis_client_connection();
        let key = "test_delete_key_in_redis";
        let value = 8.6f64;
        let _ = set_key_value_in_redis(&mut con, key, value, None);

        let res = get_value_from_key_in_redis(&mut con, key);
        assert_eq!(Some(value.to_string()), res.ok());

        let _ = delete_key_in_redis(&mut con, key);

        let res2 = get_value_from_key_in_redis(&mut con, key);
        assert_eq!(None, res2.ok());
    }

}
