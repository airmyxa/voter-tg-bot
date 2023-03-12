mod configuration;
mod db;
mod models;
mod utils;
mod views;

use crate::db::sqlite::database::{SQLiteConnectionType, SQLiteDb, SQLiteSettings};
use crate::views::handler::Dependencies;
use std::ops::Deref;
use std::sync::Arc;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting voter bot");

    let bot = Bot::from_env();

    let dependencies = Dependencies::new(Arc::new(SQLiteDb::connect(SQLiteSettings::new(
        SQLiteConnectionType::File(String::from("voter.db")),
    ))));

    dependencies
        .db
        .lock()
        .unwrap()
        .deref()
        .requester()
        .init_db();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(views::message::handle))
        .branch(Update::filter_callback_query().endpoint(views::callback::view::handle));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
