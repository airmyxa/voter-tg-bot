use crate::db::sqlite::requester::SQLiteRequester;
use rusqlite::{Connection, Statement};
use std::rc::Rc;
use std::sync::Arc;

pub enum SQLiteConnectionType {
    File(String),
    Memory,
}

pub struct SQLiteSettings {
    connection_type: SQLiteConnectionType,
}

impl SQLiteSettings {
    pub fn new(connection_type: SQLiteConnectionType) -> Self {
        SQLiteSettings { connection_type }
    }
}

pub struct SQLiteDb {
    settings: SQLiteSettings,
    connection: Arc<Connection>,
    requester: SQLiteRequester,
}

impl SQLiteDb {
    pub fn connect(settings: SQLiteSettings) -> Self {
        let connection = match &settings.connection_type {
            SQLiteConnectionType::Memory => Connection::open_in_memory().unwrap(),
            SQLiteConnectionType::File(file) => Connection::open(&file).unwrap(),
        };
        let connection = Arc::new(connection);
        SQLiteDb {
            settings,
            connection: connection.clone(),
            requester: SQLiteRequester::new(connection.clone()),
        }
    }

    pub fn requester<'a>(&'a self) -> &SQLiteRequester {
        return &self.requester;
    }
}
