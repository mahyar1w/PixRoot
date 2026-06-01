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