//! # Errors
//!
//! Youtube client errors

use feed_rs::parser::ParseFeedError;
use thiserror::Error;

pub type YoutubeResult<T> = Result<T, YoutubeError>;

/// An error returned by the Youtube Client
#[derive(Debug, Error)]
pub enum YoutubeError {
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("Feed parser error: {0}")]
    FeedParserError(ParseFeedError),
}

impl From<reqwest::Error> for YoutubeError {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError(e)
    }
}

impl From<ParseFeedError> for YoutubeError {
    fn from(e: ParseFeedError) -> Self {
        Self::FeedParserError(e)
    }
}
