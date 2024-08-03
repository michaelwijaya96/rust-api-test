use std::env;
use std::path::Path;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        println!("Current directory: {:?}", current_dir);

        // Check if the config file exists
        let config_path = "./config/default.toml";
        if Path::new(config_path).exists() {
            println!("Config file exists!");
        } else {
            println!("Config file does not exist!");
        }
        let current_dir = env::current_dir().expect("Failed to get current directory");
        println!("Current working directory: {:?}", current_dir);

        let mut config = Config::new();

        // Start with the default configuration file
        config.merge(File::with_name("config/default.toml"))?;

        // Add environment-specific configuration file
        let environment: String = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        config.merge(File::with_name(&format!("config/{}", environment)).required(false))?;

        // Add in settings from the environment (with a prefix of APP and '__' as separator)
        config.merge(Environment::with_prefix("APP").separator("__"))?;

        config.try_into()
    }
}
