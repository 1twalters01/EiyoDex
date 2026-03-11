use sqlx::{Pool, Sqlite};
use std::{future::Future, marker::PhantomData};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Id<T: Clone + PartialEq> {
    uuid: Uuid,
    _marker: PhantomData<T>,
}

impl<T: Clone + PartialEq> Id<T> {
    pub fn new(_type: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            _marker: PhantomData::<T>,
        }
    }

    pub fn from_uuid(uuid: Uuid, _type: T) -> Self {
        Self {
            uuid,
            _marker: PhantomData::<T>,
        }
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    pub fn set_uuid(&mut self, value: Uuid) {
        self.uuid = value;
    }
}

pub trait SaveToDatabase {
    fn save_to_database<'a>(&'a self, uuid: Uuid, pool: &'a Pool<Sqlite>) -> impl Future<Output = Result<(), sqlx::Error>> + Send + 'a;
}

pub trait GetFromDatabaseUsingId<T: Clone + PartialEq> {
    fn get_from_database_using_id<'a>(
        uuid: Uuid,
        pool: &'a Pool<Sqlite>,
    ) -> impl Future<Output = Result<Record<T>, sqlx::Error>> + Send + 'a;
}

pub trait DeleteFromDatabaseUsingId {
    fn delete_from_database_using_id<'a>(
        uuid: Uuid,
        pool: &'a Pool<Sqlite>,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send + 'a;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record<T: Clone + PartialEq> {
    pub id: Id<T>,
    pub inner: T,
}

impl<T: Clone + PartialEq> Record<T> {
    pub fn new(inner: T) -> Record<T> {
        Self {
            id: Id::new(inner.clone()),
            inner: inner,
        }
    }

    pub fn new_with_id(id: Id<T>, inner: T) -> Record<T> {
        Self { id, inner }
    }

    pub fn get_id(&self) -> Id<T> {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: Id<T>) {
        self.id = id;
    }

    pub fn get_uuid(&self) -> Uuid {
        self.id.get_uuid()
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.id.set_uuid(uuid);
    }

    pub fn get_inner(&self) -> T {
        self.inner.clone()
    }

    pub fn set_inner(&mut self, inner: T) {
        self.inner = inner;
    }
}

impl<T> Record<T>
where
    T: SaveToDatabase + Clone + PartialEq,
{
    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let uuid = self.get_uuid();
        self.inner.save_to_database(uuid, pool).await
    }
}

// impl<T> Record<T>
// where
//     T: GetFromDatabaseUsingId<T> + Clone + PartialEq, {
//     pub async fn get_from_database_using_id(&self, pool: &Pool<Sqlite>) ->
// Result<Record<T>, sqlx::Error> {         let uuid = self.get_uuid();
//         T::get_from_database_using_id(uuid, pool).await
//     }
// }

impl<T> Record<T>
where
    T: DeleteFromDatabaseUsingId + Clone + PartialEq,
{
    pub async fn delete_from_database_using_id(
        &self,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let uuid = self.get_uuid();
        T::delete_from_database_using_id(uuid, pool).await
    }
}
