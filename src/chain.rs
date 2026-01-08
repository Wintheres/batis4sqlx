use crate::Result;
use crate::repository::bind_query;
use crate::wrapper::{Bracket, GroupHaving, Order, SqlValue, Where, Wrapper};
use crate::{Entity, LambdaField};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySql, MySqlPool};
use std::collections::HashSet;
use std::marker::PhantomData;

pub struct QueryWrapper<'a, 'd, E>
where
    E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    field: Vec<&'a str>,
    wheres: Vec<Where<'a>>,
    or_index: HashSet<usize>,
    bracket: Bracket,
    group_having: GroupHaving<'a>,
    order: Vec<Order<'a>>,
    first: Option<&'a str>,
    last: Option<&'a str>,
    comment: Option<&'a str>,
    db: &'d MySqlPool,
    _ignore: PhantomData<E>,
}

impl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> QueryWrapper<'a, 'd, E> {
    pub fn new(db: &'d MySqlPool) -> Self {
        Self {
            field: vec![],
            wheres: vec![],
            or_index: HashSet::new(),
            bracket: Bracket::new(),
            group_having: GroupHaving::new(),
            order: vec![],
            first: None,
            last: None,
            comment: None,
            db,
            _ignore: Default::default(),
        }
    }

    pub fn group_by<F>(mut self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        self.group_having.field_push(field_func().0);
        self
    }

    pub fn group_by_flag<F>(mut self, field_func: F, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        if flag {
            self = self.group_by(field_func);
        }
        self
    }

    pub fn group_by_vec<F>(mut self, field_func_vec: Vec<F>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        for field_func in field_func_vec {
            self.group_having.field_push(field_func().0);
        }
        self
    }

    pub fn group_by_vec_flag<F>(mut self, field_func_vec: Vec<F>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        if flag {
            self = self.group_by_vec(field_func_vec);
        }
        self
    }

    pub fn having(mut self, having_sql: &'a str) -> Self {
        self.group_having.having(having_sql);
        self
    }

    pub fn having_flag(mut self, having_sql: &'a str, flag: bool) -> Self {
        if flag {
            self = self.having(having_sql);
        }
        self
    }

    pub fn having_values<V>(mut self, having_sql: &'a str, values: Vec<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        let group_having = &mut self.group_having;
        group_having.having(having_sql);
        for value in values {
            group_having.value_push(value.into());
        }
        self
    }

