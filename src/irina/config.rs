//! # Config
//!
//! spazio-grigio-bot configuration

#[derive(Debug, Deserialize, Serialize)]
/// Application config
pub struct Config {
    pub database_url: String,
    pub email_address: String,
    pub email_password: String,
    pub imap_server: String,
    pub imap_port: u16,
    pub redis_url: String,
    pub rsshub_url: String,
    pub teloxide_token: String,
}

impl Config {
    /// Try to create config from env
    pub fn try_from_env() -> anyhow::Result<Self> {
        envy::from_env()
            .map_err(|e| anyhow::anyhow!("could not load config from environment: {}", e))
    }
}
