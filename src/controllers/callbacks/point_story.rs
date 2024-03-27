mod dismiss;
mod open;
mod story_point;

use crate::controllers;
use crate::controllers::callbacks::point_story::open::to_open_callback_request;
use crate::controllers::callbacks::point_story::story_point::to_story_point_request;
use crate::controllers::callbacks::CallbackRequest;
use crate::controllers::handler::{HandlerTr, MaybeError};
use crate::dependencies::Dependencies;
use crate::views::callback::point_story;
use async_trait::async_trait;

enum CallbackAction {
    PointStory,
    Dismiss,
    Open,
    NoAction,
}

impl std::convert::From<&str> for CallbackAction {
    fn from(value: &str) -> Self {
        return detect_callback_action(value);
    }
}

fn detect_callback_action(text: &str) -> CallbackAction {
    if text.parse::<i32>().is_ok() {
        return CallbackAction::PointStory;
    }

    return match text {
        "Dismiss" => CallbackAction::Dismiss,
        "Open" => CallbackAction::Open,
        _ => CallbackAction::NoAction,
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
                CallbackAction::PointStory => {
                    controllers::callbacks::point_story::story_point::handle(
                        to_story_point_request(request)?,
                        dependencies,
                    )
                    .await?
                }
                CallbackAction::Dismiss => {
                    controllers::callbacks::point_story::dismiss::handle(request, dependencies)
                        .await?;
                }
                CallbackAction::Open => {
                    controllers::callbacks::point_story::open::handle(
                        to_open_callback_request(request)?,
                        dependencies,
                    )
                    .await?;
                }
                CallbackAction::NoAction => {
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
