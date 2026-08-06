# AGENTS.md — PixRoot Repository Guide

## Repository Structure

```
apps/desktop/          # Flutter Linux desktop app (Dart)
  lib/                 # Dart source code
  test/                # Flutter widget tests
  linux/               # Flutter Linux runner integration
  pubspec.yaml         # Dependencies & config
  analysis_options.yaml # Lint rules (flutter_lints)

services/daemon/       # Rust background daemon
  src/                 # Rust source (main.rs, lib.rs, modules)
  tests/               # Integration tests (socket_ipc.rs)
  Cargo.toml           # Rust dependencies

shared/protocol/       # API contract (single source of truth)
  schema/              # Contract definitions
  generated/           # Generated Dart/Rust types (future)

packaging/linux/       # Linux distribution assets (icons)
art/                   # Art sources & export pipeline
docs/architecture/     # System architecture docs
```

## Key Commands

### Flutter App (`apps/desktop/`)
```bash
cd apps/desktop
flutter pub get           # Install dependencies
flutter analyze           # Static analysis (lints)
flutter test              # Run widget tests
flutter build linux       # Build Linux release binary
flutter run -d linux      # Run on Linux desktop
```

### Rust Daemon (`services/daemon/`)
```bash
cd services/daemon
cargo build               # Debug build
cargo build --release     # Release build
cargo test                # Run all tests (unit + integration)
cargo check               # Fast type-check only
cargo clippy              # Lints
cargo run                 # Run daemon (starts DB, monitor, IPC socket)
```

## Architecture Conventions (from `docs/architecture/overview.md`)

- **Daemon modules**: `app` (wiring), `domain` (no Wayland deps), `engine` (depends on domain), `detectors` (produce facts), `ipc` (protocol translation), `storage` (interface), `notify`, `telemetry`
- **Protocol**: `shared/protocol/schema` is the single source of truth for UI↔daemon communication
- **Assets**: Runtime assets in `apps/desktop/assets/`; art sources in `art/source/`, exports in `art/exports/`
- **Packaging**: Linux icons in `packaging/linux/icons/`

## Testing Notes

- **Flutter**: `flutter test` runs widget tests in `test/`
- **Rust**: `cargo test` runs unit tests (in-module) + integration tests in `tests/`
- **Daemon integration test**: `tests/socket_ipc.rs` tests IPC socket communication

## Important Constraints

- Daemon `domain` must compile/test without Wayland/compositor access
- `engine` depends on `domain` types; does not talk directly to Wayland
- `detectors` produce observations (facts), not decisions
- `ipc` translates protocol messages ↔ internal calls; don't leak engine internals
- If UI and daemon disagree on a field, fix it in `shared/protocol/schema` first

## Environment Requirements

- Flutter SDK (^3.12.0) + Linux desktop support
- Rust toolchain (edition 2024)
- SQLite (bundled via `rusqlite` feature)