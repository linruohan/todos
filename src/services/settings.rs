use config::Config;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
}
pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config")) // 读取 config.toml
        .build()?;

    let app_config: AppConfig = settings.try_deserialize()?;
    Ok(app_config)
}
