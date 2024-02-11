use std::any::Any;
use std::collections::HashMap;
use std::ops::Deref;

pub trait Component: Send + Sync + 'static {
    fn create_component(components: &mut ComponentsContainer) -> Self
    where
        Self: Sized;

    fn component_name(&self) -> &'static str;
}

pub struct ComponentsContainer {
    components_map: HashMap<&'static str, Box<dyn Component>>,
}

impl ComponentsContainer {
    pub fn new() -> Self {
        ComponentsContainer {
            components_map: HashMap::new(),
        }
    }

    pub fn add_component<T>(&mut self, name: &str)
    where
        T: Component,
    {
        let _ = self.get_component_as(name);
    }

    pub fn get_component_as<T>(&mut self, name: &str) -> &'static T
    where
        T: Component,
    {
        if self.components_map.contains_key(&name) {
            return self.get_existing_component::<T>(&name);
        }

        let required_component = Box::new(T::create_component(self));
        self.components_map
            .insert(T::component_name(&self), component);

        return self.get_existing_component::<T>(&name);
    }

    fn get_existing_component<T>(&mut self, name: &str) -> &'static T
    where
        T: Component,
    {
        return self
            .components_map
            .get(&name)
            .unwrap()
            .downcast::<T>()
            .unwrap()
            .deref();
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
        db: DbComponent,
    }

    impl RequesterComponent {
        pub fn new(db: &'static DbComponent) -> Self {
            RequesterComponent { db }
        }
    }

    impl Component for RequesterComponent {
        fn create_component(components: &mut ComponentsContainer) -> Self {
            let db = components.get_component_as::<DbComponent>("db-component");
            let requester = RequesterComponent::new(db);
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
