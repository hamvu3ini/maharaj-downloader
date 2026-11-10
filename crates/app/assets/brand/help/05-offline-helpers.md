# The included helpers

Maharaj Downloader is complete offline: these programs come with it, in the `helpers` folder next to `MaharajDownloader.exe`, and it uses them automatically.

| Helper | What it does | Licence |
| --- | --- | --- |
| **yt-dlp** | looks up video pages and downloads videos and their sound | Unlicense (public domain) |
| **FFmpeg** (with ffprobe) | joins video and sound, converts to MP3, Opus and FLAC | GNU LGPL 2.1 or later |
| **gallery-dl** | downloads the pictures of image galleries | GNU GPL 2.0 |
| **7-Zip** | extracts 7z and RAR archives when they finish downloading | GNU LGPL + unRAR restriction |

Each was taken from its official release and checked against its publisher's checksum or signature before it was packed.

## Which copy is used

For each helper Maharaj Downloader uses, in this order:

1. a copy you chose yourself under **Helpers...** in the video window;
2. a newer copy it downloaded and verified while you were online (yt-dlp only, kept in the data folder's `tools` folder);
3. the copy included in the `helpers` folder.

So it always works offline, and still stays up to date when you are online.

## Licences

The licence texts are in the `LICENSES` folder next to the program; **Help → About → Licences and credits** opens it.
