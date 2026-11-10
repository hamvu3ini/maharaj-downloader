#!/bin/bash
# Package Maharaj Downloader for Windows: one zip, named with the version, holding the setup
# program, a portable copy, the helpers, the licences and the source. Everything works offline.
#
#   ./package.sh            build the zip for the version in src/Cargo.toml
#
# For the next release, raise `version` in src/Cargo.toml (1.0.0 -> 1.0.1 ...), rebuild with
# ./build-win.sh and run this again: every file name follows the new number.
set -euo pipefail
W=/home/claude/work
SRC=$W/src
REL=$W/target/x86_64-pc-windows-gnu/release
VERSION=$(grep -m1 '^version' "$SRC/Cargo.toml" | cut -d'"' -f2)
NAME="Maharaj_Downloader_v$VERSION"
DIST=$W/dist
OUTDIR=$DIST/$NAME
PAY=$DIST/payload
source $W/wt.sh

echo "== Maharaj Downloader $VERSION"
rm -rf "$DIST"; mkdir -p "$PAY" "$OUTDIR"

# 1. The programs (debug info stripped).
for e in MaharajDownloader MaharajDownloader-nmhost MaharajDownloader-cli MaharajDownloader-mcp; do
  x86_64-w64-mingw32-strip -o "$PAY/$e.exe" "$REL/$e.exe"
done

# 2. The helpers: yt-dlp, FFmpeg, gallery-dl, 7-Zip (checked against their publishers' sums).
cp -r $W/helpers "$PAY/helpers"

# 3. The browser extension (Chrome, Edge, Brave, Opera, Vivaldi; Firefox), with its offline
#    help and the install guide the app opens (extension\INSTALL.html).
$W/pack_extension.sh "$SRC" "$PAY/extension" "$VERSION"

# 4. Terms, licences, credits.
cp "$SRC/crates/app/assets/brand/terms.txt" "$PAY/TERMS-OF-USE.txt"
cp "$SRC/LICENSE" "$PAY/LICENSE.txt"
L="$PAY/LICENSES"; mkdir -p "$L"
cp "$SRC/LICENSE" "$L/GPL-3.0 (Maharaj Downloader and Mister Download Manager).txt"
cp "$SRC/ATTRIBUTION.md" "$L/ATTRIBUTION (icons, data and libraries).txt"
cp $W/dl/yt-dlp-LICENSE.txt "$L/yt-dlp (Unlicense).txt"
cp $W/helpers/ffmpeg/LICENSE.txt "$L/FFmpeg (LGPL-2.1-or-later).txt"
cp $W/dl/gallery-dl-LICENSE.txt "$L/gallery-dl (GPL-2.0).txt"
cp $W/helpers/7-Zip/License.txt "$L/7-Zip (LGPL + unRAR restriction).txt"
( cd "$SRC" && cargo metadata --format-version 1 --filter-platform x86_64-pc-windows-gnu 2>/dev/null ) | python3 -c '
import json, sys
m = json.load(sys.stdin)
rows = sorted({(p["name"], p["version"], p.get("license") or "see repository", p.get("repository") or "") for p in m["packages"]})
print("Rust libraries built into Maharaj Downloader\n" + "=" * 44 + "\n")
print("Each is used under its licence; their source code is at the address given.\n")
for n, v, l, r in rows:
    print(f"{n} {v}\n    licence: {l}\n    source:  {r}\n")
' > "$L/Rust libraries.txt"
cat > "$L/README.txt" <<EOF
Maharaj Downloader $VERSION - licences and credits
=============================================

Maharaj Downloader (C) 2026 Sasanka Boruah (VU3INI) <hamvu3ini@gmail.com>
is a modified version of Mister Download Manager 0.5.2 (C) 2026 Ahmed Nefzaoui,
https://github.com/anefzaoui/misterdownloadmanager

Both are free software: you can redistribute them and/or modify them under the terms of the
GNU General Public License as published by the Free Software Foundation, either version 3 of
the License, or (at your option) any later version (see "GPL-3.0 ...txt"). They come with
ABSOLUTELY NO WARRANTY.

The complete source code of this version is in the SOURCE folder of the distribution zip
(maharaj-downloader-$VERSION-source.zip), with the list of changes made to Mister Download
Manager (MAHARAJ-CHANGES.txt).

Included programs (in the "helpers" folder), each under its own licence:
  yt-dlp $(python3 -c "import json;print(json.load(open('$W/helpers/versions.json'))['yt-dlp'])")   Unlicense      https://github.com/yt-dlp/yt-dlp
  FFmpeg n8.1.3 (LGPL build by BtbN) LGPL-2.1+  https://ffmpeg.org  source: https://github.com/FFmpeg/FFmpeg/tree/n8.1.3
                                                build scripts: https://github.com/BtbN/FFmpeg-Builds
  gallery-dl 1.31.10                GPL-2.0        https://github.com/mikf/gallery-dl (source: tag v1.31.10)
  7-Zip 26.01                       LGPL-2.1 + unRAR restriction   https://www.7-zip.org
A copy of the source code of FFmpeg and gallery-dl as distributed here can also be requested
from hamvu3ini@gmail.com for three years from this release.

Names and logos: "Maharaj Downloader", the VU3INI callsign and the VU3INI logo identify this
build and its author; they are not licensed under the GPL. If you distribute a modified version,
please give it another name and remove the logo.
EOF

