use std::error::Error;
use std::fmt::Debug;
use async_trait::async_trait;
use log::{debug, info};
use teloxide::Bot;
use teloxide::types::{Me, Message};
use teloxide::utils::{command::BotCommands};
use crate::views;
use crate::views::handler::HandlerTr;
use crate::views::handler::Dependencies;
use crate::views::handler::HandlerResult;
use crate::views::commands;
use commands::Command;

#[derive(Debug)]
pub struct MessageRequest {
    pub bot: Bot,
    pub message: Message,
    pub me: Me,
}

impl MessageRequest {
    fn new(bot: Bot,
           message: Message,
           me: Me) -> Self {
        MessageRequest {
            bot,
            message,
            me,
        }
    }
}

struct Handler {}

#[async_trait]
impl HandlerTr<MessageRequest, Dependencies> for Handler {
    async fn handle(self, request: MessageRequest, dependencies: Dependencies)
              -> HandlerResult {
        info!("Start handling message request: {}", request.message.text().unwrap());
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
            Ok(Command::Help) => {
                commands::help::view::handle(request, dependencies).await?
            }
            Ok(Command::Start) => {
                commands::help::view::handle(request, dependencies).await?
            }
            Ok(Command::PointStory) => {
                commands::help::view::handle(request, dependencies).await?
            }
            Err(_) => {
                commands::help::view::handle(request, dependencies).await?
            }
        }

        Ok(())
    }
}

pub async fn handle(bot: Bot,
                    message: Message,
                    me: Me) -> HandlerResult {
    let handler = Handler {};
    handler.handle(MessageRequest::new(bot, message, me), Dependencies {}).await?;
    Ok(())
}