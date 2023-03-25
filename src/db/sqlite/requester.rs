use crate::db::sqlite::database::SQLiteDb;
use crate::db::sqlite::migrations;
use crate::db::sqlite::models::user_votes::UserVote;
use crate::db::sqlite::models::vote::Vote;
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
            .execute(migrations::CREATE_TABLE_VOTES, ())
            .unwrap();
        self.db
            .get_connection()
            .execute(migrations::CREATE_TABLE_USER_VOTES, ())
            .unwrap();
    }

    pub fn insert_vote(&self, chat_id: &String, message_id: &String, text: &String) {
        self.db
            .get_connection()
            .execute(sql::INSERT_VOTE, &[&chat_id, &message_id, &text])
            .unwrap();
    }

    pub fn upsert_user_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        user_name: &String,
        query_data: &String,
    ) {
        self.db
            .get_connection()
            .execute(
                sql::UPSERT_USER_VOTES,
                &[&chat_id, &message_id, &user_name, &query_data],
            )
            .unwrap();
    }

    pub fn select_voted_users_usernames(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Vec<String> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS).unwrap();
        return statement
            .query_map(&[&chat_id, &message_id], |row| {
                return Ok(String::from(row.get_unwrap::<_, String>(1)));
            })
            .unwrap()
            .map(|value| {
                return value.unwrap();
            })
            .collect::<Vec<String>>();
    }

    pub fn select_voted_users(&self, chat_id: &String, message_id: &String) -> Vec<UserVote> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS).unwrap();
        return statement
            .query_map(&[&chat_id, &message_id], |row| {
                let vote_id = row.get_unwrap(0);
                let username = row.get_unwrap(1);
                let query_data = row.get_unwrap(2);
                return Ok(UserVote {
                    vote_id,
                    username,
                    query_data,
                });
            })
            .unwrap()
            .map(|value| {
                return value.unwrap();
            })
            .collect::<Vec<UserVote>>();
    }

    pub fn select_vote(&self, chat_id: &String, message_id: &String) -> Option<Vote> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTE).unwrap();
        let result = statement
            .query_row(&[&chat_id, &message_id], |row| {
                let id = row.get_unwrap(0);
                let chat_id = row.get_unwrap(1);
                let message_id = row.get_unwrap(2);
                let text = row.get_unwrap(3);
                let result = Vote {
                    id,
                    chat_id,
                    message_id,
                    text,
                };
                return Ok(result);
            })
            .unwrap();
        return Some(result);
    }
}
