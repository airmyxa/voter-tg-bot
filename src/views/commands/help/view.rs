use crate::dependencies::Dependencies;
use crate::views::commands::Command;
use crate::views::handler::{HandlerTr, MaybeError};
use crate::views::message::view::MessageRequest;
use async_trait::async_trait;
use log::info;
use teloxide::payloads::{SendInvoiceSetters, SendMessageSetters};
use teloxide::requests::Requester;
use teloxide::utils::command::BotCommands;

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies) -> MaybeError {
        info!("Start handling help request",);
        self.send_help_message(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn send_help_message(self, request: MessageRequest, _: Dependencies) -> MaybeError {
        let thread_id = request.message.thread_id;
        let mut request = request
            .bot
            .send_message(request.message.chat.id, Command::descriptions().to_string());
        request.message_thread_id = thread_id;
        request.await?;
        Ok(())
    }
}

pub async fn handle(request: MessageRequest, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
