use crate::controllers::callbacks::CallbackRequest;
use crate::controllers::handler::{GenericError, HandlerTr, MaybeError};
use crate::dependencies::Dependencies;
use crate::models::point_story::text::ResultText;
use crate::views::error::ValidationError;
use async_trait::async_trait;
use teloxide::requests::Requester;
use teloxide::types::Message;

#[derive(Debug)]
pub struct OpenCallbackRequest {
    message: Message,
    data: CallbackRequest,
}

struct Handler {}

#[async_trait]
impl HandlerTr<OpenCallbackRequest, Dependencies> for Handler {
    async fn handle(self, request: OpenCallbackRequest, dependencies: Dependencies) -> MaybeError {
        self.process(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn process(self, request: OpenCallbackRequest, dependencies: Dependencies) -> MaybeError {
        let chat_id = request.message.chat.id.to_string();
        let message_id = request.message.id.to_string();

        let vote_results = dependencies
            .db_requester
            .select_voted_users(&chat_id, &message_id)?;
        let vote = dependencies
            .db_requester
            .select_vote(&chat_id, &message_id)?;

        let vote = if let Some(vote) = vote {
            vote
        } else {
            return Err(Box::new(ValidationError::new(String::from(format!(
                "Cannot find vote with chat_id = {} and message_id = {}",
                chat_id, message_id
            )))));
        };

        let new_text = ResultText {
            text: vote.text,
            user_votes: vote_results,
        };

        request
            .data
            .bot
            .edit_message_text(chat_id, request.message.id, new_text.to_string())
            .await?;

        request
            .data
            .bot
            .answer_callback_query(request.data.query.id)
            .await?;

        Ok(())
    }
}

pub async fn handle(request: OpenCallbackRequest, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}

pub fn to_open_callback_request(
    request: CallbackRequest,
) -> Result<OpenCallbackRequest, GenericError> {
    return match &request.query.message {
        Some(message) => {
            let result = OpenCallbackRequest {
                message: message.clone(),
                data: request,
            };
            Ok(result)
        }
        None => Err(Box::new(ValidationError::new(String::from(
            "Open callback request parse error. Text is required field.",
        )))),
    };
}
