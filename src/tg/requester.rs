use crate::{components_container::ComponentsContainer, models::point_story::keyboard::Keyboard};
use std::sync::Arc;

use crate::components_container::Component;
use teloxide::Bot;
use teloxide::{payloads::SendMessageSetters, types::ChatId};
use teloxide::{requests::Requester, types::InlineKeyboardMarkup};

pub struct InitVoteRequest {
    pub text: String,
    pub keyboard: Keyboard,
    pub chat_id: i64,
    pub thread_id: Option<i32>,
}

pub struct TgRequester {
    bot: Bot,
}

impl Component for TgRequester {
    fn create_component(components: &mut ComponentsContainer) -> Self {
        TgRequester {
            bot: Bot::from_env(),
        }
    }

    fn component_name(&self) -> &'static str {
        return "teloxide-requester";
    }
}

impl TgRequester {
    async fn send_vote(&self, request: InitVoteRequest) -> anyhow::Result<()> {
        let mut tg_request = self
            .bot
            .send_message(ChatId(request.chat_id), request.text)
            .reply_markup(InlineKeyboardMarkup::from(request.keyboard));
        tg_request.message_thread_id = request.thread_id;
        tg_request.await?;
        Ok(())
    }
}