    pub fn having_values_flag<V>(mut self, having_sql: &'a str, values: Vec<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        if flag {
            self = self.having_values(having_sql, values);
        }
        self
    }

    pub fn having_values_opt<V>(mut self, having_sql: &'a str, values: Option<Vec<V>>) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        if let Some(values) = values {
            self = self.having_values(having_sql, values);
        }
        self
    }

    pub fn having_values_opt_flag<V>(
        mut self,
        having_sql: &'a str,
        values: Option<Vec<V>>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        if flag {
            self = self.having_values_opt(having_sql, values);
        }
        self
    }

    pub fn order_asc<F>(self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        self.order_asc_field(field_func().0)
    }

    pub fn order_desc<F>(self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        self.order_desc_field(field_func().0)
    }

    pub fn select<F>(mut self, field_func_vec: Vec<F>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
    {
        for f in field_func_vec {
            self.field.push(f().0);
        }
        self
    }

    pub fn order_asc_field(mut self, field: &'a str) -> Self {
        self.order.push(Order::new(field, true));
        self
    }

    pub fn order_desc_field(mut self, field: &'a str) -> Self {
        self.order.push(Order::new(field, false));
        self
    }

    pub fn select_field(mut self, field: &[&'a str]) -> Self {
        for &f in field {
            self.field.push(f);
        }
        self
    }

    pub fn sql(&mut self) -> String {
        let mut sql = String::new();
        if let Some(first) = self.first {
            sql += first;
            sql += " ";
        }
        sql += "SELECT ";
        if self.field.is_empty() {
            sql += "*";
        } else {
            sql += self
                .field
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(", ")
                .as_str();
        };
        sql += &format!(" FROM {}", E::table_name());
        sql += self.r#where().as_str();
        if !self.order.is_empty() {
            sql += " ORDER BY ";
            let orders: Vec<String> = self
                .order
                .iter()
                .map(|o| {
                    if o.asc_desc {
                        format!("{} ASC", o.field)
                    } else {
                        format!("{} DESC", o.field)
                    }
                })
                .collect();

            sql += &orders.join(", ");
        }
        if let Some(last) = self.last {
            sql += &format!(" {last}");
        }
        if let Some(comment) = self.comment {
            sql += &format!(" -- {comment}");
        }
        sql
    }

    async fn aggregation(mut self) -> Result<Option<i64>> {
        self.order.clear();
        let sql = self.sql();
        self.bind_query_scalar(sqlx::query_scalar::<MySql, i64>(&sql))
            .fetch_optional(self.db)
            .await
    }

    pub async fn exists(mut self) -> Result<bool> {
        self.field = vec!["1"];
        Ok(self.aggregation().await?.is_some())
    }

    pub async fn count(mut self) -> Result<Option<i64>> {
        self.field = vec!["COUNT(*)"];
        self.aggregation().await
    }

    pub async fn sum(mut self, field: &'a str) -> Result<Option<i64>> {
        let field = format!("SUM({})", field);
        self.field = vec![&field];
        self.aggregation().await
    }

    pub async fn max(mut self, field: &'a str) -> Result<Option<i64>> {
        let field = format!("MAX({})", field);
        self.field = vec![&field];
        self.aggregation().await
    }

    pub async fn min(mut self, field: &'a str) -> Result<Option<i64>> {
        let field = format!("MIN({})", field);
        self.field = vec![&field];
        self.aggregation().await
    }

    pub async fn vec(mut self) -> Result<Vec<E>> {
        let sql = self.sql();
        self.bind_query_as::<E>(sqlx::query_as(&sql))
            .fetch_all(self.db)
            .await
    }

    pub async fn opt(mut self) -> Result<Option<E>> {
        let sql = self.sql();
        self.bind_query_as::<E>(sqlx::query_as(&sql))
            .fetch_optional(self.db)
            .await
    }
}

impl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> Wrapper<'a>
    for QueryWrapper<'a, 'd, E>
{
    fn wheres(&self) -> &Vec<Where<'a>> {
        &self.wheres
    }

    fn wheres_push(&mut self, r#where: Where<'a>) {
        self.wheres.push(r#where);
    }

    fn or_index(&self) -> &HashSet<usize> {
        &self.or_index
    }

    fn or_index_insert(&mut self, index: usize) {
        self.or_index.insert(index);
    }

    fn bracket(&self) -> &Bracket {
        &self.bracket
    }

    fn bracket_mut(&mut self) -> &mut Bracket {
        &mut self.bracket
    }

    fn first(mut self, sql: &'a str) -> Self
    where
        Self: Sized,
    {
        self.first = Some(sql);
        self
    }

    fn last(mut self, sql: &'a str) -> Self
    where
        Self: Sized,
    {
        self.last = Some(sql);
        self
    }

    fn comment(mut self, comment: &'a str) -> Self
    where
        Self: Sized,
    {
        self.comment = Some(comment);
        self
    }
}

pub struct UpdateWrapper<'a, 'd, E>
where
    E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    set_key: Vec<&'a str>,
    set_value: Vec<SqlValue>,
    wheres: Vec<Where<'a>>,
    or_index: HashSet<usize>,
    bracket: Bracket,
    first: Option<&'a str>,
    last: Option<&'a str>,
    comment: Option<&'a str>,
    db: &'d MySqlPool,
    _ignore: PhantomData<E>,
}

impl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> UpdateWrapper<'a, 'd, E> {
    pub fn new(db: &'d MySqlPool) -> Self {
        Self {
            set_key: Vec::new(),
            set_value: Vec::new(),
            wheres: vec![],
            or_index: HashSet::new(),
            bracket: Bracket::new(),
            first: None,
            last: None,
            comment: None,
            db,
            _ignore: Default::default(),
        }
    }

    pub fn set<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
    {
        self.set_field(*field_func(), value)
    }

    pub fn set_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
    {
        if flag {
            self = self.set(field_func, value);
        }
        self
    }

    pub fn set_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
    {
        if let Some(value) = value {
            self = self.set_field(*field_func(), value);
        }
        self
    }

    pub fn set_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
    {
        if flag {
            self = self.set_opt(field_func, value);
        }
        self
    }

    pub fn set_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        self.set_key.push(field);
        self.set_value.push(value.into());
        self
    }

    pub fn set_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        if flag {
            self = self.set_field(field, value);
        }
        self
    }

    pub fn set_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        if let Some(value) = value {
            self = self.set_field(field, value);
        }
        self
    }

    pub fn set_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
    {
        if flag {
            self = self.set_field_opt(field, value);
        }
        self
    }

    pub fn sql(&mut self) -> String {
        let mut sql = String::new();
        if let Some(first) = self.first {
            sql += first;
            sql += " ";
        }
        sql += &format!("UPDATE {} SET ", E::table_name());
        sql += self
            .set_key
            .iter()
            .map(|key| format!("{key} = ?"))
            .collect::<Vec<_>>()
            .join(", ")
            .as_str();
        sql += self.r#where().as_str();
        if let Some(last) = self.last {
            sql += &format!(" {last}");
        }
        if let Some(comment) = self.comment {
            sql += &format!(" -- {comment}");
        }
        sql
    }

    pub async fn execute(mut self) -> Result<u64> {
        let sql = self.sql();
        let values = self
            .set_value
            .iter()
            .map(|value| value.clone())
            .collect::<Vec<_>>();
        Ok(self
            .bind_query(bind_query(sqlx::query(&sql), &values))
            .execute(self.db)
            .await?
            .rows_affected())
    }
}

