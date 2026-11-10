#!/bin/bash
# Build the Maharaj Downloader browser extension into OUT (as tools/extension/pack.ps1 does for
# the upstream project): chromium/ (Chrome, Edge, Brave, Opera, Vivaldi: Load unpacked),
# maharaj-downloader-chromium.zip, maharaj-downloader-firefox.xpi (unsigned), INSTALL.html
# (the guide the app opens) and README.txt.
#
#   ./pack_extension.sh SRC_ROOT OUT VERSION
set -euo pipefail
SRC=$1; OUT=$2; VERSION=$3
EXTSRC="$SRC/extension"
rm -rf "$OUT"; mkdir -p "$OUT"
python3 /home/claude/work/make_ext_help.py "$SRC" "$EXTSRC/help.html" "$OUT/INSTALL.html" >/dev/null
for kind in chromium firefox; do
  D="$OUT/build-$kind"; mkdir -p "$D"
  for f in background.js video.js page.js popup.html popup.css popup.js help.html; do cp "$EXTSRC/$f" "$D/"; done
  cp -r "$EXTSRC/icons" "$EXTSRC/_locales" "$D/"
  manifest=manifest.json; [ $kind = firefox ] && manifest=manifest.firefox.json
  # Firefox reads Norwegian Bokmål as "nb".
  [ $kind = firefox ] && [ -d "$D/_locales/no" ] && cp -r "$D/_locales/no" "$D/_locales/nb"
  python3 - "$EXTSRC/$manifest" "$D/manifest.json" "$VERSION" <<'PY'
import re, sys
t = open(sys.argv[1], encoding='utf-8').read()
t = re.sub(r'("version"\s*:\s*)"[^"]*"', r'\1"%s"' % sys.argv[3], t, count=1)
open(sys.argv[2], 'w', encoding='utf-8').write(t)
PY
done
mv "$OUT/build-chromium" "$OUT/chromium"
( cd "$OUT/chromium" && zip -qr -X -9 "../maharaj-downloader-chromium.zip" . )
( cd "$OUT/build-firefox" && zip -qr -X -9 "../maharaj-downloader-firefox.xpi" . )
rm -rf "$OUT/build-firefox"
cat > "$OUT/README.txt" <<'EOF'
Maharaj Downloader - browser extension
======================================
Hands the downloads you click in your browser to Maharaj Downloader, puts a "Download video"
button over videos, and adds "Download with Maharaj Downloader" to the right-click menu.
Open INSTALL.html for the full illustrated steps.

Chrome, Edge, Brave, Opera, Vivaldi (offline):
  1. Open the browser's extensions page (chrome://extensions, edge://extensions ...).
  2. Turn on "Developer mode".
  3. Click "Load unpacked" and choose the "chromium" folder next to this file.
  4. Pin "Maharaj Downloader" on the toolbar; its button should say "Connected".

Firefox: maharaj-downloader-firefox.xpi (not signed by Mozilla yet)
  - Firefox Developer Edition / Nightly / ESR: about:config -> xpinstall.signatures.required
    = false, then about:addons -> gear -> Install Add-on From File... -> choose the .xpi.
  - Regular Firefox: about:debugging -> This Firefox -> Load Temporary Add-on... (until
    Firefox closes). To install it for good, sign it once on addons.mozilla.org as a
    self-distributed (unlisted) add-on.

Browser integration must be set up (the setup's "Set up browser integration" box, ticked by
default). The portable copy can't connect to browsers.
Contact: hamvu3ini@gmail.com
EOF
sed -i 's/\r\?$/\r/' "$OUT/README.txt"
echo "extension $VERSION: $(ls "$OUT" | tr '\n' ' ')"
