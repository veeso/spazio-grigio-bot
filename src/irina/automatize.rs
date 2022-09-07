//! # Automatize
//!
//! A module to automatize messages

use super::newsletter::Newsletter;
use super::redis::RedisRepository;
use super::repository::Repository;
use super::rsshub::RssHubClient;
use super::youtube::Youtube;
use super::AnswerBuilder;

use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type AutomatizerResult<T> = Result<T, AutomatizerError>;

/// Automatizer error
#[derive(Debug, Error)]
pub enum AutomatizerError {
    #[error("scheduler error: {0}")]
    Scheduler(JobSchedulerError),
}

impl From<JobSchedulerError> for AutomatizerError {
    fn from(e: JobSchedulerError) -> Self {
        Self::Scheduler(e)
    }
}

/// Automatizer takes care of sending messages to subscribed users
pub struct Automatizer {
    scheduler: JobScheduler,
}

impl Automatizer {
    /// Start automatizer
    pub async fn start() -> AutomatizerResult<Self> {
        debug!("starting automatizer");
        Ok(Self {
            scheduler: Self::setup_cron_scheduler().await?,
        })
    }

    /// Subscribe a chat to the automatizer
    pub async fn subscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.insert_chat(*chat).await?;
        info!("subscribed {} to the automatizer", chat);
        Ok(())
    }

    /// Unsubscribe chat from automatizer. If the chat is not currently subscribed, return error
    pub async fn unsubscribe(&self, chat: &ChatId) -> anyhow::Result<()> {
        let repository = Repository::connect().await?;
        repository.delete_chat(*chat).await?;
        info!("unsubscribed {} from the automatizer", chat);
        Ok(())
    }

    /// Setup cron scheduler
    async fn setup_cron_scheduler() -> AutomatizerResult<JobScheduler> {
        let sched = JobScheduler::new().await?;
        // good_morning_job
        let good_morning_job = Job::new_async("0 5 6 * * *", |_, _| {
            Box::pin(async move {
                info!("running good_morning_job");
                if let Err(err) = Self::send_good_morning().await {
                    error!("good_morning_job failed: {}", err);
                }
            })
        })?;
        sched.add(good_morning_job).await?;
        // newsletter_job
        let newsletter_job = Job::new_async("0 30 19 * * *", |_, _| {
            Box::pin(async move {
                info!("running newsletter_job");
                if let Err(err) = Self::fetch_latest_newsletter().await {
                    error!("newsletter_job failed: {}", err);
                }
            })
        })?;
        sched.add(newsletter_job).await?;
        // newsletter_job
        let instagram_job = Job::new_async("0 40 * * * *", |_, _| {
            Box::pin(async move {
                info!("running instagram_job");
                if let Err(err) = Self::fetch_latest_unseen_instagram_post().await {
                    error!("instagram_job failed: {}", err);
                }
            })
        })?;
        sched.add(instagram_job).await?;
        // new video check
        let new_video_check_job = Job::new_async("0 30 * * * *", |_, _| {
            Box::pin(async move {
                info!("running new_video_check_job");
                if let Err(err) = Self::fetch_latest_video().await {
                    error!("new_video_check_job failed: {}", err);
                }
            })
        })?;
        sched.add(new_video_check_job).await?;

        sched
            .start()
            .await
            .map(|_| sched)
            .map_err(AutomatizerError::from)
    }

    async fn send_good_morning() -> anyhow::Result<()> {
        let bot = Bot::from_env().auto_send();
        let message = super::Irina::good_morning();
        for chat in Self::subscribed_chats().await?.iter() {
            debug!("sending scheduled good morning to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send scheduled good morning to {}: {}", chat, err);
            }
        }
        Ok(())
    }

    /// Send perla
    async fn fetch_latest_newsletter() -> anyhow::Result<()> {
        let message = match Newsletter::connect()
            .await?
            .get_latest_message("info@spaziogrigio.com")
            .await
        {
            Ok(Some(v)) => v,
            Ok(None) => {
                info!("inbox is empty; return OK");
                return Ok(());
            }
            Err(err) => {
                anyhow::bail!("failed to check latest message: {}", err)
            }
        };
        let mut redis_client = RedisRepository::connect()?;
        let last_post_pubdate = redis_client.get_last_newsletter_update().await?;
        debug!(
            "last time I checked newsletter message, had date {:?}; latest has {}",
            last_post_pubdate, message.date
        );
        if last_post_pubdate.map(|x| x < message.date).unwrap_or(true) {
            let bot = Bot::from_env().auto_send();
            info!(
                "spazio grigio published a mail ({}): {}",
                message.date, message.subject
            );
            let answer = AnswerBuilder::default()
                .text(format!(
                    "Ciao sono Irina.\n{}\n\n{}",
                    message.subject, message.body,
                ))
                .finalize();
            for chat in Self::subscribed_chats().await?.iter() {
                debug!("sending new newsletter notify to {}", chat);
                if let Err(err) = answer.clone().send(&bot, *chat).await {
                    error!("failed to send scheduled newsletter to {}: {}", chat, err);
                }
            }
            redis_client
                .set_last_newsletter_update(message.date)
                .await?;
        }
        Ok(())
    }

    /// Fetch latest video job
    async fn fetch_latest_video() -> anyhow::Result<()> {
        let mut redis_client = RedisRepository::connect()?;
        let last_post_pubdate = redis_client
            .get_last_video_pubdate()
            .await?
            .unwrap_or_default();
        let video = match Youtube::get_oldest_unseen_video(last_post_pubdate).await {
            Ok(Some(v)) => v,
            Ok(None) => {
                debug!("could not find any unseen video from spazio grigio");
                return Ok(());
            }
            Err(err) => {
                anyhow::bail!("failed to check latest video: {}", err)
            }
        };

        debug!(
                "last time I checked spazio-grigio videos, spazio-grigio video had date {:?}; latest has {:?}",
                last_post_pubdate,
                video.date
            );
        let bot = Bot::from_env().auto_send();
        info!(
            "spazio grigio published a new video ({:?}): {}",
            video.date,
            video.title.as_deref().unwrap_or_default()
        );
        let message = AnswerBuilder::default()
            .text(format!(
                "Ciao sono Irina. Ho appena pubblicato questo nuovo mio video: {}\n👉 {}",
                video.title.as_deref().unwrap_or_default(),
                video.url
            ))
            .finalize();
        for chat in Self::subscribed_chats().await?.iter() {
            debug!("sending new video notify to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send scheduled video notify to {}: {}", chat, err);
            }
        }
        if let Some(date) = video.date {
            redis_client.set_last_video_pubdate(date).await?;
        }

        Ok(())
    }

    /// Fetch latest video job
    async fn fetch_latest_unseen_instagram_post() -> anyhow::Result<()> {
        let mut redis_client = RedisRepository::connect()?;
        let last_post_pubdate = redis_client
            .get_last_instagram_update()
            .await?
            .unwrap_or_default();
        let post = match RssHubClient::get_oldest_unseen_post(last_post_pubdate).await {
            Ok(Some(v)) => v,
            Ok(None) => {
                debug!("no unseen posts from instagram could be found");
                return Ok(());
            }
            Err(err) => {
                anyhow::bail!("failed to check latest post: {}", err)
            }
        };
        debug!(
                "last time I checked spazio-grigio posts, spazio-grigio post had date {:?}; latest has {:?}",
                last_post_pubdate,
                post.date
            );
        let bot = Bot::from_env().auto_send();
        info!(
            "spazio grigio published a new ig post ({:?}): {}",
            post.date,
            post.title.as_deref().unwrap_or_default()
        );
        let message = AnswerBuilder::default()
                .text(format!(
                    "Ciao sono Irina. Ho appena pubblicato questo nuovo mio post su Instagram: {}\n{}\n👉 {}",
                    post.title.as_deref().unwrap_or_default(),
                    post.summary,
                    post.url
                ))
                .finalize();
        for chat in Self::subscribed_chats().await?.iter() {
            debug!("sending new post notify to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send scheduled post notify to {}: {}", chat, err);
            }
        }
        if let Some(date) = post.date {
            redis_client.set_last_instagram_update(date).await?;
        }

        Ok(())
    }

    pub async fn subscribed_chats() -> anyhow::Result<Vec<ChatId>> {
        let repository = Repository::connect().await?;
        repository.get_subscribed_chats().await
    }
}

impl Drop for Automatizer {
    fn drop(&mut self) {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            info!("Shutting scheduler down");
            if let Err(err) = self.scheduler.shutdown().await {
                error!("failed to stop scheduler: {}", err);
            }
        });
    }
}
