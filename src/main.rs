mod components;
mod configuration;
mod controller;
mod db;
mod dependencies;
mod models;
mod utils;
mod views;
mod actions;

use crate::components::create_components;
use log::error;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Me;

#[tokio::main]
async fn main() -> Result<(), i32> {
    pretty_env_logger::init();
    log::info!("Starting voter bot");

    let bot = Bot::from_env();

    let components = create_components();
    let components = match components {
        Err(err) => {
            error!("Cannot create components: {}", err);
            return Err(-1);
        }
        Ok(components) => components,
    };

    let components = Arc::new(components);
    let components_message = Arc::clone(&components);
    let components_callback_query = Arc::clone(&components_message);

    // components.dependencies().requester.init_db()?;

    let handler = dptree::entry()
        .branch(
            Update::filter_message().endpoint(move |bot: Bot, message: Message, me: Me| {
                return views::message::handle(bot, message, me, components_message.dependencies());
            }),
        )
        .branch(
            Update::filter_callback_query().endpoint(move |bot: Bot, query: CallbackQuery| {
                return views::callback::view::handle(
                    bot,
                    query,
                    components_callback_query.dependencies(),
                );
            }),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
