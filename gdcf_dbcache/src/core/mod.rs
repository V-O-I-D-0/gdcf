use self::query::Query;
use std::fmt::Debug;

//#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
//pub mod cache;
//pub mod schema;
#[macro_use]
pub(crate) mod macros;
pub(crate) mod query;
pub(crate) mod table;
pub(crate) mod backend;
pub(crate) mod statement;

pub(crate) trait AsSql<DB: Database>: Debug {
    fn as_sql(&self) -> DB::Types;
    fn as_sql_string(&self) -> String;
}

pub(crate) trait FromSql<DB: Database> {
    fn from_sql(sql: DB::Types) -> Self;
}

pub(crate) trait Database: Debug {
    type Types;
    type Error;

    fn prepare(idx: usize) -> String;

    fn execute<'a>(&'a self, query: &'a Query<'a, Self>) -> Result<(), Self::Error>
        where
            Self: Sized
    {
        let (stmt, params) = query.to_sql();
        self.execute_raw(stmt.to_statement(Self::prepare), &params)
    }

    fn execute_unprepared<'a>(&'a self, query: &'a Query<'a, Self>) -> Result<(), Self::Error>
        where
            Self: Sized
    {
        self.execute_raw(query.to_sql_unprepared(), &[])
    }

    fn execute_raw(&self, statement: String, params: &[&AsSql<Self>]) -> Result<(), Self::Error>;
}