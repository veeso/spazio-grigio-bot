//! # Youtube
//!
//! This module exposes the function to fetch the youtube latest videos from spazio grigio

use crate::feed::Entry;
use crate::youtube::{Feed, YoutubeClient};

use chrono::{DateTime, Utc};

// <https://www.youtube.com/feeds/videos.xml?channel_id=UCK3cMi97Kf_WENLvRFdztoQ>
const CHANNEL_ID: &str = "UCK3cMi97Kf_WENLvRFdztoQ";

pub struct Youtube;

impl Youtube {
    /// Get latest video from spazio grigio
    pub async fn get_latest_video() -> anyhow::Result<Entry> {
        if let Some(video) = Self::get_latest_videos().await?.entries().next() {
            Ok(video.clone())
        } else {
            anyhow::bail!("Ciao sono Irina. Non ho nessun video da mostrarti.")
        }
    }

    /// Get oldest unseen video from youtube
    pub async fn get_oldest_unseen_video(
        last_video_pubdate: DateTime<Utc>,
    ) -> anyhow::Result<Option<Entry>> {
        let feed = Self::get_latest_videos().await?;
        // sort by date
        let mut entries: Vec<Entry> = feed.entries().cloned().collect();
        entries.sort_by_key(|x| x.date);
        for entry in entries.into_iter() {
            if entry.date > Some(last_video_pubdate) {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }

    /// Get latest videos from spazio grigio
    pub async fn get_latest_videos() -> anyhow::Result<Feed> {
        let client = YoutubeClient::new(CHANNEL_ID);
        client.fetch().await.map_err(|e| {
            anyhow::anyhow!(
                "Ciao sono Irina. Non riesco ad ottenere gli ultimi video di Spazio Grigio: {}",
                e
            )
        })
    }
}
