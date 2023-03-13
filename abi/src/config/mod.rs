use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db: DbConfig,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub database_url: String,
}
