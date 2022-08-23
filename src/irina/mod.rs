//! # Irina
//!
//! This module implements the spazio grigio bot

mod answer;
mod automatize;
mod commands;
mod config;
mod morning_routine;
mod newsletter;
mod repository;
mod youtube;

use teloxide::{dispatching::update_listeners::webhooks, prelude::*, utils::command::BotCommands};
use url::Url;

use answer::{Answer, AnswerBuilder};
use automatize::Automatizer;
use commands::Command;
use morning_routine::MorningRoutine;
use once_cell::sync::OnceCell;

pub static AUTOMATIZER: OnceCell<Automatizer> = OnceCell::new();

/// Irina bot application
pub struct Irina {
    bot: AutoSend<Bot>,
}

impl Irina {
    /// Initialize irina
    pub async fn init() -> anyhow::Result<Self> {
        if std::env::var("TELOXIDE_TOKEN").is_err() {
            anyhow::bail!("TELOXIDE_TOKEN is NOT set. You must set this variable in the environment with your bot token API")
        }
        let automatizer = Automatizer::start()
            .await
            .map_err(|e| anyhow::anyhow!("failed to start automatizer: {}", e))?;
        if AUTOMATIZER.set(automatizer).is_err() {
            anyhow::bail!("failed to set automatizer");
        };
        let bot = Bot::from_env().auto_send();
        Ok(Self { bot })
    }

    /// Run irina
    pub async fn run(self) -> anyhow::Result<()> {
        // setup hooks
        let port = Self::get_heroku_port()?;
        if let Some(port) = port {
            Self::run_on_heroku(self, port).await
        } else {
            Self::run_simple(self).await
        }
    }

    /// run bot with heroku webhooks
    async fn run_on_heroku(self, port: u16) -> anyhow::Result<()> {
        info!("running bot with heroku listener (PORT: {})", port);
        let addr = ([0, 0, 0, 0], port).into();
        let token = self.bot.inner().token();
        let host = std::env::var("HOST").map_err(|_| anyhow::anyhow!("HOST is not SET"))?;
        let url = Url::parse(&format!("https://{host}/webhooks/{token}")).unwrap();
        debug!("configuring listener {}...", url);
        let listener = webhooks::axum(self.bot.clone(), webhooks::Options::new(addr, url))
            .await
            .map_err(|e| anyhow::anyhow!("could not configure listener: {}", e))?;
        // start bot
        teloxide::commands_repl_with_listener(self.bot, Self::answer, listener, Command::ty())
            .await;
        Self::notify_restart().await;
        Ok(())
    }

    /// run bot without webhooks
    async fn run_simple(self) -> anyhow::Result<()> {
        info!("running bot without webhooks");
        teloxide::commands_repl(self.bot, Self::answer, Command::ty()).await;
        Self::notify_restart().await;
        Ok(())
    }

    /// Answer handler for bot
    async fn answer(
        bot: AutoSend<Bot>,
        message: Message,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("got command {:?}", command);
        let answer = match command {
            Command::Start => Self::start(),
            Command::Help => Answer::simple_text(Command::descriptions()),
            Command::CiaoIrina => Self::subscribe_to_automatizer(&message.chat.id).await,
            Command::BuongiornoIrina => Self::good_morning(),
            Command::SiAlConsumismo => Self::unsubscribe_from_automatizer(&message.chat.id).await,
            Command::SerataSenzaTv => Self::get_latest_videos().await,
            Command::VideoMinimalista => Self::get_latest_video().await,
        };
        answer.send(&bot, message.chat.id).await
    }

    /// Get latest videos from papi
    async fn get_latest_videos() -> Answer {
        match youtube::Youtube::get_latest_videos().await {
            Ok(feed) => {
                let mut message =
                    String::from("Ciao sono Irina. Ecco cosa puoi guardare questa sera:\n\n");
                for video in feed.videos() {
                    message.push_str(
                        format!(
                            "• {} 👉 {}\n",
                            video.title.as_deref().unwrap_or_default(),
                            video.url
                        )
                        .as_str(),
                    );
                }
                Answer::simple_text(message)
            }
            Err(err) => Self::error(err),
        }
    }

    /// Get latest video from papi
    async fn get_latest_video() -> Answer {
        match youtube::Youtube::get_latest_video().await {
            Ok(video) => Answer::simple_text(format!(
                "Ciao sono Irina. Guarda il mio ultimo video \"{}\" 👉 {}",
                video.title.unwrap_or_default(),
                video.url
            )),
            Err(err) => Self::error(err),
        }
    }

    /// Subscribe chat to the automatizer
    async fn subscribe_to_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().subscribe(chat_id).await {
            Ok(_) => AnswerBuilder::default()
            .text("Ciao sono Irina e ti do il benvenuto in Spazio Grigio. Da ora riceverai tutti gli aggiornamenti per proseguire nel tuo percorso verso il Minimalismo.")
            .finalize(),
            Err(err) => Self::error(err),
        }
    }

    async fn unsubscribe_from_automatizer(chat_id: &ChatId) -> Answer {
        match AUTOMATIZER.get().unwrap().unsubscribe(chat_id).await {
            Ok(()) => AnswerBuilder::default()
                .text("Hai deciso di abbandonare il tuo percorso verso il Minimalismo. Mi dispiace tanto, se vuoi cambiare idea, ricomincia da qui /ciaoirina")
                .finalize(),
            Err(err) => Self::error(err),
        }
    }

    /// Send a reminder to all the subscribed chats, to notify the reboot
    async fn notify_restart() {
        info!("bot is shutting down; sending reboot");
        let bot = Bot::from_env().auto_send();
        let message = AnswerBuilder::default()
            .text("Ciao sono Irina. Ora devo fare yoga, ma quando avrò finito tornerò qui con importanti novità.")
            .finalize();
        let chats = match automatize::Automatizer::subscribed_chats().await {
            Ok(c) => c,
            Err(err) => {
                error!("failed to get chats: {}", err);
                return;
            }
        };
        for chat in chats.iter() {
            debug!("sending reboot to {}", chat);
            if let Err(err) = message.clone().send(&bot, *chat).await {
                error!("failed to reboot to {}: {}", chat, err);
            }
        }
    }

    pub fn good_morning() -> Answer {
        Answer::simple_text(format!(
            "Buongiorno sono Irina. Segui la mia morning routine per cominciare la tua giornata 👉 {}",
            MorningRoutine::get_random()
        ))
    }

    fn start() -> Answer {
        Answer::simple_text(
            "Ciao sono Irina e ti do il benvenuto in Spazio Grigio. Digita /help per cominciare",
        )
    }

    /// The answer to return in case of an error
    fn error(err: impl ToString) -> Answer {
        AnswerBuilder::default().text(err).finalize()
    }

    // get heroku port
    fn get_heroku_port() -> anyhow::Result<Option<u16>> {
        match std::env::var("PORT").map(|x| x.parse()) {
            Err(_) => Ok(None),
            Ok(Ok(p)) => Ok(Some(p)),
            Ok(Err(e)) => anyhow::bail!("could not parse PORT environment variable: {}", e),
        }
    }
}
