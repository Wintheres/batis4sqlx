pub extern crate batis4sqlx_macros;
use crate::wrapper::{SqlValue, Wrapper};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, FromRow, MySqlPool};
use std::ops::Deref;

pub mod chain;
pub mod repository;
pub mod wrapper;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Entity {
    fn table_name() -> &'static str;

    fn primary_key() -> &'static str;
}

pub struct LambdaField<'a>(&'a str);

impl<'a> LambdaField<'a> {
    pub fn new(field: &'a str) -> Self {
        Self(field)
    }
}

impl<'a> Deref for LambdaField<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ServiceImpl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> {
    fn borrow_db(&self) -> &MySqlPool;

    fn lambda_query(&'d self) -> chain::QueryWrapper<'a, 'd, E> {
        chain::QueryWrapper::<E>::new(self.borrow_db())
    }

    fn lambda_update(&'d self) -> chain::UpdateWrapper<'a, 'd, E> {
        chain::UpdateWrapper::<E>::new(self.borrow_db())
    }

    fn lambda_delete(&'d self) -> chain::RemoveWrapper<'a, 'd, E> {
        chain::RemoveWrapper::<E>::new(self.borrow_db())
    }

    fn get_by_primary_key<K>(&'d self, primary_key: K) -> impl Future<Output = Result<Option<E>>>
    where
        K: Into<SqlValue> + Copy,
    {
        self.lambda_query()
            .eq_field(E::primary_key(), primary_key)
            .last("LIMIT 1")
            .opt()
    }

    fn vec(&'d self) -> impl Future<Output = Result<Vec<E>>> {
        self.lambda_query().vec()
    }
}