# 5. A short read-me beside the program.
cat > "$PAY/README.txt" <<EOF
Maharaj Downloader $VERSION
==========================
A free download manager for personal use, by Sasanka Boruah (VU3INI) - hamvu3ini@gmail.com

Start:      MaharajDownloader.exe (accept the Terms of Use when it starts)
Help:       F1 in the program, or Help -> Help contents (searchable, works offline)
Theme:      the "Dark theme" / "Light theme" button at the bottom right, or the View menu
Videos/MP3: Tasks -> Download video from a page...
Licences:   the LICENSES folder
EOF

# 7. The setup's Portable option puts this read-me in the program's `data` folder.
cat > "$DIST/portable-data-README.txt" <<'TXT'
This folder makes Maharaj Downloader portable: the download list and settings are kept here
instead of in your Windows profile, and nothing is written to the registry. Copy the whole
Maharaj Downloader folder to a USB stick or anywhere else and run MaharajDownloader.exe.
Delete this "data" folder to keep the settings in your profile instead.
TXT
sed -i 's/\r\?$/\r/' "$DIST/portable-data-README.txt"

# 6. The setup program (Inno Setup, run under Wine), with wizard pictures from the logo.
python3 - "$SRC/crates/app/assets/brand/logo-circle-256.png" "$DIST" <<'PY'
import sys
from PIL import Image, ImageDraw, ImageFont
logo = Image.open(sys.argv[1]).convert('RGBA')
def card(w, h, size, out, text):
    im = Image.new('RGB', (w, h), (255, 255, 255))
    if text:
        im = Image.new('RGB', (w, h), (28, 92, 168))
    l = logo.resize((size, size), Image.LANCZOS)
    im.paste(l, ((w - size) // 2, (h - size) // 2 - (40 if text else 0)), l)
    if text:
        d = ImageDraw.Draw(im)
        try:
            f1 = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf', 17)
            f2 = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 11)
        except OSError:
            f1 = f2 = ImageFont.load_default()
        for y, s, f in [(h // 2 + 75, 'Maharaj', f1), (h // 2 + 97, 'Downloader', f1), (h // 2 + 125, 'by Sasanka Boruah', f2), (h // 2 + 141, 'VU3INI', f2)]:
            tw = d.textlength(s, font=f)
            d.text(((w - tw) / 2, y), s, fill=(255, 255, 255), font=f)
    im.save(out)
card(55, 55, 53, sys.argv[2] + '/wizard-small.bmp', False)
card(164, 314, 140, sys.argv[2] + '/wizard-large.bmp', True)
PY
ISS_DIR="$SRC/tools/installer"
WPAY=$(winepath -w "$PAY" 2>/dev/null)
( cd "$ISS_DIR" && timeout 900 wine 'C:\InnoSetup\ISCC.exe' /Qp "/DAppVersion=$VERSION" "/DPayload=$WPAY" maharaj.iss ) > $DIST/iscc.log 2>&1 \
  || { tail -20 $DIST/iscc.log; exit 1; }
mv "$DIST/Maharaj_Downloader_Setup_v$VERSION.exe" "$OUTDIR/"

# 8. The source code (GPL): the modified tree, the rebranding script and the build scripts.
mkdir -p "$OUTDIR/SOURCE"
SRCZIP="$OUTDIR/SOURCE/maharaj-downloader-$VERSION-source.zip"
STAGE=$DIST/srcstage/maharaj-downloader-$VERSION
mkdir -p "$STAGE"
rsync -a --exclude .git --exclude target --exclude website --exclude .claude --exclude 'notes' "$SRC/" "$STAGE/"
mkdir -p "$STAGE/maharaj-build"
cp $W/rebrand.py $W/build-win.sh $W/package.sh $W/wt.sh $W/pack_extension.sh $W/make_ext_help.py "$STAGE/maharaj-build/"
cp -r $W/tests "$STAGE/maharaj-build/tests"
cp $W/MAHARAJ-CHANGES.txt "$STAGE/MAHARAJ-CHANGES.txt"
cp $W/MAHARAJ-CHANGES.txt "$OUTDIR/SOURCE/MAHARAJ-CHANGES.txt"
( cd $DIST/srcstage && zip -qr -9 "$SRCZIP" "maharaj-downloader-$VERSION" )
rm -rf $DIST/srcstage

# 9. Read me first, the terms, checksums, and the zip.
cp "$PAY/TERMS-OF-USE.txt" "$OUTDIR/TERMS-OF-USE.txt"
cp $W/READ-ME-FIRST.txt "$OUTDIR/READ-ME-FIRST.txt"
sed -i "s/{VERSION}/$VERSION/g" "$OUTDIR/READ-ME-FIRST.txt"
# Windows line ends for the text files people open in Notepad (before the checksums).
find "$OUTDIR" -maxdepth 3 -name '*.txt' -exec sed -i 's/\r\?$/\r/' {} +
( cd "$OUTDIR" && find . -type f ! -name SHA256SUMS.txt -print0 | sort -z | xargs -0 sha256sum | sed 's#  \./#  #' > SHA256SUMS.txt )
( cd "$DIST" && rm -f "$NAME.zip" && zip -qr -9 "$NAME.zip" "$NAME" )
ls -la "$DIST/$NAME.zip"
echo "== done: $DIST/$NAME.zip"
