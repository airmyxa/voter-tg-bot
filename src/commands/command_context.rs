use crate::tg::requester::TgRequester;

pub struct CommandParams {
    pub command_string: String,
    pub text: String,
    pub chat_id: i64,
    pub thread_id: i64,
}

pub struct CommandDependencies {
    requester: &'static TgRequester,
}
