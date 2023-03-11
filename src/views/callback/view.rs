use crate::models::point_story::keyboard::KeyboardBuilder;
use crate::models::point_story::text::Text;
use crate::views::handler::Dependencies;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use log::{debug, info};
use std::fmt::Debug;
use teloxide::payloads::EditMessageTextSetters;
use teloxide::requests::Requester;
use teloxide::types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Me, Message};
use teloxide::Bot;

#[derive(Debug)]
pub struct CallbackRequest {
    pub bot: Bot,
    pub query: CallbackQuery,
}

impl CallbackRequest {
    fn new(bot: Bot, query: CallbackQuery) -> Self {
        CallbackRequest { bot, query }
    }
}

struct Handler {}

#[async_trait]
impl HandlerTr<CallbackRequest, Dependencies> for Handler {
    async fn handle(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        let message: String = if let Some(message) = &request.query.message {
            message.text().unwrap_or("").to_string()
        } else {
            String::default()
        };

        info!(
            "Start handling callback request:\n \
               callback={},\n \
               text={}",
            request.query.data.as_ref().unwrap_or(&String::default()),
            &message,
        );

        self.process(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn process(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        if let Some(pointstory) = &request.query.data {
            if let Some(message) = request.query.message {
                let mut new_text =
                    Text::from_string(&message.text().unwrap_or_default().to_string());
                new_text.add_ready_user(
                    request
                        .query
                        .from
                        .mention()
                        .unwrap_or(request.query.from.full_name()),
                );
                request
                    .bot
                    .edit_message_text(message.chat.id, message.id, new_text.to_string())
                    .reply_markup(KeyboardBuilder::make_keyboard())
                    .await?;
            }

            request.bot.answer_callback_query(request.query.id).await?;
        }

        Ok(())
    }
}

pub async fn handle(bot: Bot, query: CallbackQuery) -> HandlerResult {
    let handler = Handler {};
    handler
        .handle(CallbackRequest::new(bot, query), Dependencies {})
        .await?;
    Ok(())
}
