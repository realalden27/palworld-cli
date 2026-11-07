use clap::{Args, Parser, Subcommand};
use palworld_api::PalworldClient;
use rconet::{Client as RconClient, Preset as RconPreset};
use std::time::Duration;

use crate::error::{CliError, Result};

/// Unified CLI tool for Palworld Dedicated Server administration via REST API and RCON.
#[derive(Debug, Parser)]
#[command(
    name = "palworld-cli",
    version,
    about = "Unified CLI for Palworld Dedicated Server administration via REST API and RCON",
    long_about = "Fast, production-grade CLI providing complete management for Palworld Dedicated Servers.\nCombines the official REST API with binary RCON in a single tool."
)]
pub struct Cli {
    #[command(flatten)]
    pub connection: ConnectionArgs,

    /// Output responses in structured JSON format instead of human-friendly tables
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

/// Global server connection arguments.
#[derive(Debug, Args, Clone)]
pub struct ConnectionArgs {
    /// Palworld server hostname or IP address
    #[arg(
        short = 'H',
        long,
        env = "PALWORLD_HOST",
        default_value = "127.0.0.1",
        global = true
    )]
    pub host: String,

    /// REST API port (default: 8212)
    #[arg(long, env = "PALWORLD_API_PORT", default_value_t = 8212, global = true)]
    pub api_port: u16,

    /// RCON TCP port (default: 25575)
    #[arg(
        long,
        env = "PALWORLD_RCON_PORT",
        default_value_t = 25575,
        global = true
    )]
    pub rcon_port: u16,

    /// Server admin password (PalWorldSettings.ini AdminPassword)
    #[arg(short = 'p', long, env = "PALWORLD_PASSWORD", global = true)]
    pub password: Option<String>,

    /// Request timeout in seconds
    #[arg(
        short = 't',
        long,
        env = "PALWORLD_TIMEOUT",
        default_value_t = 10,
        global = true
    )]
    pub timeout: u64,
}

impl ConnectionArgs {
    /// Resolves the admin password or returns an actionable error.
    pub fn require_password(&self) -> Result<&str> {
        self.password
            .as_deref()
            .filter(|p| !p.trim().is_empty())
            .ok_or_else(|| {
                CliError::config(
                    "Admin password required. Provide --password <PASS> or set the PALWORLD_PASSWORD environment variable.",
                )
            })
    }

    /// Creates an authenticated [`PalworldClient`] targeting the REST API endpoint.
    pub fn api_client(&self) -> Result<PalworldClient> {
        let password = self.require_password()?;
        let base_url = format!("http://{}:{}", self.host, self.api_port);
        let client = PalworldClient::builder(base_url, password)
            .timeout(Duration::from_secs(self.timeout))
            .connect_timeout(Duration::from_secs(5))
            .build()?;
        Ok(client)
    }

    /// Connects an authenticated [`RconClient`] targeting the RCON port using [`RconPreset::Palworld`].
    pub async fn rcon_client(&self) -> Result<RconClient> {
        let password = self.require_password()?;
        let addr = format!("{}:{}", self.host, self.rcon_port);
        let client = RconClient::builder()
            .address(addr)
            .password(password)
            .preset(RconPreset::Palworld)
            .command_timeout(Duration::from_secs(self.timeout))
            .connect()
            .await?;
        Ok(client)
    }
}

/// Available management subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Retrieve general server information (version, name, GUID) via REST API
    Info,

    /// Display live server performance metrics (FPS, players, uptime, base camps)
    #[command(alias = "status")]
    Metrics,

    /// List all currently connected players
    #[command(alias = "list")]
    Players,

    /// Query active server gameplay and network settings
    Settings,

    /// Retrieve live world actor snapshot (requires server flag -enable-gamedata-api)
    #[command(alias = "gamedata")]
    GameData,

    /// Broadcast an in-game announcement message to all connected players
    #[command(alias = "broadcast")]
    Announce {
        /// Message text to display to all players
        message: String,
    },

    /// Force an immediate world save
    Save,

    /// Kick a player from the server
    Kick {
        /// User ID (e.g. steam_00000000000000000)
        userid: String,
        /// Optional message displayed to the kicked player
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Ban a player from the server
    Ban {
        /// User ID (e.g. steam_00000000000000000)
        userid: String,
        /// Optional message displayed to the banned player
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Lift a ban for a specified user ID
    Unban {
        /// User ID to unban
        userid: String,
    },

    /// Schedule a graceful server shutdown with a countdown
    Shutdown {
        /// Countdown time to wait in seconds
        seconds: u32,
        /// Optional warning message displayed before shutdown
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Force an immediate server stop without a countdown
    Stop,

    /// Execute a raw console command over TCP RCON via rconet
    Rcon {
        /// Command to execute (e.g. "ShowPlayers", "Info", "Save", "DoExit")
        #[arg(required = true, num_args = 1..)]
        command: Vec<String>,
    },

    /// Live terminal dashboard refreshing server metrics and players continuously
    Watch {
        /// Refresh interval in seconds
        #[arg(short = 'i', long, default_value_t = 2)]
        interval: u64,
    },
}
