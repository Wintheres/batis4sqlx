use crate::{
    Entity, LambdaField,
    repository::{bind_query, bind_query_as, bind_query_scalar},
};
use rust_decimal::Decimal;
use sqlx::MySql;
use sqlx::mysql::MySqlArguments;
use sqlx::query::{Query, QueryAs, QueryScalar};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

pub trait Wrapper<'a> {
    fn wheres(&self) -> &Vec<Where<'a>>;
    fn wheres_push(&mut self, r#where: Where<'a>);
    fn or_index(&self) -> &HashSet<usize>;
    fn or_index_insert(&mut self, index: usize);
    fn bracket(&self) -> &Bracket;
    fn bracket_mut(&mut self) -> &mut Bracket;

    fn eq<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.eq_field(*field_func(), value)
    }

    fn eq_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.eq(field_func, value);
        }
        self
    }

    fn eq_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.eq_field(*field_func(), value)
        }
        self
    }

    fn eq_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.eq_opt(field_func, value);
        }
        self
    }

    fn eq_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Eq, vec![value.into()]));
        self
    }

    fn eq_field_flag<F, V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.eq_field(field, value);
        }
        self
    }

    fn eq_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.eq_field(field, value);
        }
        self
    }

    fn eq_field_opt_flag<F, V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.eq_field_opt(field, value);
        }
        self
    }

    fn ne<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.ne_field(*field_func(), value)
    }

    fn ne_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ne(field_func, value);
        }
        self
    }

    fn ne_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.ne_field(*field_func(), value);
        }
        self
    }

    fn ne_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ne_opt(field_func, value);
        }
        self
    }

    fn ne_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Ne, vec![value.into()]));
        self
    }

    fn ne_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ne_field(field, value);
        }
        self
    }

    fn ne_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.ne_field(field, value);
        }
        self
    }

    fn ne_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ne_field_opt(field, value);
        }
        self
    }

    fn gt<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.gt_field(*field_func(), value)
    }

    fn gt_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.gt(field_func, value);
        }
        self
    }

    fn gt_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.gt_field(*field_func(), value);
        }
        self
    }

    fn gt_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.gt_opt(field_func, value);
        }
        self
    }

    fn gt_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Gt, vec![value.into()]));
        self
    }

    fn gt_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.gt_field(field, value);
        }
        self
    }

    fn gt_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.gt_field(field, value);
        }
        self
    }

    fn gt_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.gt_field_opt(field, value);
        }
        self
    }

    fn ge<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.ge_field(*field_func(), value)
    }

    fn ge_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ge(field_func, value);
        }
        self
    }

    fn ge_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.ge_field(*field_func(), value);
        }
        self
    }

    fn ge_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ge_opt(field_func, value);
        }
        self
    }

    fn ge_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Ge, vec![value.into()]));
        self
    }

    fn ge_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ge_field(field, value);
        }
        self
    }

    fn ge_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.ge_field(field, value);
        }
        self
    }

    fn ge_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.ge_field_opt(field, value);
        }
        self
    }

    fn lt<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.lt_field(*field_func(), value)
    }

    fn lt_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.lt(field_func, value);
        }
        self
    }

    fn lt_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.lt_field(*field_func(), value);
        }
        self
    }

    fn lt_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.lt_opt(field_func, value);
        }
        self
    }

    fn lt_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Lt, vec![value.into()]));
        self
    }

    fn lt_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.lt_field(field, value);
        }
        self
    }

    fn lt_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.lt_field(field, value);
        }
        self
    }

    fn lt_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.lt_field_opt(field, value);
        }
        self
    }

    fn le<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.le_field(*field_func(), value)
    }

    fn le_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.le(field_func, value);
        }
        self
    }

    fn le_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.le_field(*field_func(), value);
        }
        self
    }

    fn le_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.le_opt(field_func, value);
        }
        self
    }

    fn le_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Le, vec![value.into()]));
        self
    }

    fn le_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.le_field(field, value);
        }
        self
    }

    fn le_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.le_field(field, value);
        }
        self
    }

    fn le_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.le_field_opt(field, value);
        }
        self
    }

    fn between<F, V>(self, field_func: F, value_left: V, value_right: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.between_field(*field_func(), value_left, value_right)
    }

    fn between_flag<F, V>(
        mut self,
        field_func: F,
        value_left: V,
        value_right: V,
        flag: bool,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.between(field_func, value_left, value_right);
        }
        self
    }

    fn between_opt<F, V>(
        mut self,
        field_func: F,
        value_left: Option<V>,
        value_right: Option<V>,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value_left) = value_left
            && let Some(value_right) = value_right
        {
            self = self.between_field(*field_func(), value_left, value_right);
        }
        self
    }

    fn between_opt_flag<F, V>(
        mut self,
        field_func: F,
        value_left: Option<V>,
        value_right: Option<V>,
        flag: bool,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.between_opt(field_func, value_left, value_right);
        }
        self
    }

    fn between_field<V>(mut self, field: &'a str, value_left: V, value_right: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::Between,
            vec![value_left.into(), value_right.into()],
        ));
        self
    }

    fn between_field_flag<V>(
        mut self,
        field: &'a str,
        value_left: V,
        value_right: V,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.between_field(field, value_left, value_right);
        }
        self
    }

    fn between_field_opt<V>(
        mut self,
        field: &'a str,
        value_left: Option<V>,
        value_right: Option<V>,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value_left) = value_left
            && let Some(value_right) = value_right
        {
            self = self.between_field(field, value_left, value_right);
        }
        self
    }

    fn between_field_opt_flag<V>(
        mut self,
        field: &'a str,
        value_left: Option<V>,
        value_right: Option<V>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.between_field_opt(field, value_left, value_right);
        }
        self
    }

    fn not_between<F, V>(self, field_func: F, value_left: V, value_right: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.not_between_field(*field_func(), value_left, value_right)
    }

    fn not_between_flag<F, V>(
        mut self,
        field_func: F,
        value_left: V,
        value_right: V,
        flag: bool,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_between(field_func, value_left, value_right);
        }
        self
    }

    fn not_between_opt<F, V>(
        mut self,
        field_func: F,
        value_left: Option<V>,
        value_right: Option<V>,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value_left) = value_left
            && let Some(value_right) = value_right
        {
            self = self.not_between_field(*field_func(), value_left, value_right);
        }
        self
    }

    fn not_between_opt_flag<F, V>(
        mut self,
        field_func: F,
        value_left: Option<V>,
        value_right: Option<V>,
        flag: bool,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_between_opt(field_func, value_left, value_right);
        }
        self
    }

    fn not_between_field<V>(mut self, field: &'a str, value_left: V, value_right: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::NotBetween,
            vec![value_left.into(), value_right.into()],
        ));
        self
    }

    fn not_between_field_flag<V>(
        mut self,
        field: &'a str,
        value_left: V,
        value_right: V,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_between_field(field, value_left, value_right);
        }
        self
    }

    fn not_between_field_opt<V>(
        mut self,
        field: &'a str,
        value_left: Option<V>,
        value_right: Option<V>,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value_left) = value_left
            && let Some(value_right) = value_right
        {
            self = self.not_between_field(field, value_left, value_right);
        }
        self
    }

    fn not_between_field_opt_flag<V>(
        mut self,
        field: &'a str,
        value_left: Option<V>,
        value_right: Option<V>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_between_field_opt(field, value_left, value_right);
        }
        self
    }

    fn like<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.like_field(*field_func(), value)
    }

    fn like_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like(field_func, value);
        }
        self
    }

    fn like_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.like_field(*field_func(), value);
        }
        self
    }

    fn like_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_opt(field_func, value);
        }
        self
    }

    fn like_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::Like, vec![value.into()]));
        self
    }

    fn like_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_field(field, value);
        }
        self
    }

    fn like_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.like_field(field, value);
        }
        self
    }

    fn like_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_field_opt(field, value);
        }
        self
    }

    fn not_like<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.not_like_field(*field_func(), value)
    }

    fn not_like_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_like(field_func, value);
        }
        self
    }

    fn not_like_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.not_like_field(*field_func(), value);
        }
        self
    }

    fn not_like_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_like_opt(field_func, value);
        }
        self
    }

    fn not_like_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::NotLike, vec![value.into()]));
        self
    }

    fn not_like_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_like_field(field, value);
        }
        self
    }

    fn not_like_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.not_like_field(field, value);
        }
        self
    }

    fn not_like_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_like_field_opt(field, value);
        }
        self
    }

    fn like_left<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.like_left_field(*field_func(), value)
    }

    fn like_left_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_left(field_func, value);
        }
        self
    }

    fn like_left_pot<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.like_left_field(*field_func(), value);
        }
        self
    }

    fn like_left_pot_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_left_pot(field_func, value);
        }
        self
    }

    fn like_left_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::LikeLeft,
            vec![value.into()],
        ));
        self
    }

    fn like_left_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_left_field(field, value);
        }
        self
    }

    fn like_left_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.like_left_field(field, value);
        }
        self
    }

    fn like_left_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_left_field_opt(field, value);
        }
        self
    }

    fn like_right<F, V>(self, field_func: F, value: V) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.like_right_field(*field_func(), value)
    }

    fn like_right_flag<F, V>(mut self, field_func: F, value: V, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_right(field_func, value);
        }
        self
    }

    fn like_right_opt<F, V>(mut self, field_func: F, value: Option<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.like_right_field(*field_func(), value);
        }
        self
    }

    fn like_right_opt_flag<F, V>(mut self, field_func: F, value: Option<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_right_opt(field_func, value);
        }
        self
    }

    fn like_right_field<V>(mut self, field: &'a str, value: V) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::LikeRight,
            vec![value.into()],
        ));
        self
    }

    fn like_right_field_flag<V>(mut self, field: &'a str, value: V, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_right_field(field, value);
        }
        self
    }

    fn like_right_field_opt<V>(mut self, field: &'a str, value: Option<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(value) = value {
            self = self.like_right_field(field, value);
        }
        self
    }

    fn like_right_field_opt_flag<V>(mut self, field: &'a str, value: Option<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.like_right_field_opt(field, value);
        }
        self
    }

    fn r#in<F, V>(self, field_func: F, values: HashSet<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.in_field(*field_func(), values)
    }

    fn in_flag<F, V>(mut self, field_func: F, values: HashSet<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.r#in(field_func, values);
        }
        self
    }

    fn in_opt<F, V>(mut self, field_func: F, values: Option<HashSet<V>>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.in_field(*field_func(), values);
        }
        self
    }

    fn in_opt_flag<F, V>(mut self, field_func: F, values: Option<HashSet<V>>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_opt(field_func, values);
        }
        self
    }

    fn in_field<V>(mut self, field: &'a str, values: HashSet<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::In,
            values.iter().map(|v| v.clone().into()).collect::<Vec<_>>(),
        ));
        self
    }

    fn in_field_flag<V>(mut self, field: &'a str, values: HashSet<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_field(field, values);
        }
        self
    }

    fn in_field_opt<V>(mut self, field: &'a str, values: Option<HashSet<V>>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.in_field(field, values);
        }
        self
    }

    fn in_field_opt_flag<V>(
        mut self,
        field: &'a str,
        values: Option<HashSet<V>>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_field_opt(field, values);
        }
        self
    }

    fn in_vec<F, V>(self, field_func: F, values: Vec<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.in_vec_field(*field_func(), values)
    }

    fn in_vec_flag<F, V>(mut self, field_func: F, values: Vec<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_vec(field_func, values);
        }
        self
    }

    fn in_vec_opt<F, V>(mut self, field_func: F, values: Option<Vec<V>>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.in_vec_field(*field_func(), values);
        }
        self
    }

    fn in_vec_opt_flag<F, V>(mut self, field_func: F, values: Option<Vec<V>>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_vec_opt(field_func, values);
        }
        self
    }

    fn in_vec_field<V>(mut self, field: &'a str, values: Vec<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::In,
            values.iter().map(|v| v.clone().into()).collect::<Vec<_>>(),
        ));
        self
    }

    fn in_vec_field_flag<V>(mut self, field: &'a str, values: Vec<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_vec_field(field, values);
        }
        self
    }

    fn in_vec_field_opt<V>(mut self, field: &'a str, values: Option<Vec<V>>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.in_vec_field(field, values);
        }
        self
    }

    fn in_vec_field_opt_flag<V>(
        mut self,
        field: &'a str,
        values: Option<Vec<V>>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.in_vec_field_opt(field, values);
        }
        self
    }

    fn not_in<F, V>(self, field_func: F, values: HashSet<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.not_in_field(*field_func(), values)
    }

    fn not_in_flag<F, V>(mut self, field_func: F, values: HashSet<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in(field_func, values);
        }
        self
    }

    fn not_in_opt<F, V>(mut self, field_func: F, values: Option<HashSet<V>>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.not_in_field(*field_func(), values);
        }
        self
    }

    fn not_in_opt_flag<F, V>(
        mut self,
        field_func: F,
        values: Option<HashSet<V>>,
        flag: bool,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_opt(field_func, values);
        }
        self
    }

    fn not_in_field<V>(mut self, field: &'a str, values: HashSet<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::NotIn,
            values.iter().map(|v| v.clone().into()).collect::<Vec<_>>(),
        ));
        self
    }

    fn not_in_field_flag<V>(mut self, field: &'a str, values: HashSet<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_field(field, values);
        }
        self
    }

    fn not_in_field_opt<V>(mut self, field: &'a str, values: Option<HashSet<V>>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.not_in_field(field, values);
        }
        self
    }

    fn not_in_opt_field_flag<V>(
        mut self,
        field: &'a str,
        values: Option<HashSet<V>>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_field_opt(field, values);
        }
        self
    }

    fn not_in_vec<F, V>(self, field_func: F, values: Vec<V>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.not_in_vec_field(*field_func(), values)
    }

    fn not_in_vec_flag<F, V>(mut self, field_func: F, values: Vec<V>, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_vec(field_func, values);
        }
        self
    }

    fn not_in_vec_opt<F, V>(mut self, field_func: F, values: Option<Vec<V>>) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.not_in_vec_field(*field_func(), values);
        }
        self
    }

    fn not_in_vec_opt_flag<F, V>(
        mut self,
        field_func: F,
        values: Option<Vec<V>>,
        flag: bool,
    ) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_vec_opt(field_func, values);
        }
        self
    }

    fn not_in_vec_field<V>(mut self, field: &'a str, values: Vec<V>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(
            field,
            Relationship::NotIn,
            values.iter().map(|v| v.clone().into()).collect::<Vec<_>>(),
        ));
        self
    }

    fn not_in_vec_field_flag<V>(mut self, field: &'a str, values: Vec<V>, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_vec_field(field, values);
        }
        self
    }

    fn not_in_vec_field_opt<V>(mut self, field: &'a str, values: Option<Vec<V>>) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if let Some(values) = values {
            self = self.not_in_vec_field(field, values);
        }
        self
    }

    fn not_in_vec_field_opt_flag<V>(
        mut self,
        field: &'a str,
        values: Option<Vec<V>>,
        flag: bool,
    ) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_in_vec_field_opt(field, values);
        }
        self
    }

    fn null<F, V>(self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.null_field::<V>(*field_func())
    }

    fn null_flag<F, V>(mut self, field_func: F, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.null::<F, V>(field_func);
        }
        self
    }

    fn null_field<V>(mut self, field: &'a str) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::IsNull, vec![]));
        self
    }

    fn null_field_flag<V>(mut self, field: &'a str, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.null_field::<V>(field);
        }
        self
    }

    fn not_null<F, V>(self, field_func: F) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.not_null_field::<V>(*field_func())
    }

    fn not_null_flag<F, V>(mut self, field_func: F, flag: bool) -> Self
    where
        F: FnOnce() -> LambdaField<'a>,
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_null::<F, V>(field_func);
        }
        self
    }

    fn not_null_field<V>(mut self, field: &'a str) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        self.wheres_push(Where::new(field, Relationship::IsNotNull, vec![]));
        self
    }

    fn not_null_field_flag<V>(mut self, field: &'a str, flag: bool) -> Self
    where
        V: Into<SqlValue> + Clone,
        Self: Sized,
    {
        if flag {
            self = self.not_null_field::<V>(field);
        }
        self
    }

    fn and_fn<F>(mut self, func: F) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized,
    {
        if self.wheres().is_empty() {
            return func(self);
        }
        let start = self.wheres().len();
        self = func(self);
        let end = self.wheres().len();
        if start < end {
            self.bracket_mut().inc_left(start);
            self.bracket_mut().inc_right(end);
        }
        self
    }

    fn and_fn_flag<F>(self, func: F, flag: bool) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized,
    {
        if flag {
            return self.and_fn(func);
        }
        self
    }

    fn or(mut self) -> Self
    where
        Self: Sized,
    {
        if !self.wheres().is_empty() {
            let wheres_len = self.wheres().len();
            self.or_index_insert(wheres_len);
        }
        self
    }

    fn or_flag(mut self, flag: bool) -> Self
    where
        Self: Sized,
    {
        if flag {
            self = self.or();
        }
        self
    }

    fn or_fn<F>(mut self, func: F) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized,
    {
        if self.wheres().is_empty() {
            return func(self);
        }
        let wheres_len = self.wheres().len();
        self.or_index_insert(wheres_len);
        let start = self.wheres().len();
        self = func(self);
        let end = self.wheres().len();
        if start < end {
            self.bracket_mut().inc_left(start);
            self.bracket_mut().inc_right(end);
        }
        self
    }

    fn or_fn_flag<F>(self, flag: bool, func: F) -> Self
    where
        F: FnOnce(Self) -> Self,
        Self: Sized,
    {
        if flag {
            return self.or_fn(func);
        }
        self
    }

    fn first(self, sql: &'a str) -> Self
    where
        Self: Sized;

    fn last(self, sql: &'a str) -> Self
    where
        Self: Sized;

    fn comment(self, comment: &'a str) -> Self
    where
        Self: Sized;

    fn r#where(&mut self) -> String {
        let mut where_sql = String::new();
        let wheres = self.wheres();
        if wheres.is_empty() {
            return where_sql;
        }
        let bracket = self.bracket();
        let r#where = &wheres[0];
        where_sql += &format!(" WHERE {}", r#where.to_bind_sql());
        for (i, r#where) in wheres.iter().enumerate().skip(1) {
            for _ in 0..bracket.dec_all_right(i) {
                where_sql += ")";
            }
            where_sql += if self.or_index().contains(&i) {
                " OR "
            } else {
                " AND "
            };
            for _ in 0..bracket.dec_all_left(i) {
                where_sql += "(";
            }
            where_sql += r#where.to_bind_sql().as_str();
        }
        for (key, value) in bracket.right.borrow().iter() {
            if *key >= wheres.len() {
                for _ in 0..*value {
                    where_sql += ")";
                }
            }
        }
        where_sql
    }

    fn bind_query<'q>(
        &self,
        mut query: Query<'q, MySql, MySqlArguments>,
    ) -> Query<'q, MySql, MySqlArguments>
    where
        Self: Sized,
    {
        for r#where in self.wheres() {
            query = bind_query(query, &r#where.values);
        }
        query
    }

    fn bind_query_as<'q, E: Entity>(
        &self,
        mut query_as: QueryAs<'q, MySql, E, MySqlArguments>,
    ) -> QueryAs<'q, MySql, E, MySqlArguments>
    where
        Self: Sized,
    {
        for r#where in self.wheres() {
            query_as = bind_query_as(query_as, &r#where.values);
        }
        query_as
    }

    fn bind_query_scalar<'q, T>(
        &self,
        mut query_scalar: QueryScalar<'q, MySql, T, MySqlArguments>,
    ) -> QueryScalar<'q, MySql, T, MySqlArguments>
    where
        Self: Sized,
    {
        for r#where in self.wheres() {
            query_scalar = bind_query_scalar(query_scalar, &r#where.values);
        }
        query_scalar
    }
}

