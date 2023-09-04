use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

pub trait ComponentTr: Send + Sync + 'static {
    fn create_component(components: &mut ComponentsContainer) -> Arc<Self>;

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

    pub fn add_component<T>(&mut self, component: Arc<T>)
    where
        T: ComponentTr,
    {
        self.components_map
            .insert(component.component_name(), component);
    }

    pub fn get_component_as<T>(&mut self, name: &str) -> Arc<T>
    where
        T: ComponentTr,
    {
        let required_component = T::create_component(self);
        self.add_component::<T>(required_component);

        let result = self
            .components_map
            .get(name)
            .unwrap()
            .clone()
            .downcast::<T>()
            .unwrap();

        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::components_container::{ComponentTr, ComponentsContainer};
    use std::sync::Arc;

    struct DbComponent {}

    impl ComponentTr for DbComponent {
        fn create_component(components: &mut ComponentsContainer) -> Arc<Self> {
            return Arc::new(DbComponent {});
        }

        fn component_name(&self) -> &'static str {
            return "db-component";
        }
    }

    struct RequesterComponent {
        db: Arc<DbComponent>,
    }

    impl RequesterComponent {
        pub fn new(db: Arc<DbComponent>) -> Self {
            RequesterComponent { db: db }
        }
    }

    impl ComponentTr for RequesterComponent {
        fn create_component(components: &mut ComponentsContainer) -> Arc<Self> {
            let db = components.get_component_as::<DbComponent>("db-component");
            let requester = RequesterComponent::new(db);
            return Arc::new(requester);
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