impl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> Wrapper<'a>
    for UpdateWrapper<'a, 'd, E>
{
    fn wheres(&self) -> &Vec<Where<'a>> {
        &self.wheres
    }

    fn wheres_push(&mut self, r#where: Where<'a>) {
        self.wheres.push(r#where);
    }

    fn or_index(&self) -> &HashSet<usize> {
        &self.or_index
    }

    fn or_index_insert(&mut self, index: usize) {
        self.or_index.insert(index);
    }

    fn bracket(&self) -> &Bracket {
        &self.bracket
    }

    fn bracket_mut(&mut self) -> &mut Bracket {
        &mut self.bracket
    }

    fn first(mut self, sql: &'a str) -> Self
    where
        Self: Sized,
    {
        self.first = Some(sql);
        self
    }

    fn last(mut self, sql: &'a str) -> Self
    where
        Self: Sized,
    {
        self.last = Some(sql);
        self
    }

    fn comment(mut self, comment: &'a str) -> Self
    where
        Self: Sized,
    {
        self.comment = Some(comment);
        self
    }
}

pub struct DeleteWrapper<'a, 'd, E>
where
    E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    wheres: Vec<Where<'a>>,
    or_index: HashSet<usize>,
    bracket: Bracket,
    first: Option<&'a str>,
    last: Option<&'a str>,
    comment: Option<&'a str>,
    db: &'d MySqlPool,
    _ignore: PhantomData<E>,
}

impl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> DeleteWrapper<'a, 'd, E> {
    pub fn new(db: &'d MySqlPool) -> Self {
        Self {
            wheres: vec![],
            or_index: HashSet::new(),
            bracket: Bracket::new(),
            first: None,
            last: None,
            comment: None,
            db,
            _ignore: Default::default(),
        }
    }

    pub fn sql(&mut self) -> String {
        let mut sql = String::new();
        if let Some(first) = self.first {
            sql += first;
            sql += " ";
        }
        sql += &format!("DELETE FROM {}", E::table_name());
        sql += self.r#where().as_str();
        if let Some(last) = self.last {
            sql += &format!(" {last}");
        }
        if let Some(comment) = self.comment {
            sql += &format!(" -- {comment}");
        }
        sql
    }

    pub async fn execute(mut self) -> Result<u64> {
        let sql = self.sql();
        Ok(self
            .bind_query(sqlx::query(&sql))
            .execute(self.db)
            .await?
            .rows_affected())
    }
}

impl<'a, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> Wrapper<'a>
    for DeleteWrapper<'a, 'd, E>
{
    fn wheres(&self) -> &Vec<Where<'a>> {
        &self.wheres
    }

    fn wheres_push(&mut self, r#where: Where<'a>) {
        self.wheres.push(r#where);
    }

    fn or_index(&self) -> &HashSet<usize> {
        &self.or_index
    }

    fn or_index_insert(&mut self, index: usize) {
        self.or_index.insert(index);
    }

    fn bracket(&self) -> &Bracket {
        &self.bracket
    }

    fn bracket_mut(&mut self) -> &mut Bracket {
        &mut self.bracket
    }

    fn first(mut self, sql: &'a str) -> Self
    where
        Self: Sized,
    {
        self.first = Some(sql);
        self
    }

    fn last(mut self, sql: &'a str) -> Self
    where
        Self: Sized,
    {
        self.last = Some(sql);
        self
    }

    fn comment(mut self, comment: &'a str) -> Self
    where
        Self: Sized,
    {
        self.comment = Some(comment);
        self
    }
}
