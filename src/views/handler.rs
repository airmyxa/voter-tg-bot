use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;

pub type GenericError = Box<dyn Error + Send + Sync>;
pub type MaybeError = Result<(), GenericError>;

#[async_trait]
pub trait HandlerTr<R, D>
where
    R: Debug,
{
    async fn handle(self, request: R, dependencies: D) -> MaybeError;
}
