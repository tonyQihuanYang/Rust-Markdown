use config::{Config, ConfigError, File};
use serde::Deserialize;

const DEVELOPMENT: &str= "Development";
const PRODUCTION: &str = "Production";
const DOCKER_PRODUCTION: &str = "DockerPro";

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSetting {
    pub port: u16,
    pub url: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct MongoDBSetting {
    pub url: String,
    pub database_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSetting,
    pub database: MongoDBSetting,
}

pub fn get_setting() -> Result<Settings, ConfigError> {
    let cur_env = get_current_running_env();
    let cur_config_path = get_config_path(&cur_env); 
    println!("Currently running in enviroment {}", cur_env);

    let settings = Config::builder()
        .add_source(File::with_name(&cur_config_path))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    println!(
        "{:?}",
        settings.clone()
            .try_deserialize::<Settings>()
            .unwrap()
    );

    return settings
        .try_deserialize::<Settings>();
}


fn get_current_running_env() -> String {
    std::env::var("RUN_ENV").unwrap_or_else(|_| DEVELOPMENT.into())
}

fn get_config_path(cur_env: &str) -> String {
    let path = match cur_env {
        DEVELOPMENT => "./configs/development",
        PRODUCTION=> "./configs/production",
        DOCKER_PRODUCTION => "/usr/src/configs/docker_production",
        _ => "/configs/development",
    };
    println!("The current directory is {}", path);
    return path.to_string();
}
