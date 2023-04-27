use crate::query::clause::exists::ExistIn;
use crate::query::clause::{AsFieldName, ClauseAdder, OrderBy};
use crate::relations::{HasRelations, Relationship};
use crate::table::{HasSchema, TableColumns, TableInfo, UniqueIdentifier};
use crate::writers::limit_skip::DbLimitSkipWriter;
use std::marker::PhantomData;

/// An un-executed Query.
///
/// Build out a query that can be executed on the database.
///
/// Can be chained with other queries to make more complex queries.
///
/// Can be mapped into other queries to  make more complex queries.
pub struct QueryBuilder<'schema, T, DB: sqlx::Database> {
    _t: PhantomData<T>,
    pub(crate) wheres: Vec<Box<dyn ClauseAdder<'schema, DB>>>,
    pub(crate) exist_ins: Vec<ExistIn<'schema, DB>>,
    pub(crate) limit: Option<i64>,
    pub(crate) offset: Option<i64>,
    pub(crate) orderby: Vec<OrderBy>,
}

impl<'schema, T, DB> Default for QueryBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'schema, T, DB> QueryBuilder<'schema, T, DB>
where
    DB: sqlx::Database,
    T: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
{
    pub fn new() -> Self {
        Self {
            _t: Default::default(),
            wheres: Vec::default(),
            limit: None,
            offset: None,
            orderby: Vec::default(),
            exist_ins: Default::default(),
        }
    }

    /// Filter the results returned by this query.
    /// Used when you want to filter on the columns of this table.
    pub fn where_col(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> Box<dyn ClauseAdder<'schema, DB>>,
    ) -> Self
    where
        <T as HasSchema>::Schema: Default,
    {
        let qba = lam(Default::default());
        self.wheres.push(qba);
        self
    }

    /// Add a query to this query (JOIN on a relationship)
    /// results on a query that is filtered using the results of both queries
    pub fn where_relation<R, Ship>(
        mut self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
        filter: QueryBuilder<'schema, R, DB>,
    ) -> Self
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations + UniqueIdentifier<DB>,
        Ship: Relationship<R>,
        R: HasSchema + UniqueIdentifier<DB>,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB>,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let out_col = ship.my_key::<DB, R, T>();
        let inner_tn = <R as HasSchema>::Schema::identifier();
        let inner_tn = inner_tn.join(".");
        let inner_col = ship.their_key::<DB, R, T>();
        let exist_in = ExistIn::<'schema, DB>::new(filter, out_col, inner_tn, inner_col);
        self.exist_ins.push(exist_in);
        self
    }

    /// Results in a query that is mapped into the query of one of its relationships
    pub fn map_query<R, Ship>(
        self,
        relationship: impl Fn(<T as HasRelations>::Relation) -> Ship,
    ) -> QueryBuilder<'schema, R, DB>
    where
        DB: sqlx::Database + DbLimitSkipWriter,
        T: HasRelations + UniqueIdentifier<DB>,
        Ship: Relationship<R>,
        R: HasSchema + UniqueIdentifier<DB>,
        R: Send + Unpin + for<'r> sqlx::FromRow<'r, DB::Row> + HasSchema,
        <R as HasSchema>::Schema: TableInfo + TableColumns<DB>,
        <T as HasRelations>::Relation: Default,
    {
        let ship = relationship(Default::default());
        let mut sb: QueryBuilder<R, DB> = QueryBuilder::new();

        let out_col = ship.their_key::<DB, R, T>();
        let inner_tn = <T as HasSchema>::Schema::identifier().join(".");
        let inner_col = ship.my_key::<DB, R, T>();
        let exist_in = ExistIn::<'schema, DB>::new(self, out_col, inner_tn, inner_col);

        sb.exist_ins.push(exist_in);
        sb
    }

    /// Limit the number of rows returned by this query
    pub fn limit(mut self, x: i64) -> Self {
        self.limit = Some(x);
        self
    }

    /// Offset the starting point for the results returned by this query
    pub fn offset(mut self, x: i64) -> Self {
        self.offset = Some(x);
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_desc<FN: AsFieldName>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let fieldname = field.fieldname();
        self.orderby.push(OrderBy::new(fieldname, "DESC"));
        self
    }

    /// Order the results of the query by a given column
    ///
    /// multiple calls will result in multiple OrderBys
    pub fn order_by_asc<FN: AsFieldName>(
        mut self,
        lam: impl Fn(<T as HasSchema>::Schema) -> FN,
    ) -> Self {
        let field = lam(Default::default());
        let fieldname = field.fieldname();
        self.orderby.push(OrderBy::new(fieldname, "ASC"));
        self
    }
}