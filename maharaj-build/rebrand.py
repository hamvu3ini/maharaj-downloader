#!/usr/bin/env python3
"""Rebrand the Mister Download Manager source tree as "Maharaj Downloader".

Run once on a fresh copy of the upstream source (GPL-3.0-or-later). Every change is a plain,
reviewable text substitution; the upstream copyright and licence notices are kept.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
NAME = "Maharaj Downloader"
changed = []


def edit(path, fn):
    p = ROOT / path if not isinstance(path, pathlib.Path) else path
    old = p.read_text(encoding="utf-8")
    new = fn(old)
    if new != old:
        p.write_text(new, encoding="utf-8")
        changed.append(str(p.relative_to(ROOT)))


def sub(path, pairs):
    def fn(t):
        for a, b in pairs:
            if a not in t:
                raise SystemExit(f"{path}: expected text not found: {a!r}")
            t = t.replace(a, b)
        return t
    edit(path, fn)


# 1. Program files: MaharajDownloader.exe, -nmhost, -cli, -mcp.
for crate, old, new in [
    ("app", 'name = "misterdownloadmanager"', 'name = "MaharajDownloader"'),
    ("cli", 'name = "misterdownloadmanager-cli"', 'name = "MaharajDownloader-cli"'),
    ("mcp", 'name = "misterdownloadmanager-mcp"', 'name = "MaharajDownloader-mcp"'),
    ("nmhost", 'name = "misterdownloadmanager-nmhost"', 'name = "MaharajDownloader-nmhost"'),
]:
    sub(f"crates/{crate}/Cargo.toml", [(old, new)])
sub("crates/app/Cargo.toml", [('"Mister Download Manager, desktop app (egui)"',
                               '"Maharaj Downloader, desktop app (egui)"')])
sub("crates/cli/src/main.rs", [('format!("misterdownloadmanager{}"', 'format!("MaharajDownloader{}"')])
sub("crates/cli/src/lib.rs", [('pub const NAME: &str = "misterdownloadmanager-cli";',
                               'pub const NAME: &str = "MaharajDownloader-cli";')])
sub("crates/nmhost/src/main.rs", [
    ('"misterdownloadmanager.exe"', '"MaharajDownloader.exe"'),
    ('            "misterdownloadmanager"\n', '            "MaharajDownloader"\n'),
    ('eprintln!("misterdownloadmanager-nmhost: {e}")', 'eprintln!("MaharajDownloader-nmhost: {e}")'),
])
sub("crates/app/src/extension.rs", [('"misterdownloadmanager-nmhost.exe"', '"MaharajDownloader-nmhost.exe"')])
sub("crates/app/src/unix.rs", [('const HOST: &str = "misterdownloadmanager-nmhost";',
                                'const HOST: &str = "MaharajDownloader-nmhost";')])
sub("crates/mcp/src/lib.rs", [('            "misterdownloadmanager",\n', '            "maharajdownloader",\n')])
sub("crates/mcp/src/main.rs", [('"misterdownloadmanager-mcp {}"', '"MaharajDownloader-mcp {}"')])
# Tests that name the programs.
sub("crates/cli/tests/launcher.rs", [
    ('env!("CARGO_BIN_EXE_misterdownloadmanager-cli")', 'env!("CARGO_BIN_EXE_MaharajDownloader-cli")'),
    ('format!("misterdownloadmanager-cli{exe}")', 'format!("MaharajDownloader-cli{exe}")'),
    ('format!("misterdownloadmanager{exe}")', 'format!("MaharajDownloader{exe}")'),
])
sub("crates/mcp/tests/stdio.rs", [
    ('env!("CARGO_BIN_EXE_misterdownloadmanager-mcp")', 'env!("CARGO_BIN_EXE_MaharajDownloader-mcp")'),
    ('assert_eq!(init["serverInfo"]["name"], "misterdownloadmanager");',
     'assert_eq!(init["serverInfo"]["name"], "maharajdownloader");'),
])
sub("crates/app/tests/startup_tray.rs", [
    ('env!("CARGO_BIN_EXE_misterdownloadmanager")', 'env!("CARGO_BIN_EXE_MaharajDownloader")')])

# 2. Its own data folder, registry keys, file type and single-instance mutex, so it never mixes
#    with an installed Mister Download Manager. (The browsers' native host name stays
#    com.misterdownloadmanager.host: the browser extension calls the host by that name.)
sub("crates/ipc/src/lib.rs", [('let dir = base.join("misterdownloadmanager");',
                               'let dir = base.join("maharajdownloader");')])
sub("crates/app/src/win32.rs", [
    (r'const KEY: &str = r"Software\MisterDownloadManager";', r'const KEY: &str = r"Software\MaharajDownloader";'),
    ('const PROGID: &str = "MisterDownloadManager.Torrent";', 'const PROGID: &str = "MaharajDownloader.Torrent";'),
    ('    Some("MisterDownloadManager".to_owned())', '    Some("MaharajDownloader".to_owned())'),
    ('"MisterDownloadManager.Running\\0"', '"MaharajDownloader.Running\\0"'),
])
sub("crates/app/src/main.rs", [
    ('pub const MAIN_TITLE: &str = concat!("Mister Download Manager ", env!("CARGO_PKG_VERSION"));',
     'pub const MAIN_TITLE: &str = concat!("Maharaj Downloader ", env!("CARGO_PKG_VERSION"));'),
    ('renderer::run("misterdownloadmanager", &plan', 'renderer::run("maharajdownloader", &plan'),
])

# 3. The name in every text: source, the 86 translation catalogs, the user guide, the extension.
def rename_text(t):
    t = t.replace("Mister Download Manager", NAME)
    return t

texts = [p for p in (ROOT / "crates").rglob("*.rs")]
texts += list((ROOT / "docs").rglob("*.md"))
texts += list((ROOT / "extension").rglob("*.json"))
texts += [p for p in (ROOT / "extension").glob("*.*") if p.suffix in (".js", ".html")]
for p in texts:
    edit(p, rename_text)

# Catalogs: the name (both columns, so lookups still match) and "MDM" in the shown translation.
MDM = re.compile(r"(?<![A-Za-z0-9_])MDM(?![A-Za-z0-9_])")
for p in sorted((ROOT / "i18n").glob("*.tsv")):
    def fn(t):
        out = []
        for line in t.split("\n"):
            line = line.replace("Mister Download Manager", NAME)
            if line and not line.startswith("#") and "\t" in line:
                head, _, last = line.rpartition("\t")
                line = head + "\t" + MDM.sub(NAME, last)
            out.append(line)
        return "\n".join(out)
    edit(p, fn)

# The user guide (shown offline in Help): "MDM" read as the app's name.
for p in (ROOT / "docs").rglob("*.md"):
    edit(p, lambda t: MDM.sub(NAME, t))

print(f"{len(changed)} files changed")

# 5. Follow-ups: "Mister Download Manager (MDM)" read twice after both renames; the program
#    names in the guide.
for p in list((ROOT / "docs").rglob("*.md")) + list((ROOT / "i18n").glob("*.tsv")) + list((ROOT / "crates").rglob("*.rs")):
    edit(p, lambda t: t.replace("Maharaj Downloader (Maharaj Downloader)", NAME))
for p in (ROOT / "docs").rglob("*.md"):
    edit(p, lambda t: t.replace("misterdownloadmanager-cli", "MaharajDownloader-cli")
         .replace("misterdownloadmanager-nmhost", "MaharajDownloader-nmhost")
         .replace("misterdownloadmanager-mcp", "MaharajDownloader-mcp")
         .replace("misterdownloadmanager.exe", "MaharajDownloader.exe"))
print(f"{len(changed)} files changed in all")

# 6. The remote control's page (shown on a paired phone) and a test's path.
for p in (ROOT / "crates/remote/web").glob("*"):
    if p.is_file():
        # The full name only: the page's T('...') keys stay the catalogs' English (their
        # translations already read the new name).
        edit(p, lambda t: t.replace("Mister Download Manager", NAME))
edit(ROOT / "crates/app/src/win32.rs", lambda t: t.replace(
    r'r"c:\program files\mister download manager\MISTERDOWNLOADMANAGER.EXE"',
    r'r"c:\program files\maharaj downloader\MISTERDOWNLOADMANAGER.EXE"'))
print(f"{len(changed)} files changed in all")

# 7. Every test that runs a program by its cargo name.
for p in (ROOT / "crates").rglob("*.rs"):
    edit(p, lambda t: t.replace("CARGO_BIN_EXE_misterdownloadmanager-", "CARGO_BIN_EXE_MaharajDownloader-")
         .replace('CARGO_BIN_EXE_misterdownloadmanager"', 'CARGO_BIN_EXE_MaharajDownloader"'))
