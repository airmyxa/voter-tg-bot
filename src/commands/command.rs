use super::command_context::{CommandDependencies, CommandParams};

pub trait Command: Sync + Send {
    fn command_str() -> &'static str
    where
        Self: Sized;

    fn handle(params: CommandParams, deps: CommandDependencies) -> anyhow::Result<()>
    where
        Self: Sized;
}

pub trait CommandFactory {
    fn create_command() -> Box<dyn Command>;
}
