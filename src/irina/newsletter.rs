//! # Spazio grigio newsletter

use super::config::Config;
use crate::mail::{EmailClient, Message};

pub struct Newsletter {
    client: EmailClient,
}

impl Newsletter {
    /// Connect to the database
    pub async fn connect() -> anyhow::Result<Self> {
        let config =
            Config::try_from_env().map_err(|_| anyhow::anyhow!("failed to load configuration"))?;
        Ok(Self {
            client: EmailClient::connect(
                &config.imap_server,
                config.imap_port,
                &config.email_address,
                &config.email_password,
            )
            .await
            .map_err(|e| anyhow::anyhow!("could not connect to email server: {}", e))?,
        })
    }

    /// Get latest message
    pub async fn get_latest_message(&mut self, from: &str) -> anyhow::Result<Option<Message>> {
        Ok(self
            .client
            .get_messages()
            .await
            .map_err(|e| anyhow::anyhow!("failed to get inbox messages: {}", e))?
            .into_iter()
            .filter(|x| x.sender_address.as_str() == from)
            .last())
    }
}
