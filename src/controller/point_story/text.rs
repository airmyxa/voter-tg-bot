use crate::models::point_story::text::{InProgressText, InitText, ResultText, UserVotes};
use crate::models::vote::{UserVote, VoteState};

pub trait Text: ToString {}

impl ToString for InitText {
    fn to_string(&self) -> String {
        return self.task.to_string();
    }
}

impl InProgressText {
    fn ready_users_to_string(&self) -> String {
        let mentions: Vec<String> = self
            .ready_users
            .iter()
            .map(|username| {
                return format!("@{}", username);
            })
            .collect();
        return mentions.join(", ");
    }
}

impl ToString for InProgressText {
    fn to_string(&self) -> String {
        return format!("{}\n\nVoted: {}", self.task, self.ready_users_to_string());
    }
}

impl ResultText {
    fn prepare_vote_results(&self) -> String {
        let mut user_results = Vec::<String>::new();
        for UserVote {
            id,
            vote_id,
            username,
            choice,
        } in &self.user_votes
        {
            user_results.push(format!("@{} = {}", username, choice));
        }
        return user_results.join(", ");
    }
}

impl ToString for ResultText {
    fn to_string(&self) -> String {
        return format!(
            "{}\n\nVote results: {}",
            self.text,
            self.prepare_vote_results()
        );
    }
}

impl Text for InitText {}
impl Text for InProgressText {}
impl Text for ResultText {}
