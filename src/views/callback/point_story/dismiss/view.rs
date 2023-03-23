use crate::dependencies::Dependencies;
use crate::views::callback::request::CallbackRequest;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use log::{debug, info};
use teloxide::requests::Requester;

struct Handler {}

#[async_trait]
impl HandlerTr<CallbackRequest, Dependencies> for Handler {
    async fn handle(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        self.process(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn process(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        if let Some(message) = request.query.message {
            request
                .bot
                .delete_message(message.chat.id, message.id)
                .await?;
        }

        Ok(())
    }
}

pub async fn handle(request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
