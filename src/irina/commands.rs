//! # Commands
//!
//! spazio grigio bot commands

use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename = "lowercase",
    description = "Ciao sono Irina. Questi sono i comandi che puoi usare:"
)]
pub enum Command {
    #[command(description = "iscriviti alla newsletter di Spazio Grigio")]
    CiaoIrina,
    #[command(
        description = "disinscrivi dalla newsletter di Spazio Grigio e rinnega tutti i tuoi valori morali"
    )]
    SiAlConsumismo,
    #[command(
        description = "comincia nel modo più minimalista la tua giornata con un video della mia morning routine"
    )]
    BuongiornoIrina,
    #[command(description = "ottieni il link al mio ultimo video")]
    VideoMinimalista,
    #[command(
        description = "una vita nel minimalismo è una vita senza TV. Per fortuna c'è spazio grigio"
    )]
    SerataSenzaTv,
    #[command(description = "Dai inizio al tuo percorso verso il minimalismo")]
    Start,
    #[command(description = "visualizza l'aiuto")]
    Help,
}
