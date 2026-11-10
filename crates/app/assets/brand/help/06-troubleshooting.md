# Troubleshooting Maharaj Downloader

## Windows says "Windows protected your PC"

The program isn't code-signed. Click **More info**, then **Run anyway**. Your antivirus may scan the program the first time it runs; that is normal.

## The Terms of Use window doesn't let me continue

Maharaj Downloader starts working only after you click **I Agree (OK)**. If you click **Exit**, it closes. The window can't be closed any other way, on purpose.

## A video won't download

- Check the page plays in your browser.
- If you are online, open **Tasks → Download video from a page...** once: Maharaj Downloader then fetches the newest yt-dlp, which fixes most problems with sites that changed.
- For videos that need you signed in, choose your browser in the video window.
- See [If a video won't download](video.md#if-a-video-wont-download).

## No MP3 choice in the video window

The MP3, Opus and FLAC choices need FFmpeg, which is included in the `helpers\ffmpeg` folder. If that folder was deleted, reinstall Maharaj Downloader, or choose your own FFmpeg under **Helpers...** in the video window.

## Downloads from the browser don't come to Maharaj Downloader

Install the browser extension and set up browser integration (the setup's **Set up browser integration** option, or **Options → General**). See [Browser integration](browser-integration.md) and [Browser help](browser-help.md).

## A 7z or RAR archive isn't extracted

7-Zip is included in `helpers\7-Zip`. Check the archive finished downloading and isn't password-protected (Maharaj Downloader asks for the password when it is).

## Something went wrong / the program closed

Maharaj Downloader saves a crash report and offers to send it by email to its author. You can also write to **hamvu3ini@gmail.com** with what you did and what happened (**Help → Report a problem**).

More: [Troubleshooting](troubleshooting.md) in the user guide.
