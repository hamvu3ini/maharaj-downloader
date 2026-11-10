# Installing, carrying and removing Maharaj Downloader

Maharaj Downloader comes as one zip file whose name ends with its version number, for example `Maharaj_Downloader_v1.0.0.zip`. Everything is inside it: the setup program with the program, yt-dlp, FFmpeg, gallery-dl, 7-Zip, the browser extension, this Help and the licences. Nothing is downloaded during installation, so it installs on a computer without internet.

## Install (recommended)

1. Copy the zip to the computer and extract it (right-click → **Extract All...**).
2. Open the extracted folder and double-click **`Maharaj_Downloader_Setup_v1.0.0.exe`**.
3. If Windows says *"Windows protected your PC"*, click **More info**, then **Run anyway** (the program is not code-signed).
4. Follow the setup: it installs Maharaj Downloader for your account (no administrator rights needed), adds it to the Start menu, and can set up browser integration, open torrents and magnet links, start with Windows, and add a desktop icon.

## Carry it on a USB stick (portable)

Run the same setup program, choose a folder on the USB stick (or any other folder) as the destination, and on the **Select Additional Tasks** page tick **Portable copy** (and untick browser integration, torrents and start with Windows). The setup then creates a `data` folder next to the program: the download list and settings stay there, nothing is written to the computer's profile or registry, and there is no Start menu entry or uninstaller. Double-click **`MaharajDownloader.exe`** in that folder on any Windows 10 or 11 computer. See [Portable mode](portable.md).

## Update to a new version

Install the new version's setup over the old one: your download list and settings are kept. (Maharaj Downloader never downloads updates of itself; new versions come from its author.)

## Uninstall

**Windows Settings → Apps → Installed apps → Maharaj Downloader → Uninstall**. The uninstaller asks whether to delete your download list and settings too; downloaded files are never deleted. A portable copy has no uninstaller: delete its folder.

## Where things are

| What | Where |
| --- | --- |
| The program (installed) | `%LOCALAPPDATA%\Programs\Maharaj Downloader` |
| Download list, settings, logs | `%APPDATA%\maharajdownloader` (portable copy: its `data` folder) |
| yt-dlp, FFmpeg, gallery-dl, 7-Zip | the `helpers` folder next to the program |
| Licences of every component | the `LICENSES` folder next to the program |
| A record of your acceptances of the Terms | `terms-accepted.log` in the data folder |
