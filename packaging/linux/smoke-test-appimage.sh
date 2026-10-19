#!/usr/bin/env bash
# Launches a packaged PixRoot under a virtual X server and checks that the app
# stays up, spawns its daemon, and that the daemon's socket is listening.
# Usage: packaging/linux/smoke-test-appimage.sh <PixRoot-x86_64.AppImage | PixRoot.AppDir>
set -euo pipefail

TARGET="$(realpath "${1:?usage: smoke-test-appimage.sh <AppImage or AppDir>}")"
WORK="$(mktemp -d)"
trap 'pkill -f "${APP:-$WORK}/" 2>/dev/null || true; rm -rf "$WORK"' EXIT

if [ -d "$TARGET" ]; then
  APP="$TARGET"
else
  (cd "$WORK" && "$TARGET" --appimage-extract >/dev/null)
  APP="$WORK/squashfs-root"
fi

# Every shared library must resolve on this host, or the app dies on first launch.
if ldd "$APP/usr/bin/desktop" "$APP/usr/bin/database" | grep -q "not found"; then
  ldd "$APP/usr/bin/desktop" "$APP/usr/bin/database" | grep "not found" >&2
  echo "FAIL: missing shared libraries" >&2
  exit 1
fi

# Keep the daemon's database out of the real user data directory.
export XDG_DATA_HOME="$WORK/xdg"

xvfb-run -a "$APP/AppRun" >"$WORK/app.log" 2>&1 &
APP_PID=$!

# The app spawns the daemon and waits for its socket; give both time to come up.
sleep 20

if ! kill -0 "$APP_PID" 2>/dev/null; then
  cat "$WORK/app.log" >&2
  echo "FAIL: app exited during startup" >&2
  exit 1
fi
if ! pgrep -f "$APP/usr/bin/database" >/dev/null; then
  cat "$WORK/app.log" >&2
  echo "FAIL: daemon was not spawned" >&2
  exit 1
fi
if [ ! -S /tmp/focus_app.sock ]; then
  echo "FAIL: daemon socket /tmp/focus_app.sock is not listening" >&2
  exit 1
fi

echo "PASS: app and daemon running"
