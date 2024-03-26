use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

pub trait Component: Send + Sync {
    fn create_component(components: &mut ComponentsContainer) -> Self
    where
        Self: Sized;

    fn component_name(&self) -> &'static str;
}

pub struct ComponentsContainer {
    components_map: HashMap<&'static str, Arc<dyn Any + Send + Sync>>,
}

impl ComponentsContainer {
    pub fn new() -> Self {
        ComponentsContainer {
            components_map: HashMap::new(),
        }
    }

    pub fn add_component<T>(&mut self, name: &'static str)
    where
        T: Component + 'static,
    {
        self.get_component_as::<T>(name);
    }

    pub fn get_component_as<T>(&mut self, name: &'static str) -> Arc<T>
    where
        T: Component + 'static,
    {
        if self.components_map.contains_key(&name) {
            return self.get_existing_component::<T>(&name).clone();
        }

        let required_component = Arc::new(T::create_component(self));
        let component_name = T::component_name(&required_component);
        self.components_map
            .insert(component_name, required_component.clone());

        return self.get_existing_component::<T>(&name).clone();
    }

    fn get_existing_component<T>(&mut self, name: &'static str) -> Arc<T>
    where
        T: Component + 'static,
    {
        return self
            .components_map
            .get(&name)
            .unwrap()
            .clone()
            .downcast::<T>()
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::components_container::{Component, ComponentsContainer};

    struct DbComponent {}

    impl Component for DbComponent {
        fn create_component(components: &mut ComponentsContainer) -> Self {
            return DbComponent {};
        }

        fn component_name(&self) -> &'static str {
            return "db-component";
        }
    }

    struct RequesterComponent {
        db: Arc<DbComponent>,
    }

    impl Component for RequesterComponent {
        fn create_component(components: &mut ComponentsContainer) -> Self {
            let db = components.get_component_as::<DbComponent>("db-component");
            let requester = RequesterComponent { db };
            return requester;
        }

        fn component_name(&self) -> &'static str {
            return "db-component";
        }
    }

    #[test]
    fn test_component_system() {
        let mut container = ComponentsContainer::new();

        let db_component = DbComponent::create_component(&mut container);
        container.add_component(db_component);

        let requester_component = RequesterComponent::create_component(&mut container);
        container.add_component(requester_component);
    }

    #[test]
    fn test_component_system_reverse_component_order() {
        let mut container = ComponentsContainer::new();

        let requester_component = RequesterComponent::create_component(&mut container);
        container.add_component(requester_component);

        let db_compoenent = DbComponent::create_component(&mut container);
        container.add_component(db_compoenent);
    }
}
