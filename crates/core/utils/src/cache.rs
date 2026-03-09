use serde::{de::DeserializeOwned, Serialize};
use sled::Db;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct CacheService {
    db: Db,
}

impl CacheService {
    pub fn new() -> sled::Result<Self> {
        let workspace_root = std::env::var("WORKSPACE_ROOT").expect("WORKSPACE_ROOT must be set");
        let workspace_pathbuf = PathBuf::from(workspace_root);
        let db_location = workspace_pathbuf.join("sled");
        let db = sled::open(db_location)?;

        Ok(Self { db })
    }

    pub fn from_db(db: Db) -> Self {
        Self { db }
    }

    pub fn store_key_value<K, V>(
        &self,
        key: K,
        value: V,
        duration_in_seconds: Option<u64>,
    ) -> sled::Result<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let serialized_value =
            serde_json::to_vec(&value).map_err(|e| sled::Error::ReportableBug(e.to_string()))?;

        let expiry = match duration_in_seconds {
            Some(duration) => Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    + duration,
            ),
            None => None,
        };

        let stored_value = match expiry {
            Some(expiry) => {
                let mut buf = expiry.to_string().into_bytes();
                buf.extend(b"|");
                buf.extend(serialized_value);
                buf
            }
            None => {
                let mut buf = b"0|".to_vec();
                buf.extend(serialized_value);
                buf
            }
        };

        self.db.insert(key, stored_value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get_value<K, V>(&self, key: K) -> sled::Result<Option<V>>
    where
        K: AsRef<[u8]> + Clone,
        V: DeserializeOwned,
    {
        match self.db.get(key.clone())? {
            Some(ivec) => {
                let bytes = ivec.as_ref();

                if let Some(pos) = bytes.iter().position(|&b| b == b'|') {
                    let expiry_str = std::str::from_utf8(&bytes[..pos]).unwrap_or("0");
                    let expiry: u64 = expiry_str.parse().unwrap_or(0);

                    if expiry > 0 {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        if now > expiry {
                            self.db.remove(key)?;
                            return Ok(None);
                        }
                    }

                    let value_bytes = &bytes[pos + 1..];
                    let value: V = serde_json::from_slice(value_bytes)
                        .map_err(|e| sled::Error::ReportableBug(e.to_string()))?;
                    return Ok(Some(value));
                } else {
                    return Ok(None);
                }
            }
            None => Ok(None),
        }
    }

    pub fn delete_key<K>(&self, key: K) -> sled::Result<()>
    where
        K: AsRef<[u8]>,
    {
        self.db.remove(key)?;
        Ok(())
    }

    pub fn remove_expired(&self) -> sled::Result<()> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut batch = sled::Batch::default();
        for item in self.db.iter() {
            let (key, value) = item?;

            if let Some(pos) = value.iter().position(|&b| b == b'|') {
                let expiry_str = std::str::from_utf8(&value[..pos]).unwrap_or("0");
                let expiry: u64 = expiry_str.parse().unwrap_or(0);

                if expiry > 0 && now > expiry {
                    batch.remove(key);
                }
            }
        }

        self.db.apply_batch(batch)?;
        self.db.flush()?;
        Ok(())
    }
}
