use crate::views;
use crate::views::commands;
use crate::views::handler::Dependencies;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use commands::Command;
use log::{debug, info};
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
    async fn handle(self, request: MessageRequest, dependencies: Dependencies) -> HandlerResult {
        info!(
            "Start handling message request.\
               text={}",
            request.message.text().unwrap_or("")
        );
        self.dispatch(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn dispatch(self, request: MessageRequest, dependencies: Dependencies) -> HandlerResult {
        let raw_text = request.message.text();
        if raw_text.is_none() {
            views::commands::help::view::handle(request, dependencies).await?;
            return Ok(());
        }

        let raw_text = raw_text.unwrap();
        match BotCommands::parse(raw_text, request.me.username()) {
            Ok(Command::Help) => commands::help::view::handle(request, dependencies).await?,
            Ok(Command::Start) => commands::help::view::handle(request, dependencies).await?,
            Ok(Command::PointStory) => {
                commands::point_story::view::handle(request, dependencies).await?
            }
            Err(_) => commands::help::view::handle(request, dependencies).await?,
        }

        Ok(())
    }
}

pub async fn handle(bot: Bot, message: Message, me: Me) -> HandlerResult {
    let handler = Handler {};
    handler
        .handle(MessageRequest::new(bot, message, me), Dependencies {})
        .await?;
    Ok(())
}
