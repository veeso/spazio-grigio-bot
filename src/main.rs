const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod feed;
mod irina;
mod mail;
mod redis;
mod repository;
mod rsshub;
mod utils;
mod youtube;

use irina::Irina;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!(
        "spazio-grigio-bot v{} - developed by {}",
        APP_VERSION, APP_AUTHORS
    );
    let irina = Irina::init().await?;
    info!("application ready!");
    irina.run().await
}
