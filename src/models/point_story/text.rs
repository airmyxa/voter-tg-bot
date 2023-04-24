use crate::models::vote::UserVote;

pub type ReadyUsers = Vec<String>;
pub type UserVotes = Vec<UserVote>;

pub struct InitText {
    pub task: String,
}

pub struct InProgressText {
    pub task: String,
    pub ready_users: ReadyUsers,
}

pub struct ResultText {
    pub text: String,
    pub user_votes: UserVotes,
}
