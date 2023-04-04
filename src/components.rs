use crate::db::sqlite::database::{SQLiteConnectionType, SQLiteDb, SQLiteSettings};
use crate::db::sqlite::requester::SQLiteRequester;
use crate::dependencies::Dependencies;
use std::ops::Deref;
use std::sync::Arc;
use crate::views::handler::HandlerErr;

pub struct Components {
    db: Arc<SQLiteDb>,
    requester: Arc<SQLiteRequester>,
    dependencies: Arc<Dependencies>,
}

impl Components {
    pub fn dependencies(&self) -> Dependencies {
        return self.dependencies.deref().clone();
    }
}

fn create_db() -> Result<SQLiteDb, HandlerErr> {
    let connection = SQLiteConnectionType::File(String::from("voter.db"));
    let settings = SQLiteSettings::new(connection);
    return SQLiteDb::new(settings);
}

fn create_requester(db: Arc<SQLiteDb>) -> SQLiteRequester {
    return SQLiteRequester::new(db);
}

fn create_dependencies(db: Arc<SQLiteDb>, requester: Arc<SQLiteRequester>) -> Dependencies {
    return Dependencies::new(db, requester);
}

pub fn create_components() -> Result<Components, HandlerErr> {
    let db = Arc::new(create_db()?);

    let requester = Arc::new(create_requester(Arc::clone(&db)));

    let dependencies = Arc::new(create_dependencies(Arc::clone(&db), Arc::clone(&requester)));

    let components = Components {
        db: Arc::clone(&db),
        requester: Arc::clone(&requester),
        dependencies: Arc::clone(&dependencies),
    };

    return Ok(components);
}
