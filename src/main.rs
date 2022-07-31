#[macro_use]
extern crate lazy_static;

mod db;
mod graphql;
mod graphql_server;
mod setting;

mod api_server;
mod apis;

use std::thread::{self, JoinHandle};

use db::connect_to_db;
use mongodb::Database;
use once_cell::sync::OnceCell;
use setting::{get_setting, Settings};

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

    let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();
    threads.push(spawn_graphql_server());
    threads.push(spawn_api_server());
    while let Some(cur_thread) = threads.pop() {
        cur_thread.join().unwrap();
    }
}

fn spawn_graphql_server() -> JoinHandle<()> {
    thread::spawn(|| {
        match crate::graphql_server::connect() {
            Ok(_) => {
                log::info!("Server stopped");
            }
            Err(err) => {
                log::error!("Could not start the graphql_server => {:?}", err);
            }
        };
    })
}

fn spawn_api_server() -> JoinHandle<()> {
    thread::spawn(|| {
        log::info!("Starting - API Server");
        match crate::api_server::connect() {
            Ok(_) => {
                log::info!("API Server Stopped");
            }
            Err(err) => {
                log::error!("Could not start the API server => {:?}", err);
            }
        }
    })
}
