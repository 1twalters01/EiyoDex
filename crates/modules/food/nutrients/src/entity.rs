use identity::{inner_id::InnerIdType, Id};

#[derive(Debug, Clone, PartialEq)]
pub struct Entity<T: Clone + PartialEq> {
    id: Id<T>,
    inner: T,
}

impl<T: Clone + PartialEq> Entity<T> {
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

