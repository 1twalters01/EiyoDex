use std::str::FromStr;

#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "ulid")]
use ulid::Ulid;

#[cfg(all(feature = "uuid", feature = "ulid"))]
compile_error!("Enable only one of: `uuid` or `ulid`");

#[cfg(not(any(feature = "uuid", feature = "ulid")))]
compile_error!("You must enable one of: `uuid` or `ulid`");


#[cfg(feature = "uuid")]
pub(crate) type InnerId = Uuid;

#[cfg(feature = "ulid")]
pub(crate) type InnerId = Ulid;

pub(crate) trait InnerIdExt: Sized {
    fn from_bytes(bytes: [u8;16]) -> Self;
    fn to_bytes(&self) -> [u8;16];
}

#[cfg(feature = "uuid")]
impl InnerIdExt for Uuid {
    fn from_bytes(bytes: [u8;16]) -> Self { Uuid::from_bytes(bytes) }
    fn to_bytes(&self) -> [u8;16] { *self.as_bytes() }
}

#[cfg(feature = "ulid")]
impl InnerIdExt for Ulid {
    fn from_bytes(bytes: [u8;16]) -> Self { Ulid::from_bytes(bytes) }
    fn to_bytes(&self) -> [u8;16] { self.to_bytes() }
}

pub(crate) fn new_inner() -> InnerId {
    #[cfg(feature = "uuid")]
    { Uuid::new_v4() }

    #[cfg(feature = "ilid")]
    { Ulid::new() }
}