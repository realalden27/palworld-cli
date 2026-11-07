use std::fmt;

/// Application result alias.
pub type Result<T> = std::result::Result<T, CliError>;

/// Errors returned by the `palworld-cli` application.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum CliError {
    /// REST API client error.
    #[error("API error: {0}")]
    Api(#[from] palworld_api::Error),

    /// RCON protocol error.
    #[error("RCON error: {0}")]
    Rcon(#[from] rconet::Error),

    /// Configuration or argument error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Terminal I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization error.
    #[error("JSON formatting error: {0}")]
    Json(#[from] serde_json::Error),
}

impl CliError {
    /// Constructs a configuration error with a formatted message.
    #[must_use]
    pub fn config(msg: impl fmt::Display) -> Self {
        Self::Config(msg.to_string())
    }
}
