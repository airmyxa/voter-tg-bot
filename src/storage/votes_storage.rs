use crate::controllers::handler::GenericError;
use crate::models::vote::{UserVote, Vote, VoteTemplate};

trait VoteStorageTr {
    fn add_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        text: &String,
        invoke_command: &String,
    ) -> Result<(), GenericError>;

    fn add_vote_template(&self, invoke_command: &String) -> Result<i64, GenericError>;

    fn change_user_vote(
        &self,
        chat_id: &String,
        message_id: &String,
        user_name: &String,
        choice: &String,
    ) -> Result<(), GenericError>;

    fn get_voted_users_usernames(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<(), GenericError>;

    fn get_voted_users(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Vec<UserVote>, GenericError>;

    fn get_vote(&self, chat_id: &String, message_id: &String)
        -> Result<Option<Vote>, GenericError>;

    fn get_vote_template_by_message(
        &self,
        chat_id: &String,
        message_id: &String,
    ) -> Result<Option<VoteTemplate>, GenericError>;
}
