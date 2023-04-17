use crate::actions;
use crate::dependencies::Dependencies;
use crate::models::vote::VoteState;
use crate::views::commands::Command;
use crate::views::handler::{HandlerErr, HandlerResult, HandlerTr};
use crate::views::message::view::MessageRequest;
use async_trait::async_trait;
use log::info;
use teloxide::payloads::{ForwardMessageSetters, SendMessageSetters};
use teloxide::requests::Requester;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies) -> HandlerResult {
        info!("Start handling 'point story' request");

        self.delete_user_message(&request, &dependencies).await?;

        let text = request.message.text().unwrap_or_default().to_string();

        let response = self
            .send_initial_keyboard_message(request, &dependencies)
            .await?;

        let chat_id = response.chat.id.to_string();
        let message_id = response.id.to_string();

        info!(
            "Inserting new vote.\n\
            chat_id = {}\n\
            message_id = {}\n\
            text = {}",
            chat_id, message_id, text
        );

        dependencies
            .requester
            .insert_vote(&chat_id, &message_id, &text)?;

        Ok(())
    }
}

impl Handler {
    async fn send_initial_keyboard_message(
        &self,
        request: MessageRequest,
        dependencies: &Dependencies,
    ) -> Result<Message, HandlerErr> {
        let keyboard = actions::point_story::make_keyboard(VoteState::Init);
        let text = request.message.text().unwrap_or("");
        let response = request
            .bot
            .send_message(request.message.chat.id, text)
            .reply_markup(keyboard)
            .await?;
        Ok(response)
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
