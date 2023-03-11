use crate::models::point_story::keyboard::KeyboardBuilder;
use crate::models::point_story::text::Text;
use crate::views::callback::request::CallbackRequest;
use crate::views::handler::Dependencies;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use log::{debug, info};
use teloxide::payloads::EditMessageTextSetters;
use teloxide::requests::Requester;

struct Handler {}

#[async_trait]
impl HandlerTr<CallbackRequest, Dependencies> for Handler {
    async fn handle(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
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

pub async fn handle(request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
    let handler = Handler {};
    handler.handle(request, Dependencies {}).await?;
    Ok(())
}
