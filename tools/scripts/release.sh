#!/usr/bin/env bash
# Tags the current origin/main commit and pushes the tag, which starts
# .github/workflows/release.yml (tests, AppImage build, GitHub Release).
# Usage: tools/scripts/release.sh v1.2.3
set -euo pipefail

VERSION="${1:?usage: release.sh vMAJOR.MINOR.PATCH}"
fail() { echo "error: $*" >&2; exit 1; }

[[ "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "version must look like v1.2.3"
[ "$(git branch --show-current)" = main ] || fail "switch to main first"
[ -z "$(git status --porcelain)" ] || fail "working tree has uncommitted changes"

git fetch origin main --tags --quiet
[ "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" ] \
  || fail "local main differs from origin/main; push or pull first"
git rev-parse -q --verify "refs/tags/$VERSION" >/dev/null && fail "tag $VERSION already exists locally"
[ -z "$(git ls-remote --tags origin "refs/tags/$VERSION")" ] || fail "tag $VERSION already exists on origin"

read -r -p "Tag origin/main ($(git rev-parse --short HEAD)) as $VERSION and push? [y/N] " answer
[[ "$answer" =~ ^[Yy]$ ]] || { echo "aborted"; exit 1; }

git tag -a "$VERSION" -m "PixRoot $VERSION"
git push origin "$VERSION"

echo "Pushed $VERSION. The release build is running; watch it with: gh run watch"
