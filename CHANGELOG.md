# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-11-07

### Changed
- Sanitized documentation and CLI examples with standard dummy RFC placeholder user IDs.
- Switched crate dependencies to published crates.io registry releases.

## [0.1.0] - 2026-11-07

### Added
- Initial release of `palworld-cli`.
- Complete Palworld REST API command set:
  - `info`: Server version, server name, and world GUID inspection.
  - `metrics` (alias `status`): Real-time server FPS, average FPS, frame time, player count, uptime, and base camps.
  - `players` (alias `list`): Connected players with coordinates, ping, UID, account name, and structures.
  - `settings`: Full server configuration values and gameplay settings.
  - `game-data` (alias `gamedata`): World actor summary (Pals, players, NPCs, base camps).
  - `announce` (alias `broadcast`): In-game message broadcasting.
  - `save`: Force immediate world save.
  - `kick`, `ban`, `unban`: Player moderation commands.
  - `shutdown`, `stop`: Graceful countdown shutdown and immediate stop.
- Raw TCP binary RCON via `rconet` (`rcon` subcommand) with automatic Unreal Engine packet ID 0 handling.
- Live refreshing terminal dashboard (`watch` subcommand) with Ctrl+C graceful handling.
- Dual output formats: Prettified Unicode tables (`comfy-table`) and structured machine-readable JSON (`--json`).
- Strict zero-panic error handling and comprehensive integration test suite.
