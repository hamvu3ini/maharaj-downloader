# The Maharaj Downloader browser extension

The free **Maharaj Downloader** extension hands the downloads you start in your browser to Maharaj Downloader, puts a **Download video** button over videos, and adds **Download with Maharaj Downloader** to the right-click menu of links, pictures, videos and sound. It comes with Maharaj Downloader, so it installs offline, from the `extension` folder of the program.

The extension talks to Maharaj Downloader through its browser integration, which the setup turns on when **Set up browser integration** is ticked (it is by default).

## Add it to Chrome, Edge, Brave, Opera or Vivaldi

1. Open the browser's extensions page: type `chrome://extensions` (Edge: `edge://extensions`, Brave: `brave://extensions`, Opera: `opera://extensions`, Vivaldi: `vivaldi://extensions`) in the address bar and press Enter.
2. Turn on **Developer mode** (top right in Chrome; on the left in Edge).
3. Click **Load unpacked** and choose the folder `extension\chromium` in Maharaj Downloader's folder — installed: `%LOCALAPPDATA%\Programs\Maharaj Downloader\extension\chromium` (paste that into the folder box).
4. **Maharaj Downloader** appears in the list with its logo. Click the puzzle-piece button on the toolbar and pin it, so its button stays in sight.
5. Click the button: it should say **Connected to Maharaj Downloader**.

Keep the `chromium` folder where it is: the browser loads the extension from it at every start. When Chrome shows a note about extensions in developer mode, you can close it; the extension keeps working.

## Add it to Firefox

Firefox installs for good only extensions signed by Mozilla. The file `extension\maharaj-downloader-firefox.xpi` is the Firefox extension, not yet signed:

- **Firefox Developer Edition, Nightly or ESR**: open `about:config`, set `xpinstall.signatures.required` to `false`, then open `about:addons`, click the gear button, **Install Add-on From File...**, and choose the `.xpi`.
- **Firefox (regular)**: open `about:debugging#/runtime/this-firefox`, click **Load Temporary Add-on...** and choose the `.xpi`. Firefox removes it again when it closes.

To install it for good in regular Firefox, the author signs it once on addons.mozilla.org as a self-distributed (unlisted) add-on; a signed `.xpi` then installs by dragging it onto Firefox.

## Use it

- **Click a download link**: the file goes to Maharaj Downloader's Download File Info instead of the browser's downloads. Hold **Alt** while clicking to leave it to the browser.
- **Download video**: point at a video on a page; click the button on its edge. Its arrow hides it on one video or one site.
- **Right-click** a link, picture, video or sound: **Download with Maharaj Downloader**. On selected text or a page: download its links.
- **The toolbar button** shows whether the browser reaches Maharaj Downloader, pauses catching downloads, leaves one site to the browser, lists the media the page loaded, and opens Maharaj Downloader or its Options.

## If it says it isn't connected

- Check Maharaj Downloader is installed with **Set up browser integration** ticked (run the setup again to tick it), and running.
- The portable copy can't connect to browsers: install Maharaj Downloader with the setup.
- Reload the extension (the round arrow on its card in the extensions page) after installing Maharaj Downloader.

The extension's own help page (its toolbar button → **Help**) explains each message.

## Privacy

The extension sends the download's address, its page, and the cookies the site set for it to Maharaj Downloader on your own computer, so that it can download what you asked for. Nothing goes anywhere else.