#[derive(Debug)]
pub struct Where<'a> {
    field: &'a str,
    relationship: Relationship,
    values: Vec<SqlValue>,
}

impl<'a> Where<'a> {
    fn new(field: &'a str, relationship: Relationship, values: Vec<SqlValue>) -> Self {
        Self {
            field,
            relationship,
            values,
        }
    }

    fn to_bind_sql(&self) -> String {
        let mut sql = format!("{} {}", self.field, self.relationship.to_str());
        match self.relationship {
            Relationship::Eq
            | Relationship::Ne
            | Relationship::Gt
            | Relationship::Ge
            | Relationship::Lt
            | Relationship::Le
            | Relationship::Like
            | Relationship::NotLike
            | Relationship::LikeLeft
            | Relationship::LikeRight => sql += " ?",
            Relationship::Between | Relationship::NotBetween => sql += " ? AND ?",
            Relationship::In | Relationship::NotIn => {
                let placeholders = self
                    .values
                    .iter()
                    .map(|_| "?")
                    .collect::<Vec<_>>()
                    .join(", ");
                sql += &format!(" ({placeholders})");
            }
            Relationship::IsNull => sql += " IS NULL",
            Relationship::IsNotNull => sql += "IS NOT NULL",
        }
        sql
    }
}

pub struct Bracket {
    left: RefCell<HashMap<usize, usize>>,
    right: RefCell<HashMap<usize, usize>>,
}

