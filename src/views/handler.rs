use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;

pub type HandlerErr = Box<dyn Error + Send + Sync>;
pub type HandlerResult = Result<(), HandlerErr>;

#[async_trait]
pub trait HandlerTr<R, D>
where
    R: Debug,
{
    async fn handle(self, request: R, dependencies: D) -> HandlerResult;
}
