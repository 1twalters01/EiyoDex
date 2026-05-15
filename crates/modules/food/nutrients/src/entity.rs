use std::cmp::Ordering;

use identity::{inner_id::InnerIdType, Id};

#[derive(Debug, Clone)]
pub struct Entity<T: Clone> {
    id: Id<T>,
    inner: T,
}

impl<T: Clone> Entity<T> {
    pub fn new(inner: T) -> Entity<T> {
        Self {
            id: Id::<T>::new(InnerIdType::Uuid),
            inner: inner,
        }
    }

    pub fn new_with_id(id: Id<T>, inner: T) -> Entity<T> {
        Self { id, inner }
    }

    pub fn get_id(&self) -> Id<T> {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: Id<T>) {
        self.id = id;
    }

    pub fn get_inner(&self) -> T {
        self.inner.clone()
    }

    pub fn set_inner(&mut self, inner: T) {
        self.inner = inner;
    }
}

impl<T: Clone + PartialEq> PartialEq for Entity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Clone + PartialEq + Eq> Eq for Entity<T> where Id<T>: Eq {}

impl<T: Clone + PartialEq> PartialOrd for Entity<T> 
where 
    Id<T>: PartialOrd 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T: Clone + PartialEq + Eq> Ord for Entity<T>
where 
    Id<T>: PartialOrd
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
