use crate::dependencies::Dependencies;
use crate::models::point_story::keyboard::KeyboardBuilder;
use crate::models::point_story::text::Text;
use crate::views::callback::request::CallbackRequest;
use crate::views::error::ValidationError;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use log::{debug, info};
use teloxide::payloads::EditMessageTextSetters;
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
        if request.query.data.is_none() {
            return Err(Box::new(ValidationError::new(String::from(
                "query.data cannot be None",
            ))));
        }

        let pointstory = &request.query.data.as_ref().unwrap();
        let message = &request.query.message.as_ref().unwrap();

        dependencies.requester.upsert_user_vote(
            &message.chat.id.to_string(),
            &message.id.to_string(),
            &request
                .query
                .from
                .username
                .as_ref()
                .unwrap_or(&request.query.from.full_name()),
            &pointstory,
        )?;

        let voted_users = dependencies
            .requester
            .select_voted_users_usernames(&message.chat.id.to_string(), &message.id.to_string())?;

        let vote = dependencies
            .requester
            .select_vote(&message.chat.id.to_string(), &message.id.to_string())?;

        if vote.is_none() {
            return Err(Box::new(ValidationError::new(String::from(format!(
                "Cannot find vote with chat_id = {} and message_id = {}",
                message.chat.id.to_string(),
                message.id.to_string()
            )))));
        }

        let vote = vote.unwrap();

        let new_text = Text::new(vote.text, voted_users);
        request
            .bot
            .edit_message_text(message.chat.id, message.id, new_text.to_string())
            .reply_markup(KeyboardBuilder::make_keyboard())
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
