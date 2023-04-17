pub enum VoteState {
    Init,
    InProcess,
    Finished,
}

pub struct Vote {
    pub id: i64,
    pub chat_id: String,
    pub message_id: String,
    pub text: String,
}

pub struct UserVote {
    pub vote_id: i64,
    pub username: String,
    pub choice: String,
}
