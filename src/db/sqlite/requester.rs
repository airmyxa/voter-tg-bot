use crate::db::sqlite::database::SQLiteDb;
use crate::db::sqlite::migrations;
use crate::db::sqlite::sql;
use std::sync::Arc;

pub struct SQLiteRequester {
    db: Arc<SQLiteDb>,
}

impl SQLiteRequester {
    pub fn new(db: Arc<SQLiteDb>) -> Self {
        SQLiteRequester { db }
    }

    pub fn init_db(&self) {
        self.db
            .get_connection()
            .execute(migrations::INIT_DB, ())
            .unwrap();
    }

    pub fn upsert_user_vote(
        &self,
        chat_id: String,
        message_id: String,
        user_name: String,
        query_data: String,
    ) {
        self.db
            .get_connection()
            .execute(
                sql::UPSERT_USER_VOTES,
                &[&chat_id, &message_id, &user_name, &query_data],
            )
            .unwrap();
    }

    pub fn select_voted_users(&self, chat_id: String, message_id: String) -> Vec<String> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS).unwrap();
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
