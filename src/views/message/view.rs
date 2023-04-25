use crate::dependencies::Dependencies;
use crate::views::commands;
use crate::views::handler::HandlerTr;
use crate::views::handler::MaybeError;
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
            None => return commands::help::view::handle(request, dependencies).await,
            Some(value) => value,
        };

        match BotCommands::parse(raw_text, request.me.username()) {
            Ok(Command::Help) => commands::help::view::handle(request, dependencies).await?,
            Ok(Command::Start) => commands::help::view::handle(request, dependencies).await?,
            Ok(Command::PointStory) => {
                commands::point_story::view::handle(
                    commands::point_story::view::to_pointstory_request(request)?,
                    dependencies,
                )
                .await?
            }
            Err(_) => commands::help::view::handle(request, dependencies).await?,
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
