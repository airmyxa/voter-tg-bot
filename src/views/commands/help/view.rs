use crate::dependencies::Dependencies;
use crate::views::commands::Command;
use crate::views::handler::{HandlerTr, MaybeError};
use crate::views::message::view::MessageRequest;
use async_trait::async_trait;
use log::info;
use teloxide::requests::Requester;
use teloxide::utils::command::BotCommands;

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies) -> MaybeError {
        info!(
            "Start handling help request",
        );
        self.send_help_message(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn send_help_message(self, request: MessageRequest, _: Dependencies) -> MaybeError {
        request
            .bot
            .send_message(request.message.chat.id, Command::descriptions().to_string())
            .await?;
        Ok(())
    }
}

pub async fn handle(request: MessageRequest, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
