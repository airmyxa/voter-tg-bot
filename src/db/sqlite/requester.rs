use crate::db::sqlite::database::SQLiteDb;
use crate::db::sqlite::migrations;
use crate::db::sqlite::models::user_votes::UserVote;
use crate::db::sqlite::models::vote::Vote;
use crate::db::sqlite::sql;
use crate::views::handler::HandlerErr;
use log::warn;
use std::sync::Arc;

pub struct SQLiteRequester {
    db: Arc<SQLiteDb>,
}

impl SQLiteRequester {
    pub fn new(db: Arc<SQLiteDb>) -> Self {
        SQLiteRequester { db }
    }

    pub fn init_db(&self) -> Result<(), HandlerErr> {
        self.db
            .get_connection()
            .execute(migrations::CREATE_TABLE_VOTES, ())?;
        self.db
            .get_connection()
            .execute(migrations::CREATE_TABLE_USER_VOTES, ())?;
        Ok(())
    }

    pub fn insert_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        text: &String,
    ) -> Result<(), HandlerErr> {
        self.db
            .get_connection()
            .execute(sql::INSERT_VOTE, &[&chat_id, &message_id, &text])?;
        Ok(())
    }

    pub fn upsert_user_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        user_name: &String,
        query_data: &String,
    ) -> Result<(), HandlerErr> {
        self.db.get_connection().execute(
            sql::UPSERT_USER_VOTES,
            &[&chat_id, &message_id, &user_name, &query_data],
        )?;
        Ok(())
    }

    pub fn select_voted_users_usernames(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Vec<String>, HandlerErr> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS)?;
        let result = statement
            .query_map(&[&chat_id, &message_id], |row| {
                return Ok(String::from(row.get::<_, String>(1)?));
            })?
            .filter_map(|value| {
                return match value {
                    Err(err) => {
                        warn!("Got error while processing db response");
                        None
                    }
                    Ok(value) => Some(value),
                };
            })
            .collect::<Vec<String>>();
        return Ok(result);
    }

    pub fn select_voted_users(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Vec<UserVote>, HandlerErr> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS)?;
        let result = statement
            .query_map(&[&chat_id, &message_id], |row| {
                let vote_id = row.get(0);
                let username = row.get(1);
                let query_data = row.get(2);

                return Ok(UserVote {
                    vote_id: vote_id?,
                    username: username?,
                    query_data: query_data?,
                });
            })?
            .filter_map(|value| {
                return match value {
                    Err(err) => {
                        warn!("Got error while processing db response");
                        None
                    }
                    Ok(value) => Some(value),
                };
            })
            .collect::<Vec<UserVote>>();
        return Ok(result);
    }

    pub fn select_vote(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Option<Vote>, HandlerErr> {
        let connection = self.db.get_connection();
        let mut statement = connection.prepare(sql::SELECT_VOTE)?;
        let result = statement.query_row(&[&chat_id, &message_id], |row| {
            let id = row.get(0);
            let chat_id = row.get(1);
            let message_id = row.get(2);
            let text = row.get(3);
            let result = Vote {
                id: id?,
                chat_id: chat_id?,
                message_id: message_id?,
                text: text?,
            };
            return Ok(result);
        })?;
        return Ok(Some(result));
    }
}
