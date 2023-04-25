use crate::actions;
use crate::dependencies::Dependencies;
use crate::models::point_story::text::InProgressText;
use crate::models::vote::VoteState;
use crate::views::callback::request::CallbackRequest;
use crate::views::error::ValidationError;
use crate::views::handler::MaybeError;
use crate::views::handler::{GenericError, HandlerTr};
use async_trait::async_trait;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::requests::Requester;
use teloxide::types::Message;

#[derive(Debug)]
pub struct StoryPointCallbackRequest {
    query_data: String,
    message: Message,
    extra: CallbackRequest,
}

struct Handler {}

#[async_trait]
impl HandlerTr<StoryPointCallbackRequest, Dependencies> for Handler {
    async fn handle(
        self,
        request: StoryPointCallbackRequest,
        dependencies: Dependencies,
    ) -> MaybeError {
        self.process(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn process(
        self,
        request: StoryPointCallbackRequest,
        dependencies: Dependencies,
    ) -> MaybeError {
        let pointstory = &request.query_data;
        let message = &request.message;

        let user = &request.extra.query.from;

        let username = match &user.username {
            Some(username) => username.clone(),
            None => user.full_name(),
        };

        dependencies.requester.upsert_user_vote(
            &message.chat.id.to_string(),
            &message.id.to_string(),
            &username,
            &pointstory,
        )?;

        let voted_users = dependencies
            .requester
            .select_voted_users_usernames(&message.chat.id.to_string(), &message.id.to_string())?;

        let vote = dependencies
            .requester
            .select_vote(&message.chat.id.to_string(), &message.id.to_string())?;

        let vote = if let Some(vote) = vote {
            vote
        } else {
            return Err(Box::new(ValidationError::new(format!(
                "Cannot find vote with chat_id = {} and message_id = {}",
                message.chat.id.to_string(),
                message.id.to_string()
            ))));
        };

        let new_text = InProgressText {
            task: vote.text,
            ready_users: voted_users,
        };
        request
            .extra
            .bot
            .edit_message_text(message.chat.id, message.id, new_text.to_string())
            .reply_markup(actions::point_story::make_keyboard(VoteState::InProcess))
            .await?;

        request
            .extra
            .bot
            .answer_callback_query(request.extra.query.id)
            .await?;

        Ok(())
    }
}

pub async fn handle(request: StoryPointCallbackRequest, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}

pub fn to_story_point_request(
    request: CallbackRequest,
) -> Result<StoryPointCallbackRequest, GenericError> {
    let query_data = match &request.query.data {
        Some(query_data) => query_data,
        None => {
            return Err(Box::new(ValidationError::new(String::from(
                "Cannot process vote without query data",
            ))));
        }
    }
    .clone();
    let message = match &request.query.message {
        Some(message) => message,
        None => {
            return Err(Box::new(ValidationError::new(String::from(
                "Cannot process vote without message object",
            ))));
        }
    }
    .clone();
    let result = StoryPointCallbackRequest {
        query_data,
        message,
        extra: request,
    };
    return Ok(result);
}
