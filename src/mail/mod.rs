//! # Mail client
//!
//! this module exposes the email client

use async_native_tls::TlsStream;
use async_std::net::TcpStream;
use futures::TryStreamExt;

mod errors;
mod message;

pub use errors::{EmailError, EmailResult};
pub use message::Message;

pub struct EmailClient {
    session: async_imap::Session<TlsStream<TcpStream>>,
}

impl EmailClient {
    /// Connect to email client
    pub async fn connect(
        server: &str,
        port: u16,
        username: &str,
        password: &str,
    ) -> EmailResult<Self> {
        let tls = async_native_tls::TlsConnector::new();
        let client = async_imap::connect((server, port), server, tls)
            .await
            .map_err(EmailError::from)?;
        let session = client
            .login(username, password)
            .await
            .map_err(|(e, _)| EmailError::LoginFailed(e))?;
        Ok(Self { session })
    }

    /// Get all email messages
    pub async fn get_messages(&mut self) -> EmailResult<Vec<Message>> {
        self.session.select("INBOX").await?;
        debug!("INBOX selected");
        let mut messages: Vec<Message> = Vec::new();
        let mut seq = 1;
        loop {
            debug!("collecting message for SEQ {}", seq);
            let messages_stream = self.session.fetch(seq.to_string(), "RFC822").await?;
            let seq_messages: Vec<_> = messages_stream.try_collect().await?;
            if seq_messages.is_empty() {
                debug!("SEQ {} is empty; stop looping", seq);
                break;
            }
            for message in seq_messages {
                if let Some(body) = message.body() {
                    match Message::try_from(body) {
                        Ok(msg) => messages.push(msg),
                        Err(err) => {
                            error!("failed to parse message SEQ {}: {}", seq, err);
                        }
                    }
                }
            }
            seq += 1;
        }
        Ok(messages)
    }
}
