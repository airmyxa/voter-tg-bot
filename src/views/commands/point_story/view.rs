use crate::models::point_story::keyboard::KeyboardBuilder;
use crate::views::commands::point_story::validator::validate;
use crate::views::commands::Command;
use crate::views::handler::{Dependencies, HandlerResult, HandlerTr};
use crate::views::message::view::MessageRequest;
use async_trait::async_trait;
use log::info;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::utils::command::BotCommands;

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies) -> HandlerResult {
        info!("Start handling 'point story' request");

        self.delete_user_message(&request, &dependencies).await?;

        let validate_result = validate(&request);
        if let Err(error_message) = validate_result {
            self.send_error_message(error_message, request, dependencies)
                .await?;
            return Ok(());
        }

        self.send_initial_keyboard_message(request, dependencies)
            .await?;
        Ok(())
    }
}

impl Handler {
    async fn send_initial_keyboard_message(
        &self,
        request: MessageRequest,
        dependencies: Dependencies,
    ) -> HandlerResult {
        let keyboard = KeyboardBuilder::make_keyboard();
        let text = request.message.text().unwrap_or("");
        request
            .bot
            .send_message(request.message.chat.id, text)
            .reply_markup(keyboard)
            .await?;
        Ok(())
    }

    async fn send_error_message(
        &self,
        message: String,
        request: MessageRequest,
        dependencies: Dependencies,
    ) -> HandlerResult {
        let text = format!(
            "Error: {}\n{}",
            message,
            Command::descriptions().to_string()
        );
        request
            .bot
            .send_message(request.message.chat.id, text)
            .await?;
        Ok(())
    }

    async fn delete_user_message(
        &self,
        request: &MessageRequest,
        dependencies: &Dependencies,
    ) -> HandlerResult {
        request
            .bot
            .delete_message(request.message.chat.id, request.message.id)
            .await?;
        Ok(())
    }
}

pub async fn handle(request: MessageRequest, dependencies: Dependencies) -> HandlerResult {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
