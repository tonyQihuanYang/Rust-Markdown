use mongodb::{options::ClientOptions, Client, Database};

use crate::{CONFIG, setting::MongoDBSetting};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    pub database: Database,
}
impl DB {
    pub async fn connect() -> DB {
        let MongoDBSetting {url, database_name} = &CONFIG.database;
        let mut client_options = ClientOptions::parse(url.to_owned())
            .await
            .expect("client option is missing");
        client_options.app_name = Some("booky".to_string());
        let client = Client::with_options(client_options).expect("Client could not create");
        let database = client.database(database_name);
        DB { client, database }
    }
}

pub async fn connect_to_db() -> Database {
    log::info!("Trying to connect to database...");
    let MongoDBSetting {url, database_name} = &CONFIG.database;
    let mut client_options = ClientOptions::parse(url.to_owned())
        .await
        .expect("client option is missing");
    client_options.app_name = Some("booky".to_string());
    let client = Client::with_options(client_options).expect("Client could not create");
    let database = client.database(database_name); 
    log::info!("Connected successfully to to database to url {}, database name {}", url, database_name);
    return database;
}