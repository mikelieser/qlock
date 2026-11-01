#!/bin/sh
# Renders themes in the web app (headless Chrome) and natively, side by side, for checking ports.
# Needs `bunx vite preview --port 4173` running. Usage: native/tools/compare.sh <out-dir> [theme …] [-- flags]
set -eu
cd "$(dirname "$0")/../.."
out="$1"; shift
mkdir -p "$out"
themes=""
flags="all date"
query="mode=all&date=1&effect=0&transition=0&ambient=0&progress=0&stretch=0&dots=1&intro=1"
while [ $# -gt 0 ]; do
  case "$1" in
    --) shift; flags="$*"; query="$(printf '%s' "$*" | sed 's/ /=1\&/g')=1&mode=all"; break ;;
    *) themes="$themes $1"; shift ;;
  esac
done
[ -n "$themes" ] || themes=$(python3 -c "import json;print(' '.join(json.load(open('native/data/themes.json'))['themes']))")
chrome="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
profile="$out/.chrome"
for t in $themes; do
  "$chrome" --headless=new --disable-gpu --hide-scrollbars --user-data-dir="$profile" --force-device-scale-factor=1 \
    --window-size=1512,982 --virtual-time-budget=20000 --run-all-compositor-stages-before-draw --screenshot="$out/web-$t.png" \
    "http://localhost:4173/?theme=$t&locale=de&$query" >/dev/null 2>&1
  native/target/release/qlock-preview de "$t" 15:15 8 1512 982 "$out/native-$t.png" $flags 2>/dev/null
  python3 -c "
from PIL import Image
a=Image.open('$out/web-$t.png').convert('RGB').resize((756,491)); b=Image.open('$out/native-$t.png').convert('RGB').resize((756,491))
s=Image.new('RGB',(1512,491)); s.paste(a,(0,0)); s.paste(b,(756,0)); s.save('$out/cmp-$t.png')"
done
echo "$out"
