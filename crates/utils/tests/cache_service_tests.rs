use serde::{de::DeserializeOwned, Serialize};
use sled::Db;
use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
    thread::sleep,
};

#[test]
fn test_create_db_connection() {
    let cache = CacheService::new().expect("failed to open sled");

    cache
        .store_key_value("ping", "pong", None)
        .expect("insert failed");
    let result: Option<String> = cache.get_value("ping").expect("read failed");

    assert_eq!(result, Some("pong".to_string()));
    cache.delete_key("ping").expect("delete failed");
}

#[test]
fn test_new_cache_service() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let cache_service = CacheService::from_db(db);

    let key = "test_new_cache_service";
    let value = 12.6f64;
    let duration_in_seconds = None;

    let _ = cache_service.store_key_value(key, value, duration_in_seconds);
    let mut res: sled::Result<Option<f64>> = cache_service.get_value(key);
    assert_eq!(Some(value), res.unwrap());

    let _ = cache_service.delete_key(key);
    res = cache_service.get_value(key);
    assert_eq!(None, res.unwrap());
}

#[test]
fn test_from_cache_service() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let cache_service = CacheService::from_db(db);

    let key = "test_from_cache_service";
    let value = "value";
    let duration_in_seconds = Some(1);

    let _ = cache_service.store_key_value::<&str, &str>(key, value, duration_in_seconds);
    let mut res = cache_service.get_value(key);
    assert_eq!(Some(value.to_string()), res.unwrap());

    sleep(Duration::from_secs(2));

    res = cache_service.get_value(key);
    assert_eq!(None, res.unwrap());
}

#[test]
fn test_clean_db() {
    let db = sled::Config::new().temporary(true).open().unwrap();
    let cache_service = CacheService::from_db(db);
    cache_service.remove_expired().unwrap();
}