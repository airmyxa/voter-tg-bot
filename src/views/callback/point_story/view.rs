use crate::dependencies::Dependencies;
use crate::views::callback::point_story;
use crate::views::callback::point_story::open::view::to_open_callback_request;
use crate::views::callback::point_story::story_point::view::to_story_point_request;
use crate::views::callback::request::CallbackRequest;
use crate::views::handler::HandlerTr;
use crate::views::handler::MaybeError;
use async_trait::async_trait;

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
    async fn handle(self, request: CallbackRequest, dependencies: Dependencies) -> MaybeError {
        self.dispatch(request, dependencies).await?;
        Ok(())
    }
}

impl Handler {
    async fn dispatch(self, request: CallbackRequest, dependencies: Dependencies) -> MaybeError {
        if let Some(data) = &request.query.data {
            match detect_callback_action(&data) {
                Some(CallbackAction::PointStory) => {
                    point_story::story_point::view::handle(
                        to_story_point_request(request)?,
                        dependencies,
                    )
                    .await?
                }
                Some(CallbackAction::Dismiss) => {
                    point_story::dismiss::view::handle(request, dependencies).await?;
                }
                Some(CallbackAction::Open) => {
                    point_story::open::view::handle(
                        to_open_callback_request(request)?,
                        dependencies,
                    )
                    .await?;
                }
                None => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

pub async fn handle(request: CallbackRequest, dependencies: Dependencies) -> MaybeError {
    let handler = Handler {};
    handler.handle(request, dependencies).await?;
    Ok(())
}
