//! # Automatize
//!
//! A module to automatize messages

use super::newsletter::Newsletter;
use super::repository::Repository;
use super::youtube::Youtube;
use super::AnswerBuilder;

use chrono::{DateTime, Local, Utc};
use futures::lock::Mutex;
use teloxide::prelude::*;
use teloxide::types::ChatId;
use thiserror::Error;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

type AutomatizerResult<T> = Result<T, AutomatizerError>;

lazy_static! {
    static ref LAST_VIDEO_PUBLISHED_DATE: Mutex<DateTime<Local>> = Mutex::new(DateTime::default());
    static ref LAST_NEWSLETTER_EMAIL_DATE: Mutex<DateTime<Utc>> = Mutex::new(DateTime::default());
}

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
        if let Err(err) = Self::notify_started().await {
            error!("failed to send start notify: {}", err);
        }
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
        let newsletter_job = Job::new_async("0 30 12 * * *", |_, _| {
            Box::pin(async move {
                info!("running newsletter_job");
                if let Err(err) = Self::fetch_latest_newsletter().await {
                    error!("newsletter_job failed: {}", err);
                }
            })
        })?;
        sched.add(newsletter_job).await?;
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
        debug!(
            "last time I checked newsletter message, had date {}; latest has {}",
            *LAST_NEWSLETTER_EMAIL_DATE.lock().await,
            message.date
        );
        if *LAST_NEWSLETTER_EMAIL_DATE.lock().await < message.date {
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
            *LAST_NEWSLETTER_EMAIL_DATE.lock().await = message.date;
        }
        Ok(())
    }

    /// Fetch latest video job
    async fn fetch_latest_video() -> anyhow::Result<()> {
        let video = match Youtube::get_latest_video().await {
            Ok(v) => v,
            Err(err) => {
                anyhow::bail!("failed to check latest video: {}", err)
            }
        };
        if let Some(date) = video.date {
            debug!(
                "last time I checked big-luca videos, big-luca video had date {}; latest has {}",
                *LAST_VIDEO_PUBLISHED_DATE.lock().await,
                date
            );
            if *LAST_VIDEO_PUBLISHED_DATE.lock().await < date {
                let bot = Bot::from_env().auto_send();
                info!(
                    "spazio grigio published a new video ({}): {}",
                    date,
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
                *LAST_VIDEO_PUBLISHED_DATE.lock().await = date;
            }
        }
        Ok(())
    }

    pub async fn notify_started() -> anyhow::Result<()> {
        let bot = Bot::from_env().auto_send();
        let message = AnswerBuilder::default()
            .text("Ciao sono Irina. Sono tornata dallo yoga.")
            .finalize();
        for chat in Self::subscribed_chats().await?.iter() {
            debug!("sending new video notify to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to send start notify to {}: {}", chat, err);
            }
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
