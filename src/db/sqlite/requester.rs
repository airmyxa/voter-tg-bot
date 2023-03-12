use crate::db::sqlite::migrations;
use crate::db::sqlite::sql;
use rusqlite::{Connection, RowIndex};
use std::rc::Rc;
use std::sync::Arc;

pub struct SQLiteRequester {
    connection: Arc<Connection>,
}

impl SQLiteRequester {
    pub fn new(connection: Arc<Connection>) -> Self {
        SQLiteRequester { connection }
    }

    pub fn init_db(&self) {
        self.connection.execute(migrations::INIT_DB, ()).unwrap();
    }

    pub fn upsert_user_vote(
        &self,
        chat_id: String,
        message_id: String,
        user_name: String,
        query_data: String,
    ) {
        self.connection
            .execute(
                sql::UPSERT_USER_VOTES,
                &[&chat_id, &message_id, &user_name, &query_data],
            )
            .unwrap();
    }

    pub fn select_voted_users(&self, chat_id: String, message_id: String) -> Vec<String> {
        let mut statement = self.connection.prepare(sql::SELECT_VOTED_USERS).unwrap();
        return statement
            .query_map(&[&chat_id, &message_id], |row| {
                return Ok(String::from(row.get_unwrap::<_, String>(0)));
            })
            .unwrap()
            .map(|value| {
                return value.unwrap();
            })
            .collect::<Vec<String>>();
    }
}
