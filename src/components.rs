use crate::{components_container::ComponentsContainer, commands::command_dispatcher::CommandDispatcher, commands::command::Command};

pub fn add_components(components_container: &mut ComponentsContainer) {
    let _ = components_container.get_component_as<CommandDispatcher>(CommandDispatcher::component_name(&self));
}
