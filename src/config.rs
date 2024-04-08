use dotenv::dotenv;
use serde::Deserialize;

/// Environment configuration.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// Development environment flag
    pub is_dev: bool,
    /// App host
    pub host: String,
    /// App port
    pub port: String,
    /// Public URL of the CDN server
    pub cdn_server_url: String,
    /// Private URL of the RPC server
    pub rpc_server_url: String,
    /// Private RPC authentication token
    pub rpc_secret_token: String,
    /// Redis host
    pub redis_host: String,
    /// Redis port
    pub redis_port: String,
    /// The sentry DSN value
    pub sentry_dsn: String,
}

/// Returns the application environment configuration.
pub fn get_app_config() -> envy::Result<Config> {
    dotenv().ok();
    envy::from_env()
}
