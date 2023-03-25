use crate::dependencies::Dependencies;
use crate::models::point_story::text::ResultText;
use crate::views::callback::request::CallbackRequest;
use crate::views::error::ValidationError;
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
        let message = request.query.message.as_ref().unwrap();
        let chat_id = message.chat.id.to_string();
        let message_id = message.id.to_string();

        let vote_results = dependencies
            .requester
            .select_voted_users(&chat_id, &message_id);
        let vote = dependencies.requester.select_vote(&chat_id, &message_id);
        if vote.is_none() {
            return Err(Box::new(ValidationError::new(String::from(format!(
                "Cannot find vote with chat_id = {} and message_id = {}",
                message.chat.id.to_string(),
                message.id.to_string()
            )))));
        }
        let vote = vote.unwrap();

        let new_text = ResultText::new(vote.text, vote_results);
        request
            .bot
            .edit_message_text(message.chat.id, message.id, new_text.to_string())
            .await?;

        request.bot.answer_callback_query(request.query.id).await?;

        Ok(())
    }
}

pub async fn handle(request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
