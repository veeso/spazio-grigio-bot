//! # Message
//!
//! A helper struct for email message

use chrono::prelude::*;
use chrono::{DateTime, Utc};
use mail_parser::{HeaderValue, Message as ParsedMessage};

use super::EmailError;

pub struct Message {
    pub sender_address: String,
    pub sender_name: Option<String>,
    pub date: DateTime<Utc>,
    pub body: String,
    pub subject: String,
}

impl TryFrom<&[u8]> for Message {
    type Error = EmailError;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        debug!("parsing message");
        let parsed = match ParsedMessage::parse(body) {
            Some(p) => p,
            None => {
                return Err(EmailError::ParseError(String::from(
                    "failed to parse message",
                )))
            }
        };
        debug!("message parsed");
        let (sender_address, sender_name) = match parsed.get_from() {
            HeaderValue::Address(addr) => {
                let address = match addr.address.as_deref() {
                    None => return Err(EmailError::ParseError(String::from("FROM is empty"))),
                    Some(a) => a.to_string(),
                };
                (address, addr.name.as_ref().map(|x| x.to_string()))
            }
            _ => return Err(EmailError::ParseError(String::from("FROM is empty"))),
        };
        debug!("mail sender is {} (name: {:?}", sender_address, sender_name);
        let subject = parsed.get_subject().unwrap_or_default().to_string();
        debug!("mail subject: {}", subject);
        let date = parsed
            .get_date()
            .map(|x| x.clone())
            .map(|x| {
                Utc.ymd_opt(x.year as i32, x.month as u32, x.day as u32)
                    .and_hms_opt(x.hour as u32, x.minute as u32, x.second as u32)
                    .unwrap()
            })
            .unwrap_or_else(|| Utc::now());
        let body = parsed
            .get_text_body(0)
            .map(|x| x.to_string())
            .unwrap_or_default();
        Ok(Self {
            sender_address,
            sender_name,
            date,
            subject,
            body,
        })
    }
}
