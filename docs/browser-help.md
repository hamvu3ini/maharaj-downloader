# The browser and Maharaj Downloader aren't connected

[← Back to docs index](README.md)

When the browser can't hand a download to Maharaj Downloader, it keeps the download itself, so nothing is lost. The extension's toolbar button then shows a red **!**. Click it to see what's wrong, then **How to fix it** brings you to the matching section below.

To check Maharaj Downloader's side, open **Options → General** in Maharaj Downloader and click **Test browser integration...**. It checks that browser integration is on, that this is the installed copy, that the browsers can find Maharaj Downloader, and which browsers' extension has connected; **How to fix it** opens the section for the first that fails.

## Maharaj Downloader isn't installed, or browser integration isn't set up {#not-installed}

The browser can't find the small helper program it talks to. It is registered when Maharaj Downloader is installed.

1. Install Maharaj Downloader with the setup from [the download page](https://misterdownloadmanager.com/download.html), and keep **Set up browser integration** ticked.
2. Already installed? Open Maharaj Downloader, then **Options → General → Test browser integration**, and click **Set it up** if it offers to.
3. The portable copy can't be connected: it writes nothing outside its own folder. Use the installed copy for browser integration. See [Portable mode](portable.md).
4. On Linux and macOS, Maharaj Downloader sets browser integration up itself each time it starts: start Maharaj Downloader once. **Test browser integration** says what went wrong if it couldn't, such as **The helper program the browsers start is missing**: **Set it up** puts it back.
5. Restart the browser after setting it up.
6. **Zen, Helium, Brave Origin or another browser built on Firefox or Chromium, as your only browser on Linux or macOS?** Older versions of Maharaj Downloader set browser integration up only for the browsers they knew; the current one also knows Brave Origin and the Beta, Dev and Nightly versions of Brave, Chrome and Edge. Update Maharaj Downloader and start it once.
7. **The AppImage on Linux:** the AppImage's files are gone once it quits, so Maharaj Downloader keeps a copy of its helper program in its data folder (`~/.local/share/misterdownloadmanager/browser-host`) and points the browsers there. Start Maharaj Downloader once after moving or renaming the AppImage, or after an AppImage manager (Gear Lever, AppImageLauncher and the like) moved it, so the helper knows where to start it from.
8. **Zen on Windows:** Zen 1.21.15b and later have a bug of their own that keeps add-ons from reaching their helper program ([zen-browser/desktop#15432](https://github.com/zen-browser/desktop/issues/15432)), with any add-on that uses one. Until Zen fixes it, Zen 1.21.14b works.

## Maharaj Downloader didn't start in time {#not-running}

When Maharaj Downloader isn't running, the browser starts it for each download and waits for it. It waited about 20 seconds this time and gave up.

- Start Maharaj Downloader yourself and try the download again.
- A first start, a slow disk, or an antivirus scanning Maharaj Downloader can take that long. Next time it is usually quick.
- With a VPN or proxy program, allow connections to `127.0.0.1` (your own computer): the browser's helper reaches Maharaj Downloader through one.
- If Maharaj Downloader never opens, see [The setup or Maharaj Downloader won't start](troubleshooting.md).

## The connection to Maharaj Downloader failed {#host-failed}

The browser started Maharaj Downloader's helper, but it stopped before answering. Security software is the usual cause.

- Allow `MaharajDownloader-nmhost.exe` (in Maharaj Downloader's folder) in your antivirus or anti-malware program.
- Reinstall Maharaj Downloader to restore its files.
- On Linux, with Firefox or Chromium installed as a Snap, see [Flatpak and Snap browsers](#linux-sandbox).
- Uninstalled Maharaj Downloader on Linux? The .deb and .rpm take their browser files with them; the AppImage's and the tar.gz's stay in your home folder: [remove them](getting-started.md#uninstall-linux), and the extension too.

## Maharaj Downloader doesn't accept this copy of the extension {#forbidden}

Maharaj Downloader's helper serves only its own extension. The browser says this copy isn't one it knows.

- **Maharaj Downloader 0.2.x with the extension from the Chrome Web Store?** The store's copy needs a newer Maharaj Downloader:
  0.2.x knows only the copy in its own folder. Update Maharaj Downloader with **Help → Check for updates...**.
  Until then, remove the store's copy
  and load the one in Maharaj Downloader's folder instead (`extension\chromium`, see
  [Browser integration](browser-integration.md#chrome-edge-brave-opera-and-vivaldi)).
- **Two copies installed?** Open the browser's extensions page (`chrome://extensions`, `edge://extensions`, `brave://extensions`) and keep only one **Maharaj Downloader**.
- **A copy from elsewhere?** Remove it and add the extension as Maharaj Downloader's **Add Maharaj Downloader to your browser** window shows (see [Browser integration](browser-integration.md)).
- **An older Maharaj Downloader?** Update Maharaj Downloader: a newer extension may have an ID an old helper doesn't know.

## The extension is newer than Maharaj Downloader {#update-app}

Downloads still work, but some of the extension's features need a newer Maharaj Downloader. Update it with **Help → Check for updates...** in Maharaj Downloader.

If Maharaj Downloader says it's up to date, there's nothing for you to do. The extension updates by itself through its store and can get ahead of the app for a while. Everything keeps working, and the message goes away with the next version of Maharaj Downloader.

## A download's details were too large {#too-large}

A download carries its site's cookies (and a form's data) to Maharaj Downloader. A few sites set so many that together they exceed what Maharaj Downloader accepts from the browser (1 MB). The browser kept this one. Signing out and back in to the site usually trims them.

## Downloads stay with the browser although Maharaj Downloader is connected {#options}

The extension is connected, but Maharaj Downloader's own settings leave this download to the browser. Check, in Maharaj Downloader:

1. **Options → General → Use advanced browser integration** is on, and your browser is ticked under **Capture downloads from the following browsers**.
2. The file's type is listed in **Options → File types**.
3. The site isn't in **Don't start downloading automatically from the following sites**, nor the address in **Don't start downloading automatically from the following addresses**.
4. The file isn't smaller than **Don't take files smaller than** in **Options → File types**.

Also check the extension's toolbar button: with **Take downloads from this browser** off, everything stays with the browser.

## The browser opens PDFs and music instead of sending them {#opened}

Maharaj Downloader also takes the file types the browser would show or play itself (a PDF, an MP3), when they're listed in **Options → File types** and **Also take files the browser would open** is ticked in **Options → General → Keys...**. This needs a Chromium browser (Chrome, Edge, Brave, Opera, Vivaldi) version 128 or later, or Firefox. Sites in Maharaj Downloader's excluded sites keep opening them in the browser. Hold **Alt** while clicking a link to leave it to the browser: Firefox then opens it, while Chromium browsers download it themselves (Alt+click is their own "download link").

## No Download video button on YouTube in Chrome, Edge or Brave {#store-youtube}

The copy of the extension from the Chrome Web Store showed no **Download video** button on YouTube before version 0.5.1. From 0.5.1 it shows it. Your browser updates extensions by itself; to get it sooner, open its extensions page and update them there.

You can always save a video without the button: in Maharaj Downloader, open **Tasks → Download video from a page** and paste the video's address.

## The extension can't read this site {#site-access}

In Chrome, Edge and Brave you can limit an extension to some sites (right-click its toolbar button → **This can read and change site data**). On a site it may not read, the extension can't pass your sign-in on, so it leaves the download to the browser. Choose **On all sites** to let Maharaj Downloader take downloads everywhere.

## Firefox forgets the extension after a restart {#firefox-temporary}

An add-on loaded with **Load Temporary Add-on** (from `about:debugging`) is removed when Firefox closes. Remove it and install the add-on from [its page on addons.mozilla.org](https://addons.mozilla.org/firefox/addon/mister-download-manager/) instead: Firefox keeps that one, and updates it.

## Linux: Flatpak and Snap browsers {#linux-sandbox}

Browsers installed as a Flatpak or a Snap run in a sandbox, which decides whether they can reach Maharaj Downloader.

- **Firefox as a Snap** (Ubuntu's own Firefox) reaches Maharaj Downloader through the desktop, which reads the browser files in your home folder and those the .deb or .rpm installs for everyone. The first time the extension calls Maharaj Downloader, the desktop asks **Allow Firefox to start WebExtension backend?** for Maharaj Downloader: choose **Allow**. After **Don't allow**, the extension says the connection failed and the desktop doesn't ask again; to be asked again, run this in a terminal, then use the extension again:

  `gdbus call --session --dest org.freedesktop.impl.portal.PermissionStore --object-path /org/freedesktop/impl/portal/PermissionStore --method org.freedesktop.impl.portal.PermissionStore.Delete webextensions com.misterdownloadmanager.host`
- **Chromium as a Snap** (Ubuntu's Chromium) starts Maharaj Downloader's helper inside its sandbox, where it can't reach Maharaj Downloader, so Maharaj Downloader doesn't list it in **Add Maharaj Downloader to your browser**. Use Firefox, or Google Chrome from its own `.deb`.
- **Flatpak browsers** (such as Zen's `app.zen_browser.zen`) can't start Maharaj Downloader's helper, so Maharaj Downloader doesn't list them either. Install the browser from your distribution's packages (or the browser maker's own `.deb`, `.rpm`, tarball or AppImage) to use browser integration.

## Next steps

- [Browser integration →](browser-integration.md)
- [Troubleshooting →](troubleshooting.md)
- [Back to the docs index →](README.md)
