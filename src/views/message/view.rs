use std::error::Error;
use std::fmt::Debug;
use log::{debug, info};
use teloxide::Bot;
use teloxide::types::{Me, Message};
use teloxide::utils::{command::BotCommands};
use crate::views::handler::HandlerTr;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
    #[command(description = "Point story")]
    PointStory
}

type MessageHandlerResponse = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Debug)]
pub struct MessageRequest {
    bot: Bot,
    message: Message,
    me: Me,
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

pub struct Handler {}

impl<D> HandlerTr<MessageRequest, D> for Handler {
    fn handle(self, request: MessageRequest, dependencies: D) {
        debug!("do_handle scope. Got message: {}", request.message.text().unwrap());
    }
}

pub async fn handle(bot: Bot,
                    message: Message,
                    me: Me) -> MessageHandlerResponse {
    let handler = Handler {};
    handler.handle(MessageRequest::new(bot, message, me), ());
    Ok(())
}