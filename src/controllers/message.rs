use crate::controllers;
use crate::controllers::commands::point_story::to_pointstory_request;
use crate::controllers::handler::{HandlerTr, MaybeError};
use crate::dependencies::Dependencies;
use crate::views::commands;
use async_trait::async_trait;
use commands::Command;
use log::info;
use std::fmt::Debug;
use teloxide::types::{Me, Message};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

#[derive(Debug)]
pub struct MessageRequest {
    pub bot: Bot,
    pub message: Message,
    pub me: Me,
}

impl MessageRequest {
    fn new(bot: Bot, message: Message, me: Me) -> Self {
        MessageRequest { bot, message, me }
    }
}

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies) -> MaybeError {
        info!(
            "Start handling message request.\
               text={}",
            request.message.text().unwrap_or_default()
        );
        self.dispatch(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn dispatch(self, request: MessageRequest, dependencies: Dependencies) -> MaybeError {
        let raw_text = match request.message.text() {
            None => return controllers::commands::help::handle(request, dependencies).await,
            Some(value) => value,
        };

        match BotCommands::parse(raw_text, request.me.username()) {
            Ok(Command::Help) => controllers::commands::help::handle(request, dependencies).await?,
            Ok(Command::Start) => {
                controllers::commands::help::handle(request, dependencies).await?
            }
            Ok(Command::PointStory) => {
                controllers::commands::point_story::handle(
                    to_pointstory_request(request)?,
                    dependencies,
                )
                .await?
            }
            Err(_) => {}
        }

        Ok(())
    }
}

pub async fn handle(bot: Bot, message: Message, me: Me, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};

    handler
        .handle(MessageRequest::new(bot, message, me), dependencies)
        .await?;
    Ok(())
}
