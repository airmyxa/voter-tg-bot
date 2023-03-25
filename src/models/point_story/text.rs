use crate::db::sqlite::models::user_votes::UserVote;

type ReadyUsers = Vec<String>;

static READY_USERS_PREFIX: &'static str = "Voted: ";

pub struct Text {
    task: String,
    ready_users: ReadyUsers,
}

impl Text {
    pub fn new(task: String, ready_users: ReadyUsers) -> Self {
        Text { task, ready_users }
    }

    pub fn to_string(&self) -> String {
        return format!("{}\n\nVoted: {}", self.task, self.ready_users_to_string());
    }

    pub fn add_ready_user(&mut self, user: String) {
        if !self.ready_users.contains(&user) {
            self.ready_users.push(user);
        }
    }

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

pub struct ResultText {
    text: String,
    user_votes: Vec<UserVote>,
}

impl ResultText {
    pub fn new(text: String, user_votes: Vec<UserVote>) -> Self {
        ResultText { text, user_votes }
    }

    pub fn to_string(&self) -> String {
        return format!(
            "{}\n\nVote results: {}",
            self.text,
            self.prepare_vote_results()
        );
    }

    fn prepare_vote_results(&self) -> String {
        let mut user_results = Vec::<String>::new();
        for UserVote {
            vote_id,
            username,
            query_data,
        } in &self.user_votes
        {
            user_results.push(format!("@{} = {}", username, query_data));
        }
        return user_results.join(", ");
    }
}
