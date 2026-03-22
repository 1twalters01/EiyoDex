use std::{
    marker::PhantomData,
    str::FromStr
};

use crate::inner_id{InnerId, InnerIdExt, new_inner};


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<T> {
    inner: InnerId,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            inner: new_inner(),
            _marker: PhantomData,
        }
    }

    pub const fn from_inner(inner: InnerId) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> InnerId {
        self.inner
    }
    pub fn get_inner(&self) -> InnerId {
        self.inner
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        self.inner.to_bytes()
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self::from_inner(InnerId::from_bytes(bytes))
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, &'static str> {
        if slice.len() != 16 {
            return Err("Slice length must be 16 bytes");
        }

        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(slice);
        Ok(Self::from_bytes(bytes))
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> From<InnerId> for Id<T> {
    fn from(inner: InnerId) -> Self {
        Self::from_inner(inner)
    }
}

impl<T> From<Id<T>> for InnerId {
    fn from(id: Id<T>) -> Self {
        id.inner
    }
}

impl<T> FromStr for Id<T> {
    type Err = <InnerId as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_inner(s.parse()?))
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<T> Serialize for Id<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.inner.serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for Id<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let inner = InnerId::deserialize(deserializer)?;
            Ok(Self::from_inner(inner))
        }
    }
}