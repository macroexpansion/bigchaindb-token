use diesel::{
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment},
    sql_types::BigInt,
    PgConnection, QueryId, QueryResult,
};
use diesel_async::{methods::LoadQuery, AsyncPgConnection, RunQueryDsl};
use serde::Serialize;

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct LoadCountPages<U> {
    pub records: Vec<U>,
    pub total_pages: i64,
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

impl<'a, T: Query + 'a> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }

    pub async fn load_and_count_pages<U>(
        self,
        conn: &mut AsyncPgConnection,
    ) -> QueryResult<LoadCountPages<U>>
    where
        Self: LoadQuery<'a, AsyncPgConnection, (U, i64)>,
        U: Send,
    {
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn).await?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Ok(LoadCountPages {
            records,
            total_pages,
        })
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> diesel::RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> diesel::RunQueryDsl<AsyncPgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
