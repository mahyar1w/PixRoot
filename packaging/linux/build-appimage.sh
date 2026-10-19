#!/usr/bin/env bash
# Assembles PixRoot-x86_64.AppImage from the release Flutter bundle and daemon.
# Usage: packaging/linux/build-appimage.sh <path-to-appimagetool>
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
APPTOOL="${1:?usage: build-appimage.sh <path-to-appimagetool>}"
BUNDLE="$ROOT/apps/desktop/build/linux/x64/release/bundle"
DAEMON="$ROOT/services/daemon/target/release/database"
APPDIR="$(mktemp -d)/PixRoot.AppDir"

[ -x "$BUNDLE/desktop" ] || { echo "missing Flutter bundle: run 'flutter build linux --release' first" >&2; exit 1; }
[ -x "$DAEMON" ] || { echo "missing daemon: run 'cargo build --release' first" >&2; exit 1; }

mkdir -p "$APPDIR/usr/bin" "$APPDIR/usr/share/applications" "$APPDIR/usr/share/icons/hicolor/128x128/apps"
cp -r "$BUNDLE"/. "$APPDIR/usr/bin/"
cp "$DAEMON" "$APPDIR/usr/bin/database"
install -m 755 "$ROOT/packaging/linux/AppRun" "$APPDIR/AppRun"
install -m 644 "$ROOT/packaging/linux/pixroot.desktop" "$APPDIR/pixroot.desktop"
install -m 644 "$ROOT/packaging/linux/pixroot.desktop" "$APPDIR/usr/share/applications/pixroot.desktop"
install -m 644 "$ROOT/packaging/linux/icons/pixroot.png" "$APPDIR/pixroot.png"
install -m 644 "$ROOT/packaging/linux/icons/pixroot.png" "$APPDIR/usr/share/icons/hicolor/128x128/apps/pixroot.png"

ARCH=x86_64 "$APPTOOL" "$APPDIR" "$ROOT/PixRoot-x86_64.AppImage"
