use anyhow::Result;
use std::env;
#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub server_address: String,
    pub base_url: String,
    pub surreal_username: String,
    pub surreal_password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env if it exists
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "wss://cloakystores-wss://cloakystores-06a9f7u3jlrsf43q77o8ttu1kk.aws-euw1.surreal.cloud".to_string());

        let server_address =
            env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        let surreal_password = env::var("SURREAL_USERNAME").unwrap_or_else(|_| "".to_string());
        let surreal_username = env::var("SURREAL_PASSWORD").unwrap_or_else(|_| "".to_string());

        Ok(Config {
            database_url,
            server_address,
            base_url,
            surreal_username,
            surreal_password,
        })
    }
}
