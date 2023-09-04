use crate::components_container::{ComponentTr, ComponentsContainer};
use crate::db::sqlite::database::SQLiteDb;
use crate::db::sqlite::sql;
use crate::models::vote::{UserVote, Vote, VoteTemplate};
use crate::views::handler::GenericError;
use log::{info, warn};
use rusqlite::OptionalExtension;
use std::sync::Arc;

pub struct SQLiteRequester {
    db: Arc<SQLiteDb>,
}

impl ComponentTr for SQLiteRequester {
    fn create_component(components: &mut ComponentsContainer) -> Arc<Self> {
        let db = components.get_component_as::<SQLiteDb>("sqlite-db");
        let requester = SQLiteRequester { db };
        return Arc::new(requester);
    }

    fn component_name(&self) -> &'static str {
        return "sqlite-requester";
    }
}

impl SQLiteRequester {
    pub fn new(db: Arc<SQLiteDb>) -> Self {
        SQLiteRequester { db }
    }

    pub fn insert_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        text: &String,
        invoke_command: &String,
    ) -> Result<(), GenericError> {
        let vote_template_id: Option<i64> = self
            .db
            .get_connection()?
            .query_row(
                sql::SELECT_VOTE_TEMPLATE_BY_INVOKE_COMMAND,
                &[&invoke_command],
                |row| {
                    let id = row.get(0)?;
                    return Ok(id);
                },
            )
            .optional()?;

        let vote_template_id = if let Some(vote_template_id) = vote_template_id {
            info!(
                "Vote template for command '{}' already in db. Id = '{}'",
                invoke_command, vote_template_id
            );
            vote_template_id
        } else {
            info!(
                "Didn't find vote template for command '{}' in db. Inserting new one.",
                invoke_command
            );
            self.insert_vote_template(&invoke_command)?
        };

        self.db.get_connection()?.execute(
            sql::INSERT_VOTE,
            &[&chat_id, &message_id, &text, &vote_template_id.to_string()],
        )?;
        Ok(())
    }

    pub fn insert_vote_template(&self, invoke_command: &String) -> Result<i64, GenericError> {
        let row_id: i64 = self.db.get_connection()?.query_row(
            sql::INSERT_VOTE_TEMPLATE,
            &[&invoke_command],
            |row| {
                let id: i64 = row.get(0)?;
                return Ok(id);
            },
        )?;
        Ok(row_id)
    }

    pub fn upsert_user_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        user_name: &String,
        choice: &String,
    ) -> Result<(), GenericError> {
        self.db.get_connection()?.execute(
            sql::UPSERT_USER_VOTES,
            &[&chat_id, &message_id, &user_name, &choice],
        )?;
        Ok(())
    }

    pub fn select_voted_users_usernames(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Vec<String>, GenericError> {
        let connection = self.db.get_connection()?;
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS)?;
        let result = statement
            .query_map(&[&chat_id, &message_id], |row| {
                return Ok(String::from(row.get::<_, String>(2)?));
            })?
            .filter_map(|value| {
                return match value {
                    Err(err) => {
                        warn!("[select_voted_users_usernames] Got error while processing db response: {}", err);
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
    ) -> Result<Vec<UserVote>, GenericError> {
        let connection = self.db.get_connection()?;
        let mut statement = connection.prepare(sql::SELECT_VOTED_USERS)?;
        let result = statement
            .query_map(&[&chat_id, &message_id], |row| {
                let id = row.get(0)?;
                let vote_id = row.get(1)?;
                let username = row.get(2)?;
                let choice = row.get(3)?;

                return Ok(UserVote {
                    id,
                    vote_id,
                    username,
                    choice,
                });
            })?
            .filter_map(|value| {
                return match value {
                    Err(err) => {
                        warn!(
                            "[select_voted_users] Got error while processing db response: {}",
                            err
                        );
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
    ) -> Result<Option<Vote>, GenericError> {
        let connection = self.db.get_connection()?;
        let mut statement = connection.prepare(sql::SELECT_VOTE)?;
        let result = statement
            .query_row(&[&chat_id, &message_id], |row| {
                let id = row.get(0)?;
                let chat_id = row.get(1)?;
                let message_id = row.get(2)?;
                let text = row.get(3)?;
                let template_id = row.get(4)?;
                let result = Vote {
                    id,
                    chat_id,
                    message_id,
                    text,
                    template_id,
                };
                return Ok(result);
            })
            .optional()?;
        return Ok(result);
    }

    pub fn select_vote_template_by_message(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Option<VoteTemplate>, GenericError> {
        let connection = self.db.get_connection()?;
        let mut statement = connection.prepare(sql::SELECT_VOTE_TEMPLATE_BY_MESSAGE)?;
        let result = statement
            .query_row(&[&chat_id, &message_id], |row| {
                let id = row.get(0)?;
                let invoke_command = row.get(1)?;

                let result = VoteTemplate { id, invoke_command };
                return Ok(result);
            })
            .optional()?;
        return Ok(result);
    }
}
