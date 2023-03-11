use teloxide::types::CallbackQuery;
use teloxide::Bot;

#[derive(Debug)]
pub struct CallbackRequest {
    pub bot: Bot,
    pub query: CallbackQuery,
}

impl CallbackRequest {
    fn new(bot: Bot, query: CallbackQuery) -> Self {
        CallbackRequest { bot, query }
    }
}
