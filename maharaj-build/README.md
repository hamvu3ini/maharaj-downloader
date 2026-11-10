# Building Maharaj Downloader

Maharaj Downloader is a Rust workspace (the upstream project's, see `../UPSTREAM-README.md`) plus the
changes listed in `../MAHARAJ-CHANGES.md`. These are the scripts the releases were made with.

## On Windows (simplest)

Needs the Rust toolchain (`rustup`, stable 1.95 or later, MSVC) and, for the setup program,
[Inno Setup 6](https://jrsoftware.org/isinfo.php).

```
cargo build --release -p mdm-app -p mdm-nmhost -p mdm-cli -p mdm-mcp
cargo test -p mdm-app -p mdm-ipc -p mdm-nmhost
```

The programs are `target\release\MaharajDownloader.exe`, `MaharajDownloader-nmhost.exe`,
`MaharajDownloader-cli.exe` and `MaharajDownloader-mcp.exe`. The setup script is
`tools\installer\maharaj.iss` (`ISCC /DAppVersion=X.Y.Z /DPayload=<folder> maharaj.iss`, the payload
folder laid out as `package.sh` makes it: the programs, `helpers\`, `extension\`, `LICENSES\`,
`TERMS-OF-USE.txt`).

## Cross-built on Linux (how the releases are made)

The scripts here cross-compile for Windows with MinGW and run the Windows programs under Wine for the
tests. They expect a working folder `/home/claude/work` with the source in `src/`; change the paths at
their top for another place.

| Script | What it does |
|---|---|
| `rebrand.py` | Turns a fresh upstream checkout into Maharaj Downloader's names (run once) |
| `build-win.sh` | Builds the four Windows programs (`x86_64-pc-windows-gnu`, std built from source; copy `cargo-config-cross.toml` to `.cargo/config.toml` first) |
| `build-tests.sh` | Builds the unit tests for Windows (run them under Wine) |
| `pack_extension.sh`, `make_ext_help.py` | Build the browser extension (Chromium folder and zip, Firefox .xpi, its offline help and INSTALL.html) |
| `package.sh` | Makes `Maharaj_Downloader_vX.Y.Z.zip`: setup, helpers, extension, licences, source, checksums |
| `tests/engine_tests.sh` | Download engine tests against the lab server (faults, FTP, resume, errors), SHA-256 checked |
| `tests/extension_e2e.mjs` | The extension in real Chrome, through the real browser host, to the running app |

Helpers shipped in releases (not in this repository): yt-dlp, FFmpeg (BtbN LGPL shared build),
gallery-dl and 7-Zip, each checked against its publisher's checksum or signature, with
`helpers/versions.json` naming their versions.

## A new version

Raise `version` in `../Cargo.toml` (1.0.1 → 1.0.2 …), build, and run `package.sh`: every file name
follows the new number.

The browser extension's signing key (`key.pem`, which fixes its Chrome ID
`hpmflmangdjofkncmfkchhahehlcjbho`) is kept private by the author and is not in this repository; the
public half is in `extension/manifest.json`.
