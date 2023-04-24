pub enum VoteState {
    Init,
    InProcess,
    Finished,
}

pub struct VoteTemplate {
    pub id: i64,
    pub invoke_command: String,
}

pub struct Vote {
    pub id: i64,
    pub chat_id: String,
    pub message_id: String,
    pub text: String,
    pub template_id: i64,
}

pub struct UserVote {
    pub id: i64,
    pub vote_id: i64,
    pub username: String,
    pub choice: String,
}
