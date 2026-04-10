use std::fmt;

use ulid::Ulid;
use uuid::Uuid;

pub enum InnerIdType {
    Uuid,
    Ulid,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum InnerId {
    Uuid(Uuid),
    Ulid(Ulid),
}

impl InnerId {
    pub fn new(id_type: InnerIdType) -> InnerId {
        match id_type {
            InnerIdType::Ulid => InnerId::Ulid(Ulid::new()),
            InnerIdType::Uuid => InnerId::Uuid(Uuid::new_v4()),
        }
    }

    pub fn from_bytes(id_type: InnerIdType, bytes: [u8; 16]) -> Self {
        match id_type {
            InnerIdType::Ulid => InnerId::Ulid(Ulid::from_bytes(bytes)),
            InnerIdType::Uuid => InnerId::Uuid(Uuid::from_bytes(bytes)),
        }
    }

    pub fn from_slice(id_type: InnerIdType, slice: &[u8]) -> Result<Self, &'static str> {
        if slice.len() != 16 {
            return Err("expected 16 bytes");
        }

        let array: [u8; 16] = slice.try_into()
            .map_err(|_| "failed to convert slice to array")?;

        Ok(Self::from_bytes(id_type, array))
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        match self {
            InnerId::Ulid(ulid) => ulid.to_bytes(),
            InnerId::Uuid(uuid) => *uuid.as_bytes(),
        }
    }
}

impl fmt::Debug for InnerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ulid(ulid) => ulid.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
        }
    }
}

impl fmt::Display for InnerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ulid(ulid) => ulid.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
        }
    }
}
