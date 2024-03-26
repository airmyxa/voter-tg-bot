use super::command_context::{CommandDependencies, CommandParams};

pub trait Command: Sync + Send {
    fn command_str(&self) -> &'static str;

    fn handle(&self, params: CommandParams, deps: CommandDependencies) -> anyhow::Result<()>;
}

pub trait CommandFactory {
    fn create_command(&self) -> Box<dyn Command>;
}
