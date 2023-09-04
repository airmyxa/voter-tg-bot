use crate::components_container::ComponentsContainer;
use crate::dependencies::Dependencies;
use crate::views;
use std::ops::Deref;
use std::sync::Arc;
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::types::{CallbackQuery, Me, Message, Update};
use teloxide::{dptree, Bot};

pub struct Application {
    components: ComponentsContainer,
    dependencies: Arc<Dependencies>,
}

impl Application {
    pub fn new() -> Self {
        let mut components = ComponentsContainer::new();
        let dependencies = components.get_component_as::<Dependencies>("dependencies");
        Application {
            components,
            dependencies,
        }
    }

    pub fn start(&'static self) {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(self.dispatch());
    }

    async fn dispatch(&'static self) {
        pretty_env_logger::init();
        log::info!("Starting voter bot");

        let bot = Bot::from_env();

        let handler = dptree::entry()
            .branch(
                Update::filter_message().endpoint(move |bot: Bot, message: Message, me: Me| {
                    return views::message::handle(
                        bot,
                        message,
                        me,
                        self.dependencies.deref().clone(),
                    );
                }),
            )
            .branch(Update::filter_callback_query().endpoint(
                move |bot: Bot, query: CallbackQuery| {
                    return views::callback::view::handle(
                        bot,
                        query,
                        self.dependencies.deref().clone(),
                    );
                },
            ));

        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}
