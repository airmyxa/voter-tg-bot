mod components;
mod configuration;
mod db;
mod dependencies;
mod models;
mod utils;
mod views;

use crate::components::create_components;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Me;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting voter bot");

    let bot = Bot::from_env();

    let components = Arc::new(create_components());
    let components2 = Arc::clone(&components);

    let handler = dptree::entry()
        .branch(
            Update::filter_message().endpoint(move |bot: Bot, message: Message, me: Me| {
                return views::message::handle(bot, message, me, components.dependencies());
            }),
        )
        .branch(
            Update::filter_callback_query().endpoint(move |bot: Bot, query: CallbackQuery| {
                return views::callback::view::handle(bot, query, components2.dependencies());
            }),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
