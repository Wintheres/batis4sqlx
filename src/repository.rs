use crate::Entity;
use crate::wrapper::SqlValue;
use sqlx::mysql::{MySqlArguments, MySqlRow};
use sqlx::query::{Query, QueryAs, QueryScalar};
use sqlx::{MySql, MySqlPool};

pub trait MySqlRepository<
    E: Entity + for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + serde::Serialize,
>
{
    fn borrow_db(&self) -> &MySqlPool;
}

pub fn bind_query<'q>(
    mut query: Query<'q, MySql, MySqlArguments>,
    values: &Vec<SqlValue>,
) -> Query<'q, MySql, MySqlArguments> {
    for value in values {
        query = match value {
            SqlValue::ISize(value) => query.bind(*value as i64),
            SqlValue::USize(value) => query.bind(*value as u64),
            SqlValue::I8(value) => query.bind(*value),
            SqlValue::U8(value) => query.bind(*value),
            SqlValue::I16(value) => query.bind(*value),
            SqlValue::U16(value) => query.bind(*value),
            SqlValue::I32(value) => query.bind(*value),
            SqlValue::U32(value) => query.bind(*value),
            SqlValue::I64(value) => query.bind(*value),
            SqlValue::U64(value) => query.bind(*value),
            SqlValue::F32(value) => query.bind(*value),
            SqlValue::F64(value) => query.bind(*value),
            SqlValue::Bool(value) => query.bind(*value),
            SqlValue::Str(value) => query.bind(value.clone()),
            SqlValue::Time(value) => query.bind(*value),
            SqlValue::Date(value) => query.bind(*value),
            SqlValue::DateTime(value) => query.bind(*value),
            SqlValue::Decimal(value) => query.bind(*value),
            &SqlValue::Null => query.bind(None::<&str>),
        }
    }
    query
}

pub fn bind_query_as<'q, E: Entity>(
    mut query_as: QueryAs<'q, MySql, E, MySqlArguments>,
    values: &Vec<SqlValue>,
) -> QueryAs<'q, MySql, E, MySqlArguments> {
    for value in values {
        query_as = match value {
            SqlValue::ISize(value) => query_as.bind(*value as i64),
            SqlValue::USize(value) => query_as.bind(*value as u64),
            SqlValue::I8(value) => query_as.bind(*value),
            SqlValue::U8(value) => query_as.bind(*value),
            SqlValue::I16(value) => query_as.bind(*value),
            SqlValue::U16(value) => query_as.bind(*value),
            SqlValue::I32(value) => query_as.bind(*value),
            SqlValue::U32(value) => query_as.bind(*value),
            SqlValue::I64(value) => query_as.bind(*value),
            SqlValue::U64(value) => query_as.bind(*value),
            SqlValue::F32(value) => query_as.bind(*value),
            SqlValue::F64(value) => query_as.bind(*value),
            SqlValue::Bool(value) => query_as.bind(*value),
            SqlValue::Str(value) => query_as.bind(value.clone()),
            SqlValue::Time(value) => query_as.bind(*value),
            SqlValue::Date(value) => query_as.bind(*value),
            SqlValue::DateTime(value) => query_as.bind(*value),
            SqlValue::Decimal(value) => query_as.bind(*value),
            &SqlValue::Null => query_as.bind(None::<&str>),
        }
    }
    query_as
}

pub fn bind_query_scalar<'q, T>(
    mut query_scalar: QueryScalar<'q, MySql, T, MySqlArguments>,
    values: &Vec<SqlValue>,
) -> QueryScalar<'q, MySql, T, MySqlArguments> {
    for value in values {
        query_scalar = match value {
            SqlValue::ISize(value) => query_scalar.bind(*value as i64),
            SqlValue::USize(value) => query_scalar.bind(*value as u64),
            SqlValue::I8(value) => query_scalar.bind(*value),
            SqlValue::U8(value) => query_scalar.bind(*value),
            SqlValue::I16(value) => query_scalar.bind(*value),
            SqlValue::U16(value) => query_scalar.bind(*value),
            SqlValue::I32(value) => query_scalar.bind(*value),
            SqlValue::U32(value) => query_scalar.bind(*value),
            SqlValue::I64(value) => query_scalar.bind(*value),
            SqlValue::U64(value) => query_scalar.bind(*value),
            SqlValue::F32(value) => query_scalar.bind(*value),
            SqlValue::F64(value) => query_scalar.bind(*value),
            SqlValue::Bool(value) => query_scalar.bind(*value),
            SqlValue::Str(value) => query_scalar.bind(value.clone()),
            SqlValue::Time(value) => query_scalar.bind(*value),
            SqlValue::Date(value) => query_scalar.bind(*value),
            SqlValue::DateTime(value) => query_scalar.bind(*value),
            SqlValue::Decimal(value) => query_scalar.bind(*value),
            &SqlValue::Null => query_scalar.bind(None::<&str>),
        }
    }
    query_scalar
}
