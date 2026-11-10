#!/bin/bash
# Functional tests of Maharaj Downloader's download engine: the Windows program (CLI, under
unset RUST_BACKTRACE
# Wine) against the project's lab server with fault injection. Every file is checked byte for
# byte (SHA-256) against the same deterministic content fetched directly.
source /home/claude/work/wt.sh
APP=${APP:-/home/claude/work/t2}
OUT=$APP/engine-out
LAB=http://127.0.0.1:8090
rm -rf "$OUT"; mkdir -p "$OUT"
cd "$APP"
pass=0; fail=0
RESULTS=/home/claude/work/tests/engine_results.txt
: > "$RESULTS"
winpath() { echo "Z:${1//\//\\}"; }

check() { # check NAME URL EXPECTED_URL FILE_NAME [extra cli args...]
  local name="$1" url="$2" ref="$3" file="$4"; shift 4
  local want got t0 t1 dt
  want=$(curl -s "$ref" | sha256sum | cut -c1-64)
  t0=$(date +%s.%N)
  timeout 300 wine MaharajDownloader-cli.exe get "$url" -o "$(winpath "$OUT")" "$@" > "$OUT/$name.log" 2>&1
  local rc=$?
  t1=$(date +%s.%N); dt=$(printf '%.1f' "$(echo "$t1 - $t0" | bc)")
  got=$(sha256sum "$OUT/$file" 2>/dev/null | cut -c1-64)
  if [ "$rc" = 0 ] && [ -n "$want" ] && [ "$want" = "$got" ]; then
    pass=$((pass+1)); printf 'PASS  %-46s %6ss  sha256 %s…\n' "$name" "$dt" "${got:0:16}" | tee -a "$RESULTS"
  else
    fail=$((fail+1)); printf 'FAIL  %-46s rc=%s want=%s got=%s\n' "$name" "$rc" "${want:0:16}" "${got:0:16}" | tee -a "$RESULTS"
    tail -3 "$OUT/$name.log" | tee -a "$RESULTS"
  fi
}

check "64 MB, fast, several connections"        "$LAB/gen/64M/fast.bin?seed=11"                         "$LAB/gen/64M/fast.bin?seed=11"   fast.bin
check "32 MB, every 2nd connection cut at 2 MB" "$LAB/gen/32M/drops.bin?seed=12&drop_every=2&drop_after=2M&rate=8M" "$LAB/gen/32M/drops.bin?seed=12" drops.bin
check "16 MB, one connection in 3 crawls"       "$LAB/gen/16M/slow.bin?seed=13&slow_every=3&slow_rate=128K" "$LAB/gen/16M/slow.bin?seed=13" slow.bin
check "16 MB, server allows 2 connections"      "$LAB/gen/16M/two.bin?seed=14&max_conns=2&rate=4M"  "$LAB/gen/16M/two.bin?seed=14"    two.bin
check "8 MB, first 2 requests fail (retry)"     "$LAB/gen/8M/retry.bin?seed=15&fail_first=2"        "$LAB/gen/8M/retry.bin?seed=15"   retry.bin
check "8 MB, server without Range support"      "$LAB/gen/8M/norange.bin?seed=16&no_range=1"        "$LAB/gen/8M/norange.bin?seed=16" norange.bin
check "8 MB, 150 ms latency"                    "$LAB/gen/8M/latency.bin?seed=17&latency_ms=150"    "$LAB/gen/8M/latency.bin?seed=17" latency.bin
check "Name from Content-Disposition"           "$LAB/gen/2M/x?seed=18&cd=Maharaj%20Report%202026.pdf" "$LAB/gen/2M/x?seed=18" "Maharaj Report 2026.pdf"
check "Redirect to the file"                    "$LAB/redirect?to=%2Fgen%2F4M%2Fredir.bin%3Fseed%3D19" "$LAB/gen/4M/redir.bin?seed=19" redir.bin
check "A real file (7z archive)"                "$LAB/files/arc.7z"                                 "$LAB/files/arc.7z"               arc.7z
check "FTP, 8 MB"                               "ftp://127.0.0.1:2121/gen/8M/ftp.bin"               "$LAB/gen/8M/ftp.bin"             ftp.bin
check "One connection only (-c 1)"              "$LAB/gen/6M/single.bin?seed=20"                    "$LAB/gen/6M/single.bin?seed=20"  single.bin -c 1
check "Checksum verified (--checksum sha256)"   "$LAB/gen/3M/sum.bin?seed=22"                       "$LAB/gen/3M/sum.bin?seed=22"     sum.bin --checksum "sha256:$(curl -s "$LAB/gen/3M/sum.bin?seed=22" | sha256sum | cut -c1-64)"

# Resume: stop a slow download half way (Ctrl+C pauses it), then `resume` it from its state file.
name="Pause half way, then resume"
want=$(curl -s "$LAB/gen/12M/resume.bin?seed=21" | sha256sum | cut -c1-64)
timeout -s INT 5 wine MaharajDownloader-cli.exe get "$LAB/gen/12M/resume.bin?seed=21&rate=128K" -o "$(winpath "$OUT")" > "$OUT/resume1.log" 2>&1
wineserver -k 2>/dev/null; sleep 1
part=$(ls "$OUT"/resume.bin*.mdm.json 2>/dev/null | head -1)
partial=$(stat -c %s "$OUT"/resume.bin*.mdm.part 2>/dev/null | head -1)
if [ -n "$part" ]; then
  timeout 300 wine MaharajDownloader-cli.exe resume "$(winpath "$part")" > "$OUT/resume2.log" 2>&1
fi
got=$(sha256sum "$OUT/resume.bin" 2>/dev/null | cut -c1-64)
if [ -n "$part" ] && [ "$want" = "$got" ]; then
  pass=$((pass+1)); printf 'PASS  %-46s state file kept, finished on resume\n' "$name" | tee -a "$RESULTS"
else
  fail=$((fail+1)); printf 'FAIL  %-46s state=%s partial=%s got=%s\n' "$name" "$part" "$partial" "${got:0:16}" | tee -a "$RESULTS"
  tail -3 "$OUT/resume1.log" "$OUT/resume2.log" 2>/dev/null | tee -a "$RESULTS"
fi

# Errors are reported, not crashes.
for case in "Wrong checksum is refused|$LAB/gen/1M/bad.bin?seed=23 --checksum sha256:0000000000000000000000000000000000000000000000000000000000000000" "404 Not Found|$LAB/nope/missing.bin" "Refused connection|http://127.0.0.1:1/x.bin" "Unknown host|http://no-such-host.invalid/x.bin"; do
  name="Error: ${case%%|*}"; url="${case#*|}"
  timeout 120 wine MaharajDownloader-cli.exe get $url -o "$(winpath "$OUT")" > "$OUT/err.log" 2>&1; rc=$?
  if [ $rc -ne 0 ] && [ $rc -lt 100 ] && [ -s "$OUT/err.log" ]; then
    pass=$((pass+1)); printf 'PASS  %-46s exit %s: %s\n' "$name" "$rc" "$(tail -1 "$OUT/err.log" | cut -c1-70)" | tee -a "$RESULTS"
  else
    fail=$((fail+1)); printf 'FAIL  %-46s exit %s\n' "$name" "$rc" | tee -a "$RESULTS"; tail -2 "$OUT/err.log" | tee -a "$RESULTS"
  fi
done

echo "engine: $pass passed, $fail failed" | tee -a "$RESULTS"
