use crate::components_container::{Component, ComponentsContainer};
use crate::db::sqlite::database::{self, SQLiteDb};
use crate::db::sqlite::requester::SQLiteRequester;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct DependenciesComponent {
    db: Arc<SQLiteDb>,
    db_requester: Arc<SQLiteRequester>,
}

pub struct Dependencies {
    pub db_requester: Arc<SQLiteRequester>,
}

impl Component for DependenciesComponent {
    fn create_component(components: &mut ComponentsContainer) -> Self {
        let db = components.get_component_as::<database::SQLiteDb>("sqlite-db");
        let db_requester = components.get_component_as::<SQLiteRequester>("sqlite-requester");
        let deps = DependenciesComponent { db, db_requester };

        return deps;
    }

    fn component_name(&self) -> &'static str {
        return "dependencies";
    }
}

impl DependenciesComponent {
    pub fn dependencies(&self) -> Dependencies {
        Dependencies {
            db_requester: self.db_requester.clone(),
        }
    }
}
