use std::marker::PhantomData;

use crate::persistence::postgres::Db;

pub mod user;

pub struct DatabaseRepositoryImpl<T> {
    db: Db,
    _marker: PhantomData<T>,
}

impl<T> DatabaseRepositoryImpl<T> {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            _marker: PhantomData,
        }
    }
}
