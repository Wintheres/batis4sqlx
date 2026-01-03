use sqlx::mysql::MySqlRow;
use sqlx::{Error, FromRow, MySqlPool};

pub mod chain;
pub mod repository;
pub mod wrapper;

type Result<T> = std::result::Result<T, Error>;

pub trait Entity {
    fn table_name() -> &'static str;

    fn primary_key() -> &'static str;
}

pub struct LambdaField<'b>(&'b str);

impl<'b> LambdaField<'b> {
    pub fn new(field: &'b str) -> Self {
        Self(field)
    }
}

pub trait ServiceImpl<'a, 'b, 'c, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> {
    fn get_db(&self) -> &MySqlPool;

    fn lambda_query(&'d self) -> chain::QueryWrapper<'a, 'b, 'c, 'd, E> {
        chain::QueryWrapper::<E>::new(self.get_db())
    }

    fn lambda_update(&'d self) -> chain::UpdateWrapper<'a, 'b, 'd, E> {
        chain::UpdateWrapper::<E>::new(self.get_db())
    }

    fn lambda_delete(&'d self) -> chain::DeleteWrapper<'a, 'b, 'd, E> {
        chain::DeleteWrapper::<E>::new(self.get_db())
    }
}
