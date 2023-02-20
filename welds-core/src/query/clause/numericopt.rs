use super::{ClauseColVal, QueryBuilderAdder};
use std::marker::PhantomData;

pub struct NumericOpt<T> {
    field: String,
    _t: PhantomData<T>,
}

use crate::query::optional::HasSomeNone;
impl<'args, T> NumericOpt<T>
where
    T: Send + HasSomeNone + Clone + crate::row::ToRow<'args> + 'static,
{
    pub fn new(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            _t: Default::default(),
        }
    }

    pub fn equal(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: "=",
            val,
        };
        Box::new(cv)
    }

    pub fn not_equal(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: "!=",
            val,
        };
        Box::new(cv)
    }

    pub fn gt(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: ">",
            val,
        };
        Box::new(cv)
    }
    pub fn lt(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: "<",
            val,
        };
        Box::new(cv)
    }

    pub fn gte(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: ">=",
            val,
        };
        Box::new(cv)
    }
    pub fn lte(self, v: impl Into<T>) -> Box<dyn QueryBuilderAdder<'args>> {
        let val = v.into();
        let cv = ClauseColVal::<T> {
            isnull_clause: val.is_none(),
            col: self.field,
            operator: "<=",
            val,
        };
        Box::new(cv)
    }
}