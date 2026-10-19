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

AppImages need `libfuse2` (`fuse2` on Arch, `libfuse2` on Ubuntu). Without it, extract the file with `--appimage-extract` and run `squashfs-root/AppRun`.

### Building the AppImage yourself

Build the release binaries first (see Release above). Then package with AppImageKit's [appimagetool](https://github.com/AppImage/AppImageKit/releases/tag/continuous) (the `continuous` build; `release.yml` pins its checksum):

```bash
chmod +x appimagetool-x86_64.AppImage
packaging/linux/build-appimage.sh ./appimagetool-x86_64.AppImage
packaging/linux/smoke-test-appimage.sh PixRoot-x86_64.AppImage   # optional: launches it under Xvfb
```

The script assembles the AppDir (Flutter bundle, the daemon named `database`, and `AppRun`, the desktop file and the icon from `packaging/linux/`) and packs it with gzip, the one compression that both AppImageLauncher's runtime and AppImageHub's checker can read.

Use AppImageKit's tool, not the newer appimagetool 1.9+. Those builds use a static runtime that AppImageLauncher 2.x cannot launch, and clearing the AppImage marker to work around that stops launchers from recognizing the file.

Note: `AppRun` does not start the daemon — the Flutter app spawns it (resolved relative to its own executable).
