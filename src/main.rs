#[macro_use]
extern crate lazy_static;

mod db;
mod setting;
mod graphql_models;
mod graphql_schema;
mod graphql_server;
mod graphql_utils;

use std::thread;

use db::{connect_to_db};
use setting::{Settings, get_setting};
use once_cell::sync::OnceCell;
use mongodb::{Database};

lazy_static! {
    #[derive(Debug)]
    static ref CONFIG: Settings = get_setting().expect("Could not load setting");
}

static MONGO_DB: OnceCell<Database> = OnceCell::new();

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mongo_db = connect_to_db().await;
    MONGO_DB.set(mongo_db).unwrap(); 
    
    thread::spawn(|| {
        match crate::graphql_server::connect() {
            Ok(_) => {
                log::info!("Server stopped");
            }
            Err(err) => {
                log::error!("Could not start the graphql_server => {:?}", err);
            },
        };
    }).join().expect("Server thread failed");
}