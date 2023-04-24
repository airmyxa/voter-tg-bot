use teloxide::types::CallbackQuery;
use teloxide::types::Message;
use teloxide::Bot;

#[derive(Debug)]
pub struct CallbackRequest {
    pub bot: Bot,
    pub message: Message,
    pub text: String,
    pub data: String,
    pub query: CallbackQuery,
}
