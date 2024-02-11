use crate::components_container::{Component, ComponentsContainer};
use crate::db::sqlite::database;
use crate::db::sqlite::requester::SQLiteRequester;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone)]
pub struct DependenciesComponent {
    db: &'static database::SQLiteDb,
    db_requester: &'static SQLiteRequester,
}

pub struct Dependencies {
    pub db_requester: &'static SQLiteRequester,
}

impl Component for DependenciesComponent {
    fn create_component(components: &mut ComponentsContainer) -> Self {
        let deps = DependenciesComponent {
            db: components.get_component_as::<database::SQLiteDb>("sqlite-db"),
            db_requester: components.get_component_as::<SQLiteRequester>("sqlite-requester"),
        };

        return deps;
    }

    fn component_name(&self) -> &'static str {
        return "dependencies";
    }
}

impl DependenciesComponent {
    pub fn dependencies(&self) -> Dependencies {
        Dependencies {
            db_requester: self.db_requester.deref(),
        }
    }
}
