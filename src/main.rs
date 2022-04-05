#[macro_use]
extern crate lazy_static;

mod db;
mod setting;
mod graphql_models;
mod graphql_schema;
mod graphql_server;
mod graphql_utils;

use setting::{Settings, get_setting};

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Settings = get_setting().expect("Could not load setting");
}

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    match crate::graphql_server::connect() {
        Ok(_) => {
            log::info!("Server stopped");
        }
        Err(err) => {
            log::error!("Could not start the graphql_server => {:?}", err);
        },
    };
}
