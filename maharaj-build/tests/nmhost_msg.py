#!/usr/bin/env python3
"""Talk to MaharajDownloader-nmhost.exe the way Chrome does: a 4-byte length, then JSON, on
stdin; the reply the same way on stdout. Usage: nmhost_msg.py HOST_EXE '{"hello": true}'"""
import json, struct, subprocess, sys

host, msg = sys.argv[1], sys.argv[2]
data = json.dumps(json.loads(msg)).encode()
p = subprocess.run(
    ["wine", host, "chrome-extension://ejoleppdfgjdndddcjoehopknmlglojp/"],
    input=struct.pack("<I", len(data)) + data,
    capture_output=True, timeout=120,
)
out = p.stdout
if len(out) >= 4:
    n = struct.unpack("<I", out[:4])[0]
    print(out[4:4 + n].decode(errors="replace"))
else:
    print("no reply; exit", p.returncode, p.stderr.decode(errors="replace")[-300:])
