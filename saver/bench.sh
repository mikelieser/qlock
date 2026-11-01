#!/bin/sh
# Builds the screensaver and runs the offline bench (saver/bench/main.swift): renders frames with
# simulated time via CARenderer, no screen involved. Extra arguments go to the bench, e.g. only=particles.
# Usage: sh saver/bench.sh <out-dir> [key=value …]
set -eu
cd "$(dirname "$0")/.."
out="${1:?out dir}"
shift
sh saver/build.sh >/dev/null
mkdir -p saver/build/bench
swiftc -O -swift-version 5 saver/bench/main.swift -o saver/build/bench/qlock-bench \
  -framework ScreenSaver -framework Metal -framework QuartzCore -framework AppKit
saver/build/bench/qlock-bench saver/build/QLOCK.saver "$out" "$@"
# frames=png plus video=<file.mp4> turns the frames into a clip to watch.
for a in "$@"; do
  case "$a" in
    video=*) fps=$(printf '%s\n' "$@" | sed -n 's/^fps=//p'); fps=${fps:-60}
      ffmpeg -loglevel error -y -framerate "$fps" -i "$out/f%04d.png" -c:v libx264 -pix_fmt yuv420p -crf 18 "${a#video=}"
      echo "video: ${a#video=}" ;;
  esac
done