impl Bracket {
    pub(crate) fn new() -> Self {
        Self {
            left: RefCell::new(HashMap::new()),
            right: RefCell::new(HashMap::new()),
        }
    }

    fn inc_left_count(&self, index: usize, count: usize) {
        *self.left.borrow_mut().entry(index).or_insert(0) += count;
    }

    fn inc_right_count(&self, index: usize, count: usize) {
        *self.right.borrow_mut().entry(index).or_insert(0) += count;
    }

    fn inc_left(&self, index: usize) {
        self.inc_left_count(index, 1);
    }

    fn inc_right(&self, index: usize) {
        self.inc_right_count(index, 1);
    }

    fn dec_all_left(&self, index: usize) -> usize {
        if let Some(left) = self.left.borrow_mut().remove(&index) {
            return left;
        }
        0
    }

    fn dec_all_right(&self, index: usize) -> usize {
        if let Some(right) = self.right.borrow_mut().remove(&index) {
            return right;
        }
        0
    }
}

pub(crate) struct GroupHaving<'a> {
    fields: Vec<&'a str>,
    having: Option<&'a str>,
    values: Vec<SqlValue>,
}

impl<'a> GroupHaving<'a> {
    pub(crate) fn new() -> Self {
        Self {
            fields: vec![],
            having: None,
            values: vec![],
        }
    }

