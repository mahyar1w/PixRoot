## docs/ — documentation only (no code)
no code in here, just simple system description (not details)
* docs/architecture/
  * overview.md: over all diagram.
* docs/product/

## apps/ — end-user application (UI)
### apps/desktop — flutter linux desktop app
* desktop/linux/ — flutter Linux runner integration.
* desktop/assets/ — runtime assets used by the UI (only exported assets that the app loads at runtime go here).
* desktop/lib/ — all dart source code.
* desktop/lib/app/ — entry point (should stay thin, initial wiring, just composition and wiring).
* desktop/lib/shared/ — reusable UI code used by multiple features.

## services/ — background components
### services/daemon — rust daemon
Note: cargo.toml and cargo.lock are in here
* daemon/src/ — startup, wiring (start runtime)
* daemon/src/app — wires modules together; it does not decide focus rules.
* daemon/src/domain — domain must compile and be testable without Wayland/compositor access.
* daemon/src/engine — engine should depend on domain types; it should not talk directly to Wayland.
* daemon/src/detectors — detectors produce “facts/observations” for the engine (e.g., active app id, window title), not decisions.
  * detectors/idle — idle detection or inhibit (if you add it).
  * detectors/toplevel — active window detection implementations.
* daemon/src/ipc — IPC layer should translate between protocol messages and internal calls; avoid leaking engine internals out.
* daemon/src/storage — engine calls storage through an interface; storage details don’t spread.
* daemon/src/notify — desktop notifications. Keep it isolated to avoid coupling.
* daemon/src/telemetry — logging/tracing setup helpers.
* daemon/scripts — developer scripts (run daemon with flags, etc.)
* daemon/tests — Rust tests beyond unit tests inside modules.

## shared/ — shared contracts between UI and daemon
### shared/protocol — the API contract
This is the single source of truth for communication.
* shared/protocol/schema — The actual contract definition.
* shared/protocol/generated — for later generate Dart/Rust types from schema.
Note: If UI and daemon disagree about a field name/type, that’s a protocol bug—fix it here first.

### shared/domain 
High-level shared domain documentation (not code), or if you later choose to share generated domain types.

## packaging/ — install/distribution materials
This folder is still important so packaging doesn’t pollute app code.
### packaging/linux/ — (enough for now)
* linux/icons — install icons in required sizes.

## art/ — art sources and export pipeline
* art/source/ — Editable originals.
* art/exports/ — Exported PNGs, These are typically copied/synced into apps/desktop/assets/sprites/ for runtime.
* art/pipeline/ — Optional export scripts.
Note: App assets must be stable exports; source files stay out of the runtime assets directory.

## tools/
### tools/scripts/
Anything used by developers/CI but not shipped goes here.


