use crate::db::query::DbQuery;

pub trait Database {
    type Settings;
    type Query: DbQuery;
    type ExecutionResult;

    fn connect(settings: Self::Settings) -> Self;

    fn execute(&self, query: Self::Query) -> Self::ExecutionResult;
}
