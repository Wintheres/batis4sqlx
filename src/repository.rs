use crate::Entity;
use crate::Result;
use sqlx::mysql::MySqlRow;
use sqlx::{Encode, Error, MySqlPool, Type};
use std::collections::HashSet;

trait MySqlRepository<
    E: Entity + for<'r> sqlx::FromRow<'r, MySqlRow> + Send + Unpin + serde::Serialize,
    T: for<'q> Encode<'q, sqlx::MySql> + Type<sqlx::MySql>,
>
{
    fn get_db(&self) -> &MySqlPool;

    async fn query() {}

    async fn vec(&self) -> Result<Vec<E>> {
        sqlx::query_as(&format!("SELECT * FROM {}", E::table_name()))
            .fetch_all(self.get_db())
            .await
    }

    async fn pagination(&self, offset: u32, size: u32) -> Result<Vec<E>> {
        sqlx::query_as(&format!("SELECT * FROM {} LIMIT ?, ?", E::table_name()))
            .bind(offset)
            .bind(size)
            .fetch_all(self.get_db())
            .await
    }

    async fn get_by_primary_key(&self, primary_key: T) -> Result<Option<E>> {
        sqlx::query_as(&format!(
            "SELECT * FROM {} WHERE `{}` = ? LIMIT 1",
            E::table_name(),
            E::primary_key()
        ))
        .bind(primary_key)
        .fetch_optional(self.get_db())
        .await
    }

    async fn save(&self, vo: &mut E) -> Result<u64> {
        let mut insert_sql = format!("INSERT INTO {} ", E::table_name());
        let json = serde_json::to_value(vo);
        if let Ok(json) = json {
            if let Some(obj) = json.as_object() {
                let mut key_set = HashSet::new();
                let mut value_set = HashSet::new();
                for key in obj.keys() {
                    if let Some(value) = json.get(key) {
                        if value.is_null() {
                            continue;
                        }
                        key_set.insert(key.as_str());
                        value_set.insert("?");
                    }
                }
                insert_sql += &format!(
                    " ({}) VALUES ({})",
                    key_set
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                    value_set
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                );
                let mut insert = sqlx::query(&insert_sql);
                for key in obj.keys() {
                    insert = insert.bind(json[key].to_string());
                }
                let result = insert.execute(self.get_db()).await?;
                Ok(result.last_insert_id())
            } else {
                Err(Error::InvalidArgument(
                    "serde_json as object failed.".to_string(),
                ))
            }
        } else {
            Err(Error::InvalidArgument(
                "serialization json failed.".to_string(),
            ))
        }
    }

    async fn update_by_primary_key(&self, vo: &E) -> Result<u64> {
        let json = serde_json::to_value(vo);
        if let Ok(json) = json {
            let primary_key = E::primary_key();
            let mut update_sql = format!("UPDATE {} SET", E::table_name());
            if let Some(obj) = json.as_object() {
                let mut key_set = HashSet::new();
                let mut value_set = HashSet::new();
                for key in obj.keys() {
                    if primary_key.eq(key) {
                        continue;
                    }
                    if let Some(value) = json.get(key) {
                        if value.is_null() {
                            continue;
                        }
                    }
                    key_set.insert(key.as_str());
                    value_set.insert(json[key].to_string());
                }
                update_sql += &key_set
                    .iter()
                    .map(|k| format!(" `{k}` = COALESCE(?, `{k}`)"))
                    .collect::<Vec<_>>()
                    .join(",");
                update_sql += &format!(" WHERE `{primary_key}` = ? LIMIT 1");
                let mut update = sqlx::query(&update_sql);
                for value in value_set {
                    update = update.bind(value);
                }
                Ok(update.execute(self.get_db()).await?.rows_affected())
            } else {
                Err(Error::InvalidArgument(
                    "serde_json as object failed.".to_string(),
                ))
            }
        } else {
            Err(Error::InvalidArgument(
                "serialization json failed.".to_string(),
            ))
        }
    }

    async fn delete_in_primary_keys(&self, primary_keys: HashSet<T>) -> Result<u64> {
        let placeholders = primary_keys
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "DELETE FROM {} WHERE `{}` IN ({placeholders})",
            E::table_name(),
            E::primary_key()
        );
        let mut delete = sqlx::query(&sql);
        for id in primary_keys {
            delete = delete.bind(id);
        }
        Ok(delete.execute(self.get_db()).await?.rows_affected())
    }
}
