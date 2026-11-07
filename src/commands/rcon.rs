use serde_json::json;

use crate::config::ConnectionArgs;
use crate::error::Result;
use crate::output::{print_json, print_raw};

/// Executes a raw console command over RCON.
pub async fn run_rcon(
    conn: &ConnectionArgs,
    command_parts: &[String],
    as_json: bool,
) -> Result<()> {
    let command = command_parts.join(" ");
    let client = conn.rcon_client().await?;
    let response = client.execute(&command).await?;
    let body = response.body();

    if as_json {
        print_json(&json!({
            "command": command,
            "response": body,
        }))
    } else {
        print_raw(body);
        Ok(())
    }
}
