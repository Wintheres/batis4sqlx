use crate::Result;
use crate::wrapper::{Bracket, Order, SqlValue, Where, Wrapper};
use crate::{Entity, LambdaField};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, MySql, MySqlPool};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

pub struct QueryWrapper<'a, 'b, 'c, 'd, E>
where
    E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    field: Vec<&'a str>,
    wheres: Vec<Where<'b>>,
    or_index: HashSet<usize>,
    bracket: Bracket,
    order: Vec<Order<'c>>,
    first: Option<&'a str>,
    last: Option<&'a str>,
    comment: Option<&'a str>,
    db: &'d MySqlPool,
    _ignore: PhantomData<E>,
}

impl<'a, 'b, 'c, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin>
    QueryWrapper<'a, 'b, 'c, 'd, E>
{
}

impl<'a, 'b, 'c, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin>
    QueryWrapper<'a, 'b, 'c, 'd, E>
{
    pub fn new(db: &'d MySqlPool) -> Self {
        Self {
            field: vec![],
            wheres: vec![],
            or_index: HashSet::new(),
            bracket: Bracket::new(),
            order: vec![],
            first: None,
            last: None,
            comment: None,
            db,
            _ignore: Default::default(),
        }
    }

    pub fn order_asc<F>(self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'c>,
    {
        self.order_asc_field(field_func().0)
    }

    pub fn order_desc<F>(self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'c>,
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

    pub fn order_asc_field(mut self, field: &'c str) -> Self {
        self.order.push(Order::new(field, true));
        self
    }

    pub fn order_desc_field(mut self, field: &'c str) -> Self {
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

impl<'a, 'b, 'c, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> Wrapper<'a, 'b>
    for QueryWrapper<'a, 'b, 'c, 'd, E>
{
    fn wheres(&self) -> &Vec<Where<'b>> {
        &self.wheres
    }

    fn wheres_push(&mut self, r#where: Where<'b>) {
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

pub struct UpdateWrapper<'a, 'b, 'd, E>
where
    E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    set: HashMap<&'a str, SqlValue>,
    wheres: Vec<Where<'b>>,
    or_index: HashSet<usize>,
    bracket: Bracket,
    first: Option<&'a str>,
    last: Option<&'a str>,
    comment: Option<&'a str>,
    db: &'d MySqlPool,
    _ignore: PhantomData<E>,
}

impl<'a, 'b, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin>
    UpdateWrapper<'a, 'b, 'd, E>
{
    pub fn new(db: &'d MySqlPool) -> Self {
        Self {
            set: HashMap::new(),
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

    pub fn set<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Copy,
    {
        self.set.insert(field, value.into());
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
            .set
            .iter()
            .map(|(key, _value)| format!("{key} = ?"))
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
        Ok(self
            .bind_query(sqlx::query(&sql))
            .execute(self.db)
            .await?
            .rows_affected())
    }
}

impl<'a, 'b, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> Wrapper<'a, 'b>
    for UpdateWrapper<'a, 'b, 'd, E>
{
    fn wheres(&self) -> &Vec<Where<'b>> {
        &self.wheres
    }

    fn wheres_push(&mut self, r#where: Where<'b>) {
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

pub struct DeleteWrapper<'a, 'b, 'd, E>
where
    E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
{
    wheres: Vec<Where<'b>>,
    or_index: HashSet<usize>,
    bracket: Bracket,
    first: Option<&'a str>,
    last: Option<&'a str>,
    comment: Option<&'a str>,
    db: &'d MySqlPool,
    _ignore: PhantomData<E>,
}

impl<'a, 'b, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin>
    DeleteWrapper<'a, 'b, 'd, E>
{
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

impl<'a, 'b, 'd, E: Entity + for<'r> FromRow<'r, MySqlRow> + Send + Unpin> Wrapper<'a, 'b>
    for DeleteWrapper<'a, 'b, 'd, E>
{
    fn wheres(&self) -> &Vec<Where<'b>> {
        &self.wheres
    }

    fn wheres_push(&mut self, r#where: Where<'b>) {
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
