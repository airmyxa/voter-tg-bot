use log::info;
use async_trait::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use crate::views::commands::point_story::keyboard;
use crate::views::handler::{Dependencies, HandlerResult, HandlerTr};
use crate::views::message::view::MessageRequest;

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies)
                    -> HandlerResult {
        info!("Start handling point story request");
        self.send_initial_keyboard_message(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn send_initial_keyboard_message(self, request: MessageRequest, dependencies: Dependencies)
                               -> HandlerResult {
        let keyboard = keyboard::KeyboardBuilder::make_keyboard();
        request.bot.send_message(request.message.chat.id, "Choose your option:").reply_markup(keyboard).await?;
        Ok(())
    }
}

pub async fn handle(request: MessageRequest, dependencies: Dependencies) -> HandlerResult {
    let handler = Handler{};
    handler.handle(request, dependencies).await?;
    Ok(())
}