use crate::actions;
use crate::dependencies::Dependencies;
use crate::models::vote::VoteState;
use crate::views::error::ValidationError;
use crate::views::handler::{GenericError, HandlerTr, MaybeError};
use crate::views::message::view::MessageRequest;
use async_trait::async_trait;
use log::info;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use teloxide::types::{Me, Message};
use teloxide::Bot;

#[derive(Debug)]
struct PointStoryRequestData {
    pub bot: Bot,
    pub message: Message,
    pub me: Me,
}

#[derive(Debug)]
pub struct PointStoryRequest {
    text: String,
    data: PointStoryRequestData,
}

struct Handler {}

#[async_trait]
impl HandlerTr<PointStoryRequest, Dependencies> for Handler {
    async fn handle(self, request: PointStoryRequest, dependencies: Dependencies) -> MaybeError {
        info!("Start handling 'point story' request");

        self.delete_user_message(&request, &dependencies).await?;

        let response = self
            .send_initial_keyboard_message(&request, &dependencies)
            .await?;

        let chat_id = response.chat.id.to_string();
        let message_id = response.id.to_string();

        info!(
            "Inserting new vote.\n\
            chat_id = {}\n\
            message_id = {}\n\
            text = {}",
            chat_id, message_id, request.text
        );

        dependencies.requester.insert_vote(
            &chat_id,
            &message_id,
            &request.text,
            &String::from("/pointstory"),
        )?;

        Ok(())
    }
}

impl Handler {
    async fn send_initial_keyboard_message(
        &self,
        request: &PointStoryRequest,
        dependencies: &Dependencies,
    ) -> Result<Message, GenericError> {
        let keyboard = actions::point_story::make_keyboard(VoteState::Init);
        let text = &request.text;
        let response = request
            .data
            .bot
            .send_message(request.data.message.chat.id, text)
            .reply_markup(keyboard)
            .await?;
        Ok(response)
    }

    async fn delete_user_message(
        &self,
        request: &PointStoryRequest,
        dependencies: &Dependencies,
    ) -> MaybeError {
        request
            .data
            .bot
            .delete_message(request.data.message.chat.id, request.data.message.id)
            .await?;
        Ok(())
    }
}

pub async fn handle(request: PointStoryRequest, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}

pub fn to_pointstory_request(request: MessageRequest) -> Result<PointStoryRequest, GenericError> {
    let opt_text = request.message.text().map(str::to_string);
    return match opt_text {
        Some(text) => {
            let data = PointStoryRequestData {
                bot: request.bot,
                message: request.message,
                me: request.me,
            };
            let result = PointStoryRequest {
                text: text.to_string(),
                data: data,
            };
            Ok(result)
        }
        None => Err(Box::new(ValidationError::new(String::from(
            "Pointstory request parse error. Text is required field",
        )))),
    };
}
