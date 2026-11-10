# Test helpers for the Windows build under Wine (source this file).
export WINEPREFIX=/home/claude/wine WINEDEBUG=-all DISPLAY=:99
REL=/home/claude/work/target/x86_64-pc-windows-gnu/release
stage_run() { # stage_run DIR : fresh portable copy of the programs in DIR
  rm -rf "$1"; mkdir -p "$1/data"
  for e in MaharajDownloader MaharajDownloader-nmhost MaharajDownloader-cli MaharajDownloader-mcp; do
    x86_64-w64-mingw32-strip -o "$1/$e.exe" "$REL/$e.exe"; done
  [ -d /home/claude/work/helpers ] && cp -r /home/claude/work/helpers "$1/"
}
win() { xdotool search --name "$1" 2>/dev/null | head -1; }
wait_win() { for i in $(seq 1 ${2:-60}); do w=$(win "$1"); [ -n "$w" ] && { echo $w; return; }; sleep 1; done; }
geom() { xdotool getwindowgeometry --shell "$1"; }
click_in() { # click_in WIN X Y (client coords)
  eval $(geom "$1"); xdotool mousemove $((X+$2)) $((Y+$3)) click 1; }
shot() { import -window root "$1"; }
shot_win() { eval $(geom "$1"); import -window root -crop ${WIDTH}x${HEIGHT}+${X}+${Y} "$2"; }
kill_app() { wineserver -k 2>/dev/null; sleep 1; }
