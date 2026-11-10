Maharaj Downloader - changes made to Mister Download Manager 0.5.2
==================================================================
Modified by Sasanka Boruah (VU3INI) <hamvu3ini@gmail.com>, November 2026.
Upstream: https://github.com/anefzaoui/misterdownloadmanager (commit cd1b2b5), GPL-3.0-or-later.

Name and identity (maharaj-build/rebrand.py, applied to the upstream tree)
- The name "Mister Download Manager" / "MDM" in the interface, the 86 translation catalogs,
  the installer's translations and the user guide reads "Maharaj Downloader".
- Programs: MaharajDownloader.exe, MaharajDownloader-cli.exe, -mcp.exe, -nmhost.exe.
- Its own data folder (%APPDATA%\maharajdownloader), registry keys, .torrent file type and
  single-instance mutex, so it never mixes with an installed Mister Download Manager.
  The browsers' native messaging host keeps its name (com.misterdownloadmanager.host) so the
  browser extension can reach it.
- New icon and logo (the author's VU3INI caricature), version 1.0.0, version information.

New (crates/app/src/maharaj.rs, crates/app/assets/brand/)
- A start-up splash, 4 x 3 inches: the logo in a circle, the name, the author's credentials and
  the Terms of Use in a scrolling box. Nothing of the app runs until "I Agree (OK)" is clicked;
  "Exit" quits; it can't be closed otherwise. Each acceptance is noted in terms-accepted.log
  in the data folder. Help -> Terms of Use shows the terms again.
- The footer "By Sasanka Boruah -VU3INI" at the bottom left of the main window, and at the
  bottom right a Help button and a one-click Light/Dark theme switch.
- View menu: Light theme / Dark theme / Follow Windows theme.
- Help: the whole user guide and Maharaj Downloader's own pages built into the program,
  searchable (every page, words highlighted), with contents, links and Back; offline. F1,
  the footer's Help and the Help menu open it; the guide's links in dialogs open it too.
- Help -> About: logo, version, credits, contact hamvu3ini@gmail.com, licences, source.
- Help -> Report a problem and the crash report window write an email to the author instead of
  opening an issue in the upstream project's tracker.

Torrent search (crates/app/src/torrentsearch.rs)
- Tasks -> Search torrents..., the Add Torrent button's menu and the footer open it: one query to
  every chosen source, the results in one list (sort, filter), Download hands the magnet or the
  .torrent to Add torrent. Built-in source: Internet Archive (public domain / openly licensed,
  each item's <id>_archive.torrent). The user can add any RSS search or Torznab indexer
  (Jackett, Prowlarr) by an address with {query}; kept in torrent-search.json. No site-specific
  code is shipped. The Terms of Use say who is responsible for added sources.

Browser extension (extension/, 1.0.1)
- Its own identity: a new key (Chrome ID hpmflmangdjofkncmfkchhahehlcjbho) and Firefox ID
  maharaj-downloader@vu3ini; the browser host serves only these (crates/ipc/src/extension.rs);
  no store listing, so the app's "Show me how" opens the guide shipped beside the program
  (extension\INSTALL.html), offline.
- The author's logo as its icons and on the video button; the popup's footer "By Sasanka
  Boruah -VU3INI", Contact by email; its help page inside the extension (help.html, offline),
  made by maharaj-build/make_ext_help.py; packed by maharaj-build/pack_extension.sh into
  chromium\ (Load unpacked), a .zip and an unsigned Firefox .xpi.

Helpers included, offline (crates/tools, crates/extract)
- yt-dlp, FFmpeg (ffmpeg, ffprobe), gallery-dl and 7-Zip are shipped in the program's
  "helpers" folder and used when no copy chosen by the user or newer verified copy exists.
- yt-dlp is still refreshed from its official, signed releases when online; never older than
  the included one.

Removed
- Automatic and manual update checks against the upstream project's releases (Maharaj
  Downloader is updated by installing a new version from its author).
- Help menu: the upstream home page, Tell a friend, and Support MDM.

Build
- Cross-compiled for Windows (x86_64-pc-windows-gnu, MinGW) on Linux: the Rust standard
  library built from source (-Z build-std), the exe's icon and version information by windres,
  and a small shim for MSVC's _invoke_watson (crate crash-handler) in crates/app/build.rs.
- The installer (tools/installer/maharaj.iss, Inno Setup 6) shows the Terms of Use as its
  licence page.
