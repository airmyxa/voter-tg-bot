use crate::db::sqlite::database::{SQLiteConnectionType, SQLiteDb, SQLiteSettings};
use crate::views::callback::point_story;
use crate::views::callback::request::CallbackRequest;
use crate::views::callback::view::CallbackForTemplate::PointStory;
use crate::views::handler::Dependencies;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use log::{debug, info};
use std::sync::Arc;
use teloxide::requests::Requester;
use teloxide::types::CallbackQuery;
use teloxide::Bot;

enum CallbackForTemplate {
    PointStory,
}

fn detect_callback_template(text: &str) -> Option<CallbackForTemplate> {
    let lines = text.split("\n").collect::<Vec<&str>>();
    let line_parts = lines.first().unwrap().split(" ").collect::<Vec<&str>>();
    let command = line_parts.first().unwrap();

    match command {
        &"/pointstory" => Some(PointStory),
        _ => None,
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

        self.dispatch(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn dispatch(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        request
            .bot
            .answer_callback_query(request.query.id.clone())
            .await?;

        if let Some(message) = &request.query.message {
            if let Some(text) = message.text() {
                match detect_callback_template(&text) {
                    Some(PointStory) => point_story::view::handle(request, dependencies).await?,
                    None => {
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }
}

pub async fn handle(bot: Bot, query: CallbackQuery) -> HandlerResult {
    let dependencies = Dependencies::new(Arc::new(SQLiteDb::connect(SQLiteSettings::new(
        SQLiteConnectionType::File(String::from("voter.db")),
    ))));

    let handler = Handler {};
    handler
        .handle(CallbackRequest { bot, query }, dependencies)
        .await?;
    Ok(())
}
