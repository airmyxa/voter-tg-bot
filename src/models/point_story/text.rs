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

    pub fn from_string(text: &String) -> Self {
        let mut parts = text.split("\n");

        let task = if let Some(task_) = parts.next() {
            task_.to_string()
        } else {
            String::default()
        };

        let ready_users: ReadyUsers = if let Some(ready_users) = parts.next() {
            let ready_users = String::from(&ready_users[READY_USERS_PREFIX.len()..]);
            ready_users
                .split(" ")
                .map(|x| x.to_string())
                .collect::<ReadyUsers>()
        } else {
            ReadyUsers::default()
        };

        Text { task, ready_users }
    }

    pub fn to_string(&self) -> String {
        return format!("{}\nVoted: {}", self.task, self.ready_users_to_string());
    }

    pub fn add_ready_user(&mut self, user: String) {
        if !self.ready_users.contains(&user) {
            self.ready_users.push(user);
        }
    }

    fn ready_users_to_string(&self) -> String {
        return self.ready_users.join(" ");
    }
}
