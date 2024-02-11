use crate::{
    components_container::ComponentsContainer,
    models::point_story::keyboard::Keyboard,
    tg::requester::{self, InitVoteRequest, Requester},
};
use std::sync::Arc;

use crate::components_container::Component;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::Bot;

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
        Bot::from_env();
    }

    fn component_name(&self) -> &'static str {
        return "teloxide-requester";
    }
}

impl TgRequester {
    fn send_vote(&self, request: InitVoteRequest) -> anyhow::Result<()> {
        let mut tg_request = self
            .bot
            .send_message(request.chat_id, request.text)
            .reply_markup(request.keyboard.into());
        tg_request.message_thread_id = request.thread_id;
        tg_request.await?;
        Ok(())
    }
}
