use std::fmt::Display;
use std::time::Duration;

use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use palworld_api::{
    Player, ServerInfo, ServerMetrics, ServerSettings, UnitType, WorldActor, WorldSnapshot,
};
use serde_json::json;

use crate::error::Result;

/// Formats a duration in seconds into a human-readable string.
#[must_use]
pub fn format_duration(seconds: u64) -> String {
    let dur = Duration::from_secs(seconds);
    let total_secs = dur.as_secs();
    let days = total_secs / 86400;
    let hours = (total_secs % 86400) / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if days > 0 {
        format!("{days}d {hours}h {minutes}m {secs}s")
    } else if hours > 0 {
        format!("{hours}h {minutes}m {secs}s")
    } else if minutes > 0 {
        format!("{minutes}m {secs}s")
    } else {
        format!("{secs}s")
    }
}

/// Creates a standardized, pre-styled terminal table.
#[must_use]
pub fn new_table() -> Table {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table
}

/// Prints any serializable value as formatted JSON to standard output.
pub fn print_json<T: serde::Serialize>(val: &T) -> Result<()> {
    let serialized = serde_json::to_string_pretty(val)?;
    println!("{serialized}");
    Ok(())
}

/// Prints a standard success acknowledgement in either table or JSON format.
pub fn print_success(action: &str, details: Option<&str>, as_json: bool) -> Result<()> {
    if as_json {
        print_json(&json!({
            "status": "success",
            "action": action,
            "details": details,
        }))
    } else {
        match details {
            Some(d) => println!("✔ {action}: {d}"),
            None => println!("✔ {action}"),
        }
        Ok(())
    }
}

/// Displays server info in either table or JSON format.
pub fn print_info(info: &ServerInfo, as_json: bool) -> Result<()> {
    if as_json {
        return print_json(info);
    }

    let mut table = new_table();
    table.set_header(vec![
        Cell::new("Property").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Cyan),
    ]);

    table.add_row(vec!["Server Name", &info.server_name]);
    table.add_row(vec!["Version", &info.version]);
    table.add_row(vec!["World GUID", &info.world_guid]);
    if !info.description.is_empty() {
        table.add_row(vec!["Description", &info.description]);
    }

    println!("{table}");
    Ok(())
}

/// Displays server metrics in either table or JSON format.
pub fn print_metrics(metrics: &ServerMetrics, as_json: bool) -> Result<()> {
    if as_json {
        return print_json(metrics);
    }

    let mut table = new_table();
    table.set_header(vec![
        Cell::new("Metric").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Cyan),
    ]);

    let fps_text = format!(
        "{} (avg: {:.1})",
        metrics.server_fps, metrics.server_fps_average
    );
    let players_text = format!("{}/{}", metrics.current_player_num, metrics.max_player_num);
    let frame_time_text = format!("{:.2} ms", metrics.server_frame_time);
    let uptime_text = format_duration(metrics.uptime);
    let days_text = metrics.days.to_string();
    let base_camps_text = metrics.base_camp_num.to_string();

    table.add_row(vec!["Server FPS", &fps_text]);
    table.add_row(vec!["Frame Time", &frame_time_text]);
    table.add_row(vec!["Active Players", &players_text]);
    table.add_row(vec!["Server Uptime", &uptime_text]);
    table.add_row(vec!["In-Game Days", &days_text]);
    table.add_row(vec!["Base Camps", &base_camps_text]);

    println!("{table}");
    Ok(())
}

/// Displays connected players list in either table or JSON format.
pub fn print_players(players: &[Player], as_json: bool) -> Result<()> {
    if as_json {
        return print_json(&players);
    }

    if players.is_empty() {
        println!("No players currently connected.");
        return Ok(());
    }

    let mut table = new_table();
    table.set_header(vec![
        Cell::new("Name").fg(Color::Cyan),
        Cell::new("Level").fg(Color::Cyan),
        Cell::new("User ID").fg(Color::Cyan),
        Cell::new("Player UID").fg(Color::Cyan),
        Cell::new("Ping").fg(Color::Cyan),
        Cell::new("Location (X, Y)").fg(Color::Cyan),
        Cell::new("Structures").fg(Color::Cyan),
    ]);

    for p in players {
        let level_str = p.level.to_string();
        let ping_str = format!("{:.0} ms", p.ping);
        let loc_str = format!("{:.0}, {:.0}", p.location_x, p.location_y);
        let bld_str = p.building_count.to_string();

        table.add_row(vec![
            p.name.as_str(),
            level_str.as_str(),
            p.user_id.as_str(),
            p.player_id.as_str(),
            ping_str.as_str(),
            loc_str.as_str(),
            bld_str.as_str(),
        ]);
    }

    println!("{table}");
    Ok(())
}

/// Displays active server settings in either table or JSON format.
pub fn print_settings(settings: &ServerSettings, as_json: bool) -> Result<()> {
    if as_json {
        return print_json(settings);
    }

    let serialized = serde_json::to_value(settings)?;
    let Some(obj) = serialized.as_object() else {
        return print_json(settings);
    };

    let mut table = new_table();
    table.set_header(vec![
        Cell::new("Setting").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Cyan),
    ]);

    for (k, v) in obj {
        let val_str = match v {
            serde_json::Value::String(s) => s.clone(),
            _ => v.to_string(),
        };
        table.add_row(vec![k.as_str(), val_str.as_str()]);
    }

    println!("{table}");
    Ok(())
}

/// Summary counts of actors in a game data snapshot.
#[derive(Debug, Default)]
struct ActorCounts {
    players: usize,
    wild_pals: usize,
    base_pals: usize,
    otomo_pals: usize,
    npcs: usize,
    palboxes: usize,
    other: usize,
}

/// Displays game data world snapshot in either table or JSON format.
pub fn print_game_data(snapshot: &WorldSnapshot, as_json: bool) -> Result<()> {
    if as_json {
        return print_json(snapshot);
    }

    let mut summary_table = new_table();
    summary_table.set_header(vec![
        Cell::new("World Property").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Cyan),
    ]);

    let fps_str = format!("{:.1} (avg: {:.1})", snapshot.fps, snapshot.average_fps);
    let days_str = snapshot.in_game_days.to_string();
    let actors_count_str = snapshot.actor_data.len().to_string();

    summary_table.add_row(vec!["Timestamp", snapshot.time.as_str()]);
    summary_table.add_row(vec!["Server FPS", fps_str.as_str()]);
    summary_table.add_row(vec!["In-Game Clock", snapshot.in_game_time.as_str()]);
    summary_table.add_row(vec!["In-Game Days", days_str.as_str()]);
    summary_table.add_row(vec!["Total Actors", actors_count_str.as_str()]);

    println!("{summary_table}\n");

    let mut counts = ActorCounts::default();
    let mut named_players = Vec::new();
    let mut base_camps = Vec::new();

    for actor in &snapshot.actor_data {
        match actor {
            WorldActor::Character(ch) => match &ch.unit_type {
                UnitType::Player => {
                    counts.players = counts.players.saturating_add(1);
                    named_players.push((ch.nick_name.clone(), ch.level, ch.hp, ch.max_hp));
                },
                UnitType::WildPal => counts.wild_pals = counts.wild_pals.saturating_add(1),
                UnitType::BaseCampPal => counts.base_pals = counts.base_pals.saturating_add(1),
                UnitType::OtomoPal => counts.otomo_pals = counts.otomo_pals.saturating_add(1),
                UnitType::Npc => counts.npcs = counts.npcs.saturating_add(1),
                UnitType::Other(_) => counts.other = counts.other.saturating_add(1),
            },
            WorldActor::PalBox(pb) => {
                counts.palboxes = counts.palboxes.saturating_add(1);
                base_camps.push((pb.name.clone(), pb.guild_name.clone()));
            },
            WorldActor::Unknown(_) => counts.other = counts.other.saturating_add(1),
        }
    }

    let mut breakdown_table = new_table();
    breakdown_table.set_header(vec![
        Cell::new("Actor Type").fg(Color::Cyan),
        Cell::new("Count").fg(Color::Cyan),
    ]);

    let pl_str = counts.players.to_string();
    let wp_str = counts.wild_pals.to_string();
    let bp_str = counts.base_pals.to_string();
    let op_str = counts.otomo_pals.to_string();
    let npc_str = counts.npcs.to_string();
    let pb_str = counts.palboxes.to_string();
    let other_str = counts.other.to_string();

    breakdown_table.add_row(vec!["Players", pl_str.as_str()]);
    breakdown_table.add_row(vec!["Wild Pals", wp_str.as_str()]);
    breakdown_table.add_row(vec!["Base Camp Pals", bp_str.as_str()]);
    breakdown_table.add_row(vec!["Companion Pals (Otomo)", op_str.as_str()]);
    breakdown_table.add_row(vec!["NPCs", npc_str.as_str()]);
    breakdown_table.add_row(vec!["PalBoxes (Base Camps)", pb_str.as_str()]);
    if counts.other > 0 {
        breakdown_table.add_row(vec!["Other Entities", other_str.as_str()]);
    }

    println!("{breakdown_table}");

    if !named_players.is_empty() {
        println!();
        let mut players_table = new_table();
        players_table.set_header(vec![
            Cell::new("Player Name").fg(Color::Green),
            Cell::new("Level").fg(Color::Green),
            Cell::new("HP").fg(Color::Green),
        ]);
        for (name, lvl, hp, max_hp) in named_players {
            let lvl_str = lvl.to_string();
            let hp_str = format!("{hp} / {max_hp}");
            players_table.add_row(vec![name.as_str(), lvl_str.as_str(), hp_str.as_str()]);
        }
        println!("{players_table}");
    }

    if !base_camps.is_empty() {
        println!();
        let mut camps_table = new_table();
        camps_table.set_header(vec![
            Cell::new("Base Camp Name").fg(Color::Yellow),
            Cell::new("Guild").fg(Color::Yellow),
        ]);
        for (name, guild) in base_camps {
            camps_table.add_row(vec![name.as_str(), guild.as_str()]);
        }
        println!("{camps_table}");
    }

    Ok(())
}

/// Raw message printer for non-structured console output.
pub fn print_raw(msg: impl Display) {
    println!("{msg}");
}
