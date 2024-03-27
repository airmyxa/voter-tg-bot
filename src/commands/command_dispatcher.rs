use std::collections::HashMap;

use log::info;

use crate::components_container::Component;

use super::{
    command::{Command, CommandFactory},
    command_context::{CommandDependencies, CommandParams},
};

pub struct CommandDispatcher {
    command_container: HashMap<&'static str, Box<dyn Command>>,
}

impl Component for CommandDispatcher {
    fn create_component(components: &mut crate::components_container::ComponentsContainer) -> Self {
        Self {
            command_container: HashMap::default(),
        }
    }

    fn component_name(&self) -> &'static str {
        return "command-dispatcher";
    }
}

impl CommandDispatcher {
    pub fn add_command(&mut self, command_factory: &dyn CommandFactory) {
        let command = command_factory.create_command();
        let command_str = command.command_str();
        if self
            .command_container
            .insert(command_str, command)
            .is_none()
        {
            panic!(
                "Should not add same command more than once: {}",
                command_str
            );
        };
    }

    pub fn dispatch(&self, command_str: &str, params: CommandParams, deps: CommandDependencies) {
        let command = self.command_container.get(command_str);
        if command.is_none() {
            info!("Cannot find command handler for command '{}'", command_str);
            return;
        }
        let command = command.unwrap().handle(params, deps);
    }
}
