use crate::views::callback::point_story;
use crate::views::callback::request::CallbackRequest;
use crate::views::handler::Dependencies;
use crate::views::handler::HandlerResult;
use crate::views::handler::HandlerTr;
use async_trait::async_trait;
use log::{debug, info};

enum CallbackAction {
    PointStory,
    Dismiss,
    Open,
}

fn detect_callback_action(text: &str) -> Option<CallbackAction> {
    if text.parse::<i32>().is_ok() {
        return Some(CallbackAction::PointStory);
    }

    return match text {
        "Dismiss" => Some(CallbackAction::Dismiss),
        "Open" => Some(CallbackAction::Open),
        _ => None,
    };
}

struct Handler {}

#[async_trait]
impl HandlerTr<CallbackRequest, Dependencies> for Handler {
    async fn handle(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        self.dispatch(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn dispatch(self, request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
        if let Some(data) = &request.query.data {
            match detect_callback_action(&data) {
                Some(CallbackAction::PointStory) => {
                    point_story::story_point::view::handle(request, dependencies).await?
                }
                Some(CallbackAction::Dismiss) => {
                    return Ok(());
                }
                Some(CallbackAction::Open) => {
                    return Ok(());
                }
                None => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

pub async fn handle(request: CallbackRequest, dependencies: Dependencies) -> HandlerResult {
    let handler = Handler {};
    handler.handle(request, Dependencies {}).await?;
    Ok(())
}
