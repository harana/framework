// Harana Components - SQL Store Base Types

use sqlx::{Database, Encode, Type, pool::Pool};
use std::marker::PhantomData;

use crate::Entity;

pub struct SqlStore<T: Entity, DB: Database> {
    pool: Pool<DB>,
    table_name: String,
    _phantom: PhantomData<T>,
}

impl<T: Entity, DB: Database> SqlStore<T, DB>
where
    for<'q> String: Encode<'q, DB> + Type<DB>,
    for<'q> &'q str: Encode<'q, DB>,
{
    pub fn new(pool: Pool<DB>, table_name: impl Into<String>) -> Self {
        Self {
            pool,
            table_name: table_name.into(),
            _phantom: PhantomData,
        }
    }

    pub fn with_entity_type(pool: Pool<DB>) -> Self {
        Self::new(pool, T::entity_type())
    }

    pub fn pool(&self) -> &Pool<DB> {
        &self.pool
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    #[allow(dead_code)]
    fn id_column() -> &'static str {
        "id"
    }
}
