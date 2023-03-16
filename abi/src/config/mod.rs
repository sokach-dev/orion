use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    // rpc server port
    pub port: u16,
    // rpc server host
    pub host: String,

    pub db_config: DbConfig,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub database_url: String,
}
