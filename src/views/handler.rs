use crate::db::sqlite::database::SQLiteDb;
use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[async_trait]
pub trait HandlerTr<R, D>
where
    R: Debug,
{
    async fn handle(self, request: R, dependencies: D) -> HandlerResult;
}

pub struct Dependencies {
    pub db: Mutex<Arc<SQLiteDb>>,
}

unsafe impl Send for Dependencies {}
unsafe impl Sync for Dependencies {}

impl Dependencies {
    pub fn new(db: Arc<SQLiteDb>) -> Self {
        Dependencies {
            db: Mutex::<Arc<SQLiteDb>>::new(db),
        }
    }
}
