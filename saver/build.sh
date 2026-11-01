#!/bin/sh
# Builds saver/build/QLOCK.saver (universal, ad-hoc signed). With --install it also
# copies it to ~/Library/Screen Savers.
set -eu
cd "$(dirname "$0")/.."

OUT=saver/build
BUNDLE="$OUT/QLOCK.saver"
rm -rf "$OUT"
mkdir -p "$BUNDLE/Contents/MacOS" "$BUNDLE/Contents/Resources"

bunx vite build --mode saver --logLevel warn
bun saver/options.ts "$OUT/options.json"

for arch in arm64 x86_64; do
  swiftc -O -swift-version 5 -target "$arch-apple-macos13.0" -module-name Qlock -emit-library \
    -framework ScreenSaver -framework WebKit -framework AppKit \
    saver/Sources/*.swift -o "$OUT/Qlock-$arch"
done
lipo -create "$OUT/Qlock-arm64" "$OUT/Qlock-x86_64" -output "$BUNDLE/Contents/MacOS/Qlock"
rm "$OUT"/Qlock-*

version=$(sed -n 's/.*"version": "\(.*\)".*/\1/p' package.json)
build=$(git rev-list --count HEAD 2>/dev/null || echo 1)
sed -e "s/__VERSION__/$version/" -e "s/__BUILD__/$build/" saver/Info.plist >"$BUNDLE/Contents/Info.plist"
cp -R "$OUT/web" "$OUT/options.json" "$BUNDLE/Contents/Resources/"
[ -f saver/thumbnail.png ] && cp saver/thumbnail*.png "$BUNDLE/Contents/Resources/"

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
