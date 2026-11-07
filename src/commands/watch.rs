use std::io::{Write, stdout};
use std::time::Duration;

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};

use crate::config::ConnectionArgs;
use crate::error::Result;
use crate::output::{print_metrics, print_players};

/// Runs a live refreshing dashboard displaying server metrics and active players.
pub async fn run_watch(conn: &ConnectionArgs, interval_secs: u64) -> Result<()> {
    let client = conn.api_client()?;
    let interval = Duration::from_secs(interval_secs.max(1));

    println!(
        "Starting Palworld dashboard (refreshing every {interval_secs}s, press Ctrl+C to stop)..."
    );

    loop {
        let metrics_res = client.server_metrics().await;
        let players_res = client.players().await;

        let mut out = stdout();
        let _ = execute!(out, Clear(ClearType::All), MoveTo(0, 0));
        let _ = out.flush();

        println!("=== Palworld Server Monitor (refresh: {interval_secs}s | Ctrl+C to quit) ===\n");

        match metrics_res {
            Ok(metrics) => {
                println!("--- Performance Metrics ---");
                let _ = print_metrics(&metrics, false);
            },
            Err(e) => {
                println!("Error fetching metrics: {e}");
            },
        }

        println!();

        match players_res {
            Ok(players) => {
                println!("--- Connected Players ({}) ---", players.len());
                let _ = print_players(&players, false);
            },
            Err(e) => {
                println!("Error fetching players: {e}");
            },
        }

        tokio::select! {
            () = tokio::time::sleep(interval) => {}
            res = tokio::signal::ctrl_c() => {
                res?;
                println!("\nDashboard stopped.");
                break;
            }
        }
    }

    Ok(())
}
