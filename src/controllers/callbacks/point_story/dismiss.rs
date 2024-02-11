use crate::controllers::callbacks::CallbackRequest;
use crate::controllers::handler::{HandlerTr, MaybeError};
use crate::dependencies::DependenciesComponent;
use async_trait::async_trait;
use teloxide::requests::Requester;

struct Handler {}

#[async_trait]
impl HandlerTr<CallbackRequest, Dependenciesfor Handler {
    async fn handle(
        self,
        request: CallbackRequest,
        dependencies: DependenciesComponent,
    ) -> MaybeError {
        self.process(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn process(self, request: CallbackRequest, _: DependenciesComponent) -> MaybeError {
        if let Some(message) = request.query.message {
            request
                .bot
                .delete_message(message.chat.id, message.id)
                .await?;
        }

        Ok(())
    }
}

pub async fn handle(request: CallbackRequest, dependencies: DependenciesComponent) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
