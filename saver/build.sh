#!/bin/sh
# Builds saver/build/QLOCK.saver: Swift shell + the Rust engine from native/ (Apple Silicon, ad-hoc
# signed). With --install it also copies it to ~/Library/Screen Savers.
set -eu
cd "$(dirname "$0")/.."

OUT=saver/build
BUNDLE="$OUT/QLOCK.saver"
rm -rf "$OUT"
mkdir -p "$BUNDLE/Contents/MacOS" "$BUNDLE/Contents/Resources"

bun native/export-locales.ts >/dev/null
bun saver/options.ts "$OUT/options.json"
cargo build --release --quiet --manifest-path native/Cargo.toml -p qlock-ffi

swiftc -O -swift-version 5 -target arm64-apple-macos13.0 -module-name Qlock -emit-library \
  -import-objc-header native/ffi/qlock.h -Lnative/target/release -lqlock_ffi \
  -framework ScreenSaver -framework QuartzCore -framework AppKit \
  saver/Sources/*.swift -o "$BUNDLE/Contents/MacOS/Qlock"

version=$(sed -n 's/.*"version": "\(.*\)".*/\1/p' package.json)
build=$(git rev-list --count HEAD 2>/dev/null || echo 1)
sed -e "s/__VERSION__/$version/" -e "s/__BUILD__/$build/" saver/Info.plist >"$BUNDLE/Contents/Info.plist"
cp "$OUT/options.json" "$BUNDLE/Contents/Resources/"
if [ -f saver/thumbnail.png ]; then cp saver/thumbnail*.png "$BUNDLE/Contents/Resources/"; fi

codesign --force --sign - "$BUNDLE"
echo "Gebaut: $BUNDLE"

if [ "${1:-}" = "--install" ]; then
  target="$HOME/Library/Screen Savers"
  mkdir -p "$target"
  rm -rf "$target/QLOCK.saver"
  cp -R "$BUNDLE" "$target/"
  # The settings host caches loaded savers; restart it so the new build shows up.
  killall legacyScreenSaver "System Settings" 2>/dev/null || true
  echo "Installiert: $target/QLOCK.saver"
fi
