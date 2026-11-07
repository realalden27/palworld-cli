use crate::config::ConnectionArgs;
use crate::error::Result;
use crate::output::{
    print_game_data, print_info, print_metrics, print_players, print_settings, print_success,
};

/// Queries and displays general server information.
pub async fn run_info(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    let info = client.server_info().await?;
    print_info(&info, as_json)
}

/// Queries and displays live server metrics.
pub async fn run_metrics(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    let metrics = client.server_metrics().await?;
    print_metrics(&metrics, as_json)
}

/// Lists currently connected players.
pub async fn run_players(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    let players = client.players().await?;
    print_players(&players, as_json)
}

/// Queries and displays server configuration settings.
pub async fn run_settings(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    let settings = client.server_settings().await?;
    print_settings(&settings, as_json)
}

/// Retrieves and summarizes the live world actor snapshot.
pub async fn run_game_data(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    let snapshot = client.game_data().await?;
    print_game_data(&snapshot, as_json)
}

/// Broadcasts an in-game announcement message to all players.
pub async fn run_announce(conn: &ConnectionArgs, message: &str, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    client.announce(message).await?;
    print_success("Announcement sent", Some(message), as_json)
}

/// Forces an immediate world save.
pub async fn run_save(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    client.save_world().await?;
    print_success("World save completed", None, as_json)
}

/// Kicks a player from the server by user ID.
pub async fn run_kick(
    conn: &ConnectionArgs,
    userid: &str,
    message: Option<&str>,
    as_json: bool,
) -> Result<()> {
    let client = conn.api_client()?;
    client.kick_player(userid, message).await?;
    print_success("Player kicked", Some(userid), as_json)
}

/// Bans a player from the server by user ID.
pub async fn run_ban(
    conn: &ConnectionArgs,
    userid: &str,
    message: Option<&str>,
    as_json: bool,
) -> Result<()> {
    let client = conn.api_client()?;
    client.ban_player(userid, message).await?;
    print_success("Player banned", Some(userid), as_json)
}

/// Lifts a ban for the specified user ID.
pub async fn run_unban(conn: &ConnectionArgs, userid: &str, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    client.unban_player(userid).await?;
    print_success("Player unbanned", Some(userid), as_json)
}

/// Schedules a graceful server shutdown with a countdown in seconds.
pub async fn run_shutdown(
    conn: &ConnectionArgs,
    seconds: u32,
    message: Option<&str>,
    as_json: bool,
) -> Result<()> {
    let client = conn.api_client()?;
    client.shutdown_server(seconds, message).await?;
    let detail = format!("{seconds}s countdown");
    print_success("Server shutdown scheduled", Some(&detail), as_json)
}

/// Forces an immediate server shutdown.
pub async fn run_stop(conn: &ConnectionArgs, as_json: bool) -> Result<()> {
    let client = conn.api_client()?;
    client.stop_server().await?;
    print_success("Server stop issued", None, as_json)
}
