use std::fmt::Debug;
use log::info;

pub trait HandlerTr<R: Debug, D> {
    fn handle(self, request: R, dependencies: D);
}

pub struct Dependencies {}