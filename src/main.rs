mod configuration;
mod db;
mod models;
mod utils;
mod views;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting voter bot");

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(views::message::handle))
        .branch(Update::filter_callback_query().endpoint(views::callback::handle));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
