mod actions;
mod application;
mod components;
mod components_container;
mod controller;
mod db;
mod dependencies;
mod models;
mod views;

use crate::application::Application;
use once_cell::sync::Lazy;

static APPLICATION: Lazy<Application> = Lazy::new(|| Application::new());

fn main() -> Result<(), i32> {
    APPLICATION.start();

    Ok(())
}
