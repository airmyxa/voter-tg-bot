use crate::components_container::{ComponentTr, ComponentsContainer};
use crate::db::sqlite::database;
use crate::db::sqlite::requester::SQLiteRequester;
use std::sync::Arc;

#[derive(Clone)]
pub struct Dependencies {
    pub db: Arc<database::SQLiteDb>,
    pub requester: Arc<SQLiteRequester>,
}

impl ComponentTr for Dependencies {
    fn create_component(components: &mut ComponentsContainer) -> Arc<Self> {
        let deps = Dependencies {
            db: components.get_component_as::<database::SQLiteDb>("sqlite-db"),
            requester: components.get_component_as::<SQLiteRequester>("sqlite-requester"),
        };

        return Arc::new(deps);
    }

    fn component_name(&self) -> &'static str {
        return "dependencies";
    }
}
