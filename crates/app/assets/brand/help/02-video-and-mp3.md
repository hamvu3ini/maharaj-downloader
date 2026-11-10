# Videos and MP3 audio (yt-dlp)

Maharaj Downloader includes **yt-dlp**, the open-source video downloader that supports well over a thousand sites, and **FFmpeg**, which joins a video with its sound and converts sound to MP3, Opus or FLAC. Both are installed with Maharaj Downloader, in its `helpers` folder, so video and audio downloads work from the first start without downloading anything else.

## Download a video

1. Copy the address of the video's page (for example a YouTube, Vimeo or Facebook page).
2. Choose **Tasks → Download video from a page...** (or press **Ctrl+N** and paste the address: Maharaj Downloader recognises video pages).
3. Click **Look up**. After a few seconds the list shows every quality with its size, best first.
4. Pick a quality, check **Save to** and **File name**, and click **Download Now**.

## Download only the sound as MP3

In the same list, below the video qualities, choose one of the **Audio only** lines:

- **Audio only MP3** at the best variable bitrate, or at 320, 256, 192 or 128 kbps: an `.mp3` file that every player can play.
- **Audio only, original**: the site's own sound file (M4A or WebM/Opus), not converted — the best quality in the smallest file.
- **Audio only OPUS** or **FLAC**: for players and libraries that want those formats.

FFmpeg does the conversion; it is included, so these choices are always offered.

## Playlists and channels

Give the address of a playlist or a channel and Maharaj Downloader lists its videos; tick the ones you want, choose one quality (or MP3) for all of them, and they download one after another. See [Video downloads](video.md#playlists-and-channels).

## Subtitles

When a video has subtitles, choose them in the **Subtitles** list; tick **Put them into the video** to keep them inside the file.

## Sites that need you signed in

Some videos are visible only when you are signed in (age-restricted, members-only or private videos). Choose the browser you are signed in with in the video window, and yt-dlp uses that browser's sign-in. See [Video downloads](video.md#sites-that-want-you-signed-in).

## Keeping yt-dlp working

Video sites change often, and yt-dlp's developers release fixes every few weeks. The included copy works offline. When your computer is online, Maharaj Downloader quietly checks for a newer yt-dlp when you open the video window, verifies it against the signature of yt-dlp's developers, and uses it from then on; the included copy stays as a fallback. You can also choose your own copy under **Helpers...** in the video window.

If a video that played in your browser won't download, see [If a video won't download](video.md#if-a-video-wont-download).

## Please respect the law

Download only videos and music that you own, that are free to download, or that their owner allows you to download. Downloading copyrighted material without permission may be illegal where you live. See the [Terms of Use](terms-of-use.md).
