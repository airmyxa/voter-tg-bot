use crate::views::error::RuntimeError;
use crate::views::handler::{GenericError, MaybeError};
use log::error;
use rusqlite::Connection;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

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

    pub fn get(&self) -> Result<MutexGuard<Connection>, GenericError> {
        return match self.connection.lock() {
            Ok(mutex_guard) => Ok(mutex_guard),
            Err(err) => {
                let message = format!(
                    "Got error while trying to lock connection: {}",
                    err.to_string()
                );
                return Err(Box::new(RuntimeError::new(message)));
            }
        };
    }
}

pub struct SQLiteDb {
    settings: SQLiteSettings,
    connection: SQLiteConnection,
}

impl SQLiteDb {
    pub fn new(settings: SQLiteSettings) -> Result<Self, GenericError> {
        let connection = match &settings.connection_type {
            SQLiteConnectionType::Memory => Connection::open_in_memory()?,
            SQLiteConnectionType::File(file) => Connection::open(&file)?,
        };
        Ok(SQLiteDb {
            settings,
            connection: SQLiteConnection::new(connection),
        })
    }

    pub fn get_connection(&self) -> Result<MutexGuard<Connection>, GenericError> {
        return self.connection.get();
    }
}