    pub(crate) fn field_push(&mut self, field: &'a str) {
        self.fields.push(field);
    }

    pub(crate) fn having(&mut self, having: &'a str) {
        self.having = Some(having);
    }

    pub(crate) fn value_push(&mut self, values: SqlValue) {
        self.values.push(values);
    }
}

pub(crate) struct Order<'a> {
    pub(crate) field: &'a str,
    pub(crate) asc_desc: bool,
}

impl<'a> Order<'a> {
    pub fn new(field: &'a str, asc_desc: bool) -> Self {
        Self { field, asc_desc }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Relationship {
    // ==
    Eq,
    // <>
    Ne,
    // >
    Gt,
    // >=
    Ge,
    // <
    Lt,
    // <=
    Le,
    // BETWEEN
    Between,
    // NOT BETWEEN
    NotBetween,
    // LIKE '%str%'
    Like,
    // NOT LIKE 'str'
    NotLike,
    // LIKE '%str'
    LikeLeft,
    // LIKE 'str%'
    LikeRight,
    // IN
    In,
    // NOT IN
    NotIn,
    // IS NULL
    IsNull,
    // IS NOT NULL
    IsNotNull,
}

impl Relationship {
    fn to_str(&self) -> &str {
        match self {
            Relationship::Eq => "=",
            Relationship::Ne => "<>",
            Relationship::Gt => ">",
            Relationship::Ge => ">=",
            Relationship::Lt => "<",
            Relationship::Le => "<=",
            Relationship::Between => "BETWEEN",
            Relationship::NotBetween => "NOT BETWEEN",
            Relationship::Like | Relationship::LikeLeft | Relationship::LikeRight => "LIKE",
            Relationship::NotLike => "NOT LIKE",
            Relationship::In => "IN",
            Relationship::NotIn => "NOT IN",
            Relationship::IsNull => "IS NULL",
            Relationship::IsNotNull => "IS NOT NULL",
        }
    }
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    ISize(isize),
    USize(usize),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(String),
    Time(NaiveTime),
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Decimal(Decimal),
}

impl From<isize> for SqlValue {
    fn from(value: isize) -> Self {
        SqlValue::ISize(value)
    }
}

impl From<usize> for SqlValue {
    fn from(value: usize) -> Self {
        SqlValue::USize(value)
    }
}

impl From<i8> for SqlValue {
    fn from(value: i8) -> Self {
        SqlValue::I8(value)
    }
}

impl From<u8> for SqlValue {
    fn from(value: u8) -> Self {
        SqlValue::U8(value)
    }
}

impl From<i16> for SqlValue {
    fn from(value: i16) -> Self {
        SqlValue::I16(value)
    }
}

impl From<u16> for SqlValue {
    fn from(value: u16) -> Self {
        SqlValue::U16(value)
    }
}

impl From<i32> for SqlValue {
    fn from(value: i32) -> Self {
        SqlValue::I32(value)
    }
}

impl From<u32> for SqlValue {
    fn from(value: u32) -> Self {
        SqlValue::U32(value)
    }
}

impl From<i64> for SqlValue {
    fn from(value: i64) -> Self {
        SqlValue::I64(value)
    }
}

impl From<u64> for SqlValue {
    fn from(value: u64) -> Self {
        SqlValue::U64(value)
    }
}

impl From<f32> for SqlValue {
    fn from(value: f32) -> Self {
        SqlValue::F32(value)
    }
}

impl From<f64> for SqlValue {
    fn from(value: f64) -> Self {
        SqlValue::F64(value)
    }
}

impl From<bool> for SqlValue {
    fn from(value: bool) -> Self {
        SqlValue::Bool(value)
    }
}

impl From<&str> for SqlValue {
    fn from(value: &str) -> Self {
        SqlValue::Str(value.to_string())
    }
}

impl From<String> for SqlValue {
    fn from(value: String) -> Self {
        SqlValue::Str(value)
    }
}

impl From<NaiveTime> for SqlValue {
    fn from(value: NaiveTime) -> Self {
        SqlValue::Time(value)
    }
}

impl From<NaiveDate> for SqlValue {
    fn from(value: NaiveDate) -> Self {
        SqlValue::Date(value)
    }
}

impl From<NaiveDateTime> for SqlValue {
    fn from(value: NaiveDateTime) -> Self {
        SqlValue::DateTime(value)
    }
}

impl From<Decimal> for SqlValue {
    fn from(value: Decimal) -> Self {
        SqlValue::Decimal(value)
    }
}
