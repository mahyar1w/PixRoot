# PixRoot

PixRoot is a focus/productivity desktop app: a Flutter Linux UI (frontend) paired with a Rust daemon (backend). The daemon stores data in SQLite, monitors open applications, and streams blocking/interrupt events to the UI over a Unix socket.

## Repository layout

| Path                    | Role                                                |
| ----------------------- | --------------------------------------------------- |
| `apps/desktop/`         | Flutter Linux desktop app (UI)                      |
| `services/daemon/`      | Rust daemon (SQLite storage, app monitor, IPC)      |
| `shared/protocol/`      | UI ↔ daemon API contract                            |
| `packaging/linux/`      | Linux packaging assets (icons)                      |
| `art/`                  | Art sources & export pipeline                       |

The UI and daemon communicate over a Unix socket at `/tmp/focus_app.sock`. The daemon keeps its database at `~/.local/share/pixroot/data.db` (XDG data directory).

## Prerequisites

- Flutter SDK (^3.12.0) with Linux desktop support
- Rust toolchain (edition 2024)

## Debug

The Flutter app launches the daemon itself, so build the daemon first, then just run the app.

```bash
cd services/daemon
cargo build               # daemon binary: target/debug/database

cd ../desktop
flutter pub get
flutter run -d linux      # builds + runs the debug bundle and spawns the daemon
```

## Release

```bash
cd services/daemon
cargo build --release     # daemon binary: target/release/database

cd ../desktop
flutter build linux --release
./build/linux/x64/release/bundle/desktop
```

The release bundle is at `apps/desktop/build/linux/x64/release/bundle/`. To run it standalone (outside the repository), copy the daemon next to the bundle first:

```bash
cp services/daemon/target/release/database \
   apps/desktop/build/linux/x64/release/bundle/database
```

## AppImage

Prebuilt AppImages bundle the release Flutter app and the Rust daemon into a single portable executable. They are attached to the **GitHub Releases** page — download the latest one and run:

```bash
chmod +x PixRoot-x86_64.AppImage
./PixRoot-x86_64.AppImage
```

The Flutter app spawns the daemon found next to itself (`usr/bin/database`), so no separate install is needed. Data still lands in `~/.local/share/pixroot/data.db`.

### Building the AppImage yourself

Requires [appimagetool](https://github.com/AppImage/AppImageKit). Build the release binaries first (see Release above), then:

```bash
APP_DIR=/tmp/PixRoot.AppDir
mkdir -p $APP_DIR/usr/bin $APP_DIR/usr/share/applications \
         $APP_DIR/usr/share/icons/hicolor/256x256/apps

# Flutter bundle + daemon (the daemon must keep the name `database`)
cp -r apps/desktop/build/linux/x64/release/bundle/* $APP_DIR/usr/bin/
cp services/daemon/target/release/database $APP_DIR/usr/bin/database

# Icon
cp art/exports/sprites/part1.png \
   $APP_DIR/usr/share/icons/hicolor/256x256/apps/pixroot.png
cp $APP_DIR/usr/share/icons/hicolor/256x256/apps/pixroot.png $APP_DIR/pixroot.png
```

Create `$APP_DIR/AppRun`:

```bash
#!/bin/bash
HERE="$(dirname "$(readlink -f "${0}")")"
export LD_LIBRARY_PATH="${HERE}/usr/bin/lib:${LD_LIBRARY_PATH}"
export PATH="${HERE}/usr/bin:${PATH}"
exec "${HERE}/usr/bin/desktop" "$@"
```

Create `$APP_DIR/pixroot.desktop`:

```ini
[Desktop Entry]
Type=Application
Name=PixRoot
Comment=PixRoot - focus and productivity tracking
Exec=AppRun
Icon=pixroot
Terminal=false
Categories=Utility;Office;
StartupNotify=true
```

Package it:

```bash
chmod +x $APP_DIR/AppRun
appimagetool $APP_DIR PixRoot-x86_64.AppImage
```

Note: `AppRun` does not start the daemon — the Flutter app spawns it (resolved relative to its own executable).
