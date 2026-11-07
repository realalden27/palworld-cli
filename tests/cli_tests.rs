#![allow(clippy::panic)]

use clap::Parser;
use palworld_cli::output::format_duration;
use palworld_cli::{Cli, Command, ConnectionArgs};

#[test]
fn test_duration_formatting() {
    assert_eq!(format_duration(45), "45s");
    assert_eq!(format_duration(60), "1m 0s");
    assert_eq!(format_duration(125), "2m 5s");
    assert_eq!(format_duration(3600), "1h 0m 0s");
    assert_eq!(format_duration(3665), "1h 1m 5s");
    assert_eq!(format_duration(86400), "1d 0h 0m 0s");
    assert_eq!(format_duration(90061), "1d 1h 1m 1s");
}

#[test]
fn test_password_requirement() {
    let empty_args = ConnectionArgs {
        host: "127.0.0.1".into(),
        api_port: 8212,
        rcon_port: 25575,
        password: None,
        timeout: 10,
    };
    assert!(empty_args.require_password().is_err());

    let blank_args = ConnectionArgs {
        password: Some("   ".into()),
        ..empty_args.clone()
    };
    assert!(blank_args.require_password().is_err());

    let valid_args = ConnectionArgs {
        password: Some("admin123".into()),
        ..empty_args
    };
    assert_eq!(valid_args.require_password().ok(), Some("admin123"));
}

#[test]
fn test_cli_parse_info() {
    let cli = Cli::try_parse_from(["palworld-cli", "--password", "secret", "info"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert_eq!(cli.connection.password.as_deref(), Some("secret"));
    assert!(!cli.json);
    assert!(matches!(cli.command, Command::Info));
}

#[test]
fn test_cli_parse_metrics_with_json_and_alias() {
    let cli = Cli::try_parse_from(["palworld-cli", "--json", "status"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert!(cli.json);
    assert!(matches!(cli.command, Command::Metrics));
}

#[test]
fn test_cli_parse_players_with_alias() {
    let cli = Cli::try_parse_from(["palworld-cli", "list"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert!(matches!(cli.command, Command::Players));
}

#[test]
fn test_cli_parse_gamedata_alias() {
    let cli = Cli::try_parse_from(["palworld-cli", "gamedata"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert!(matches!(cli.command, Command::GameData));
}

#[test]
fn test_cli_parse_announce_with_alias() {
    let cli = Cli::try_parse_from(["palworld-cli", "broadcast", "Server restart in 5 minutes"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    match cli.command {
        Command::Announce { message } => {
            assert_eq!(message, "Server restart in 5 minutes");
        },
        _ => panic!("unexpected command variant"),
    }
}

#[test]
fn test_cli_parse_save() {
    let cli = Cli::try_parse_from(["palworld-cli", "save"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert!(matches!(cli.command, Command::Save));
}

#[test]
fn test_cli_parse_moderation_commands() {
    let kick = Cli::try_parse_from([
        "palworld-cli",
        "kick",
        "steam_12345",
        "--message",
        "relog please",
    ])
    .unwrap_or_else(|e| panic!("parse error: {e}"));

    match kick.command {
        Command::Kick { userid, message } => {
            assert_eq!(userid, "steam_12345");
            assert_eq!(message.as_deref(), Some("relog please"));
        },
        _ => panic!("unexpected command"),
    }

    let ban = Cli::try_parse_from(["palworld-cli", "ban", "steam_12345"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    match ban.command {
        Command::Ban { userid, message } => {
            assert_eq!(userid, "steam_12345");
            assert!(message.is_none());
        },
        _ => panic!("unexpected command"),
    }

    let unban = Cli::try_parse_from(["palworld-cli", "unban", "steam_12345"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    match unban.command {
        Command::Unban { userid } => {
            assert_eq!(userid, "steam_12345");
        },
        _ => panic!("unexpected command"),
    }
}

#[test]
fn test_cli_parse_shutdown_and_stop() {
    let shutdown = Cli::try_parse_from([
        "palworld-cli",
        "shutdown",
        "60",
        "--message",
        "Server reboot",
    ])
    .unwrap_or_else(|e| panic!("parse error: {e}"));

    match shutdown.command {
        Command::Shutdown { seconds, message } => {
            assert_eq!(seconds, 60);
            assert_eq!(message.as_deref(), Some("Server reboot"));
        },
        _ => panic!("unexpected command"),
    }

    let stop = Cli::try_parse_from(["palworld-cli", "stop"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert!(matches!(stop.command, Command::Stop));
}

#[test]
fn test_cli_parse_rcon_multi_arg() {
    let cli = Cli::try_parse_from(["palworld-cli", "rcon", "broadcast", "hello", "world"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    match cli.command {
        Command::Rcon { command } => {
            assert_eq!(command, vec!["broadcast", "hello", "world"]);
        },
        _ => panic!("unexpected command"),
    }
}

#[test]
fn test_cli_parse_watch() {
    let cli = Cli::try_parse_from(["palworld-cli", "watch", "--interval", "10"])
        .unwrap_or_else(|e| panic!("parse error: {e}"));

    match cli.command {
        Command::Watch { interval } => {
            assert_eq!(interval, 10);
        },
        _ => panic!("unexpected command"),
    }
}

#[test]
fn test_cli_custom_connection_flags() {
    let cli = Cli::try_parse_from([
        "palworld-cli",
        "-H",
        "192.168.1.50",
        "--api-port",
        "9999",
        "--rcon-port",
        "29999",
        "-p",
        "topsecret",
        "-t",
        "30",
        "info",
    ])
    .unwrap_or_else(|e| panic!("parse error: {e}"));

    assert_eq!(cli.connection.host, "192.168.1.50");
    assert_eq!(cli.connection.api_port, 9999);
    assert_eq!(cli.connection.rcon_port, 29999);
    assert_eq!(cli.connection.password.as_deref(), Some("topsecret"));
    assert_eq!(cli.connection.timeout, 30);
}
