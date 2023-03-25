use rusqlite::Connection;
use std::sync::{Arc, Mutex, MutexGuard};

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

struct SQLiteConnection {
    connection: Arc<Mutex<Connection>>,
}

impl SQLiteConnection {
    pub fn new(connection: Connection) -> Self {
        SQLiteConnection {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    pub fn get(&self) -> MutexGuard<Connection> {
        return self.connection.lock().unwrap();
    }
}

pub struct SQLiteDb {
    settings: SQLiteSettings,
    connection: SQLiteConnection,
}

impl SQLiteDb {
    pub fn new(settings: SQLiteSettings) -> Self {
        let connection = match &settings.connection_type {
            SQLiteConnectionType::Memory => Connection::open_in_memory().unwrap(),
            SQLiteConnectionType::File(file) => Connection::open(&file).unwrap(),
        };
        SQLiteDb {
            settings,
            connection: SQLiteConnection::new(connection),
        }
    }

    pub fn get_connection(&self) -> MutexGuard<Connection> {
        return self.connection.get();
    }
}
