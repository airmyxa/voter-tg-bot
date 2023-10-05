mod application;
mod components_container;
mod controllers;
mod db;
mod dependencies;
mod models;
mod services;
mod storage;
mod views;

use crate::application::Application;
use once_cell::sync::Lazy;

static APPLICATION: Lazy<Application> = Lazy::new(|| Application::new());

fn main() -> Result<(), i32> {
    APPLICATION.start();

    Ok(())
}
