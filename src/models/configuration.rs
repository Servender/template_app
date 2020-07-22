use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub db: DatabaseSettings,
    pub server: ServerSettings,
    pub redis: RedisSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub pass: String,
    pub db_name: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u32,
}
