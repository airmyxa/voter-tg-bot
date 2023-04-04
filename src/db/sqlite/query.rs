use crate::db::query::DbQuery;

pub struct SQLiteQuery {
    name: String,
    query: String,
}

impl DbQuery for SQLiteQuery {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn raw(&self) -> String {
        self.query.clone()
    }
}

impl SQLiteQuery {
    pub fn new(name: String, query: String) -> Self {
        SQLiteQuery { name, query }
    }
}
