//! # Youtube
//!
//! This module exposes the function to fetch the youtube latest videos from spazio grigio

use crate::feed::Entry;
use crate::youtube::{Feed, YoutubeClient};

// <https://www.youtube.com/feeds/videos.xml?channel_id=UCK3cMi97Kf_WENLvRFdztoQ>
const CHANNEL_ID: &str = "UCK3cMi97Kf_WENLvRFdztoQ";

pub struct Youtube;

impl Youtube {
    /// Get latest video from spazio grigio
    pub async fn get_latest_video() -> anyhow::Result<Entry> {
        if let Some(video) = Self::get_latest_videos().await?.entries().next() {
            Ok(video.clone())
        } else {
            anyhow::bail!("Ciao sono Irina. Non ho trovato nessun video per te")
        }
    }

    /// Get latest videos from spazio grigio
    pub async fn get_latest_videos() -> anyhow::Result<Feed> {
        let client = YoutubeClient::new(CHANNEL_ID);
        client.fetch().await.map_err(|e| {
            anyhow::anyhow!(
                "Ciao sono Irina e non riesco a darti i miei ultimi video: {}",
                e
            )
        })
    }
}
