use crate::db::sqlite::database;
use crate::db::sqlite::requester::SQLiteRequester;
use std::sync::Arc;

#[derive(Clone)]
pub struct Dependencies {
    pub db: Arc<database::SQLiteDb>,
    pub requester: Arc<SQLiteRequester>,
}

impl Dependencies {
    pub fn new(db: Arc<database::SQLiteDb>, requester: Arc<SQLiteRequester>) -> Self {
        Dependencies { db, requester }
    }
}
