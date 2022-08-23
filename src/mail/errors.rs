use async_imap::error::Error as ImapError;
use thiserror::Error;

pub type EmailResult<T> = Result<T, EmailError>;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("imap client error: {0}")]
    Imap(ImapError),
    #[error("login failed: {0}")]
    LoginFailed(ImapError),
    #[error("failed to parse email body: {0}")]
    ParseError(String),
}

impl From<ImapError> for EmailError {
    fn from(e: ImapError) -> Self {
        Self::Imap(e)
    }
}
