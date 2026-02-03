pub extern crate batis4sqlx_macros;
extern crate core;

use crate::wrapper::{SqlValue, Wrapper};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, FromRow, MySqlPool};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

pub mod chain;
pub mod repository;
pub mod wrapper;

pub type Result<T> = std::result::Result<T, Error>;

pub trait Entity {
    fn table_name() -> &'static str;

    fn primary_key<'b>() -> LambdaField<'b>;
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

impl<'a> Display for LambdaField<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    fn lambda_delete(&'d self) -> chain::DeleteWrapper<'a, 'd, E> {
        chain::DeleteWrapper::<E>::new(self.borrow_db())
    }

    fn get_by_primary_key<K>(&'d self, primary_key_value: K) -> impl Future<Output = Result<Option<E>>>
    where
        K: Into<SqlValue> + Clone,
    {
        self.lambda_query()
            .eq(E::primary_key, primary_key_value)
            .last("LIMIT 1")
            .opt()
    }

    fn vec(&'d self) -> impl Future<Output = Result<Vec<E>>> {
        self.lambda_query().vec()
    }
}
