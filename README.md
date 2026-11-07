# palworld-cli

[![Crates.io](https://img.shields.io/crates/v/palworld-cli.svg)](https://crates.io/crates/palworld-cli)
[![Documentation](https://docs.rs/palworld-cli/badge.svg)](https://docs.rs/palworld-cli)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Unified command-line administration tool for [Palworld Dedicated Servers](https://docs.palworldgame.com/), written in pure Rust.

Combines the official **REST API** (HTTP Basic Auth, default port `8212`) and **binary RCON** (Source-dialect TCP, default port `25575`) into a single, blazing-fast, and ergonomic interface.

---

## Features

- **Full REST API Coverage**: Query server status, inspect metrics, list connected players, inspect gameplay settings, retrieve live world entity snapshots (`game-data`), broadcast in-game messages, trigger immediate world saves, and manage player moderation (kick, ban, unban, shutdown, stop).
- **Native Binary RCON**: Execute raw console commands directly over TCP RCON via [`rconet`](https://crates.io/crates/rconet). Automatically compensates for Unreal Engine's packet ID 0 response quirk.
- **Live Terminal Dashboard (`watch`)**: Real-time terminal monitor continuously updating server tick FPS, frame time, player counts, uptime, and connected player locations.
- **Dual Output Modes**:
  - **Human-Friendly Tables**: Styled Unicode terminal tables rendered via `comfy-table`.
  - **Machine-Readable JSON (`--json`)**: Output structured JSON for automation scripts, monitoring stacks (Datadog, Prometheus, Telegraf), and CI/CD pipelines.
- **Zero-Panic Safety**: Pure asynchronous Rust using Tokio, strict lints (`forbid(unsafe_code)`), comprehensive typed error handling (`thiserror`), and zero panics.

---

## Server Prerequisites

Enable the REST API and/or RCON in your server's `PalWorldSettings.ini`:

```ini
[/Script/Pal.PalGameUserSettings]
; REST API Configuration
RESTAPIEnabled=True
RESTAPIPort=8212

; RCON Configuration
RCONEnabled=True
RCONPort=25575

; Server Authentication (used for both REST API and RCON)
AdminPassword="your-admin-password"
```

> **Tip:** Launch the server binary with the `-enable-gamedata-api` flag to unlock the `game-data` actor snapshot endpoint (`GET /v1/api/game-data`).

---

## Installation

### From crates.io
```bash
cargo install palworld-cli
```

### From source
```bash
git clone https://github.com/realalden27/palworld-cli.git
cd palworld-cli
cargo install --path .
```

---

## Configuration & Authentication

Connection parameters can be passed as CLI arguments or configured via environment variables:

| Argument | Environment Variable | Default | Description |
|---|---|---|---|
| `-H`, `--host <HOST>` | `PALWORLD_HOST` | `127.0.0.1` | Palworld server host or IP address |
| `--api-port <PORT>` | `PALWORLD_API_PORT` | `8212` | REST API HTTP port |
| `--rcon-port <PORT>` | `PALWORLD_RCON_PORT` | `25575` | Binary RCON TCP port |
| `-p`, `--password <PASS>` | `PALWORLD_PASSWORD` | *(none)* | Server `AdminPassword` |
| `-t`, `--timeout <SECS>` | `PALWORLD_TIMEOUT` | `10` | Network request timeout in seconds |
| `--json` | *(none)* | `false` | Output responses in structured JSON format |

You can export `PALWORLD_PASSWORD` in your shell or `.env` to avoid passing `--password` each time:

```bash
export PALWORLD_HOST="127.0.0.1"
export PALWORLD_PASSWORD="your-admin-password"
```

---

## Command Reference

### Server Information & Metrics

#### `info`
Queries general server metadata (version, server name, world GUID):
```bash
palworld-cli info
```
```text
┌─────────────┬──────────────────────────────────┐
│ Property    ┆ Value                            │
╞═════════════╪══════════════════════════════════╡
│ Server Name ┆ Default Palworld Server          │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Version     ┆ v1.0.5.102999                    │
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ World GUID  ┆ B879E2224347D776D08132A8097CA512 │
└─────────────┴──────────────────────────────────┘
```

#### `metrics` (alias: `status`)
Displays live performance metrics (FPS, tick rate, frame time, player count, uptime, base camps):
```bash
palworld-cli metrics
```
```text
┌────────────────┬────────────────┐
│ Metric         ┆ Value          │
╞════════════════╪════════════════╡
│ Server FPS     ┆ 32 (avg: 32.4) │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Frame Time     ┆ 31.04 ms       │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Active Players ┆ 1/32           │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Server Uptime  ┆ 31m 49s        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ In-Game Days   ┆ 1              │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Base Camps     ┆ 1              │
└────────────────┴────────────────┘
```

#### `players` (alias: `list`)
Lists all connected players with coordinates, ping, UID, and level:
```bash
palworld-cli players
```

#### `settings`
Retrieves active gameplay configuration (difficulty, capture rate, exp rate, spawn rates):
```bash
palworld-cli settings
```

#### `game-data` (alias: `gamedata`)
Retrieves world actor breakdown and active base camp locations:
```bash
palworld-cli game-data
```

---

### World Actions & Moderation

#### `announce` (alias: `broadcast`)
Broadcasts a banner message to all online players:
```bash
palworld-cli announce "Server reboot scheduled in 15 minutes!"
```

#### `save`
Forces an immediate world save to disk:
```bash
palworld-cli save
```

#### `kick`
Kicks a player from the server by user ID:
```bash
palworld-cli kick steam_00000000000000000 -m "Please reconnect"
```

#### `ban` / `unban`
Bans or unbans a player by platform user ID:
```bash
palworld-cli ban steam_00000000000000000 -m "Violating server rules"
palworld-cli unban steam_00000000000000000
```

#### `shutdown`
Schedules a graceful server shutdown with a countdown:
```bash
palworld-cli shutdown 60 -m "Server update incoming"
```

#### `stop`
Immediately stops the server without countdown:
```bash
palworld-cli stop
```

---

### Binary RCON (`rcon`)

Executes arbitrary console commands directly over TCP RCON port `25575`:

```bash
# Query server info via RCON
palworld-cli rcon "Info"

# Query connected players via RCON
palworld-cli rcon "ShowPlayers"

# Execute in-game save
palworld-cli rcon "Save"
```

---

### Live Dashboard (`watch`)

Spins up a real-time monitor refreshing metrics and connected players every `N` seconds:

```bash
palworld-cli watch -i 2
```

---

### Structured JSON Output (`--json`)

Any command supports `--json` for automation and script integration:

```bash
palworld-cli --json metrics
```
```json
{
  "serverfps": 32,
  "serverfpsaverage": 32.5099983215332,
  "currentplayernum": 1,
  "serverframetime": 31.031997680664062,
  "maxplayernum": 32,
  "uptime": 1933,
  "basecampnum": 1,
  "days": 1
}
```

---

## Security Best Practices

- Palworld's REST API and RCON communicate over plain HTTP/TCP by default.
- Never expose ports `8212` or `25575` directly to the public internet without an encrypted tunnel (WireGuard, Tailscale) or an authenticated reverse proxy (Nginx, Caddy with HTTPS).
- Avoid passing passwords on shared shell histories; prefer the `PALWORLD_PASSWORD` environment variable.

---

## License

Dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
