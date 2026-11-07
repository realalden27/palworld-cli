pub mod api;
pub mod rcon;
pub mod watch;

use crate::config::{Cli, Command};
use crate::error::Result;

/// Dispatches a parsed CLI command to its appropriate handler.
pub async fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Info => api::run_info(&cli.connection, cli.json).await,
        Command::Metrics => api::run_metrics(&cli.connection, cli.json).await,
        Command::Players => api::run_players(&cli.connection, cli.json).await,
        Command::Settings => api::run_settings(&cli.connection, cli.json).await,
        Command::GameData => api::run_game_data(&cli.connection, cli.json).await,
        Command::Announce { message } => {
            api::run_announce(&cli.connection, &message, cli.json).await
        },
        Command::Save => api::run_save(&cli.connection, cli.json).await,
        Command::Kick { userid, message } => {
            api::run_kick(&cli.connection, &userid, message.as_deref(), cli.json).await
        },
        Command::Ban { userid, message } => {
            api::run_ban(&cli.connection, &userid, message.as_deref(), cli.json).await
        },
        Command::Unban { userid } => api::run_unban(&cli.connection, &userid, cli.json).await,
        Command::Shutdown { seconds, message } => {
            api::run_shutdown(&cli.connection, seconds, message.as_deref(), cli.json).await
        },
        Command::Stop => api::run_stop(&cli.connection, cli.json).await,
        Command::Rcon { command } => rcon::run_rcon(&cli.connection, &command, cli.json).await,
        Command::Watch { interval } => watch::run_watch(&cli.connection, interval).await,
    }
}
