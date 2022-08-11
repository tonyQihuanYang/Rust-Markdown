use mongodb::{options::ClientOptions, Client, Database};

use crate::{setting::MongoDBSetting, CONFIG};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub database: Database,
}

pub async fn connect_to_db() -> Database {
    log::info!("Trying to connect to database...");
    let MongoDBSetting { url, database_name } = &CONFIG.database;
    let mut client_options = ClientOptions::parse(url.to_owned())
        .await
        .expect("client option is missing");
    client_options.app_name = Some("booky".to_string());
    let client = Client::with_options(client_options).expect("Client could not create");
    let database = client.database(database_name);
    log::info!(
        "Connected successfully to to database to url {}, database name {}",
        url,
        database_name
    );
    return database;
}

