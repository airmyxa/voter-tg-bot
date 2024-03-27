pub mod point_story;

use crate::controllers;
use crate::controllers::handler::{GenericError, HandlerTr, MaybeError};
use crate::dependencies::Dependencies;
use crate::views::error::ValidationError;
use async_trait::async_trait;
use log::info;
use teloxide::requests::Requester;
use teloxide::types::{CallbackQuery, Message};
use teloxide::Bot;

#[derive(Debug)]
pub struct CallbackRequest {
    pub bot: Bot,
    pub message: Message,
    pub text: String,
    pub data: String,
    pub query: CallbackQuery,
}

enum CallbackForTemplate {
    PointStory,
}

fn parse_callback_from_invoke_command(text: &String) -> Result<CallbackForTemplate, GenericError> {
    return match text.as_str() {
        "/pointstory" => Ok(CallbackForTemplate::PointStory),
        _ => Err(Box::new(ValidationError::new(format!(
            "Cannot parse invoke command '{}' for vote template",
            text
        )))),
    };
}

struct Handler {}

#[async_trait]
impl HandlerTr<CallbackRequest, Dependencies> for Handler {
    async fn handle(
        self,
        request: CallbackRequest,
        dependencies: Dependencies,
    ) -> MaybeError {
        info!(
            "Start handling callback request:\n \
               callback={},\n \
               text={}",
            request.data, &request.text,
        );

        self.dispatch(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    fn get_message_template(
        &self,
        request: &CallbackRequest,
        dependencies: &Dependencies,
    ) -> Result<CallbackForTemplate, GenericError> {
        let chat_id = &request.message.chat.id.to_string();
        let message_id = &request.message.id.to_string();

        let vote_template = dependencies
            .db_requester
            .select_vote_template_by_message(&chat_id, &message_id)?;
        if let Some(vote_template) = &vote_template {
            return parse_callback_from_invoke_command(&vote_template.invoke_command);
        } else {
            return Err(Box::new(ValidationError::new(format!(
                "Cannot find vote template for chat_id '{}' and message_id '{}'",
                chat_id, message_id
            ))));
        }
    }

    async fn dispatch(
        &self,
        request: CallbackRequest,
        dependencies: Dependencies,
    ) -> MaybeError {
        request
            .bot
            .answer_callback_query(request.query.id.clone())
            .await?;

        match self.get_message_template(&request, &dependencies)? {
            CallbackForTemplate::PointStory => {
                controllers::callbacks::point_story::handle(request, dependencies).await?
            }
        }

        Ok(())
    }
}

pub async fn handle(
    bot: Bot,
    query: CallbackQuery,
    dependencies: Dependencies,
) -> MaybeError {
    let message = if let Some(message) = &query.message {
        message
    } else {
        return Err(Box::new(
            ValidationError::new(String::from("Cannot process vote without message object"))
        ));
    }
    .clone();

    let text = if let Some(text) = message.text() {
        text
    } else {
        return Err(
            Box::new(ValidationError::new(String::from("Cannot process vote without text")))
        );
    }
    .to_string();

    let data = if let Some(data) = &query.data {
        data
    } else {
        return Err(
            Box::new(ValidationError::new(String::from("Cannot process vote without query data")))
        );
    }
    .clone();

    let handler = Handler {};
    handler
        .handle(
            CallbackRequest {
                bot,
                message,
                text,
                data,
                query,
            },
            dependencies,
        )
        .await?;
    Ok(())
}
