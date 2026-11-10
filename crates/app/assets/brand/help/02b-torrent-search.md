# Search torrents

Maharaj Downloader can search for torrents and hand the one you choose to its own torrent downloader. Searching needs the internet; everything else in Maharaj Downloader works offline.

## Search

1. Choose **Tasks → Search torrents...**, click **Search torrents** at the bottom right of the main window, or choose it in the menu of the **Add Torrent** button.
2. Type a few words and press **Enter** (or click **Search**).
3. The results of every chosen source come in one list: name, size, seeders and source. Sort them by **Best match**, **Most seeders**, **Largest** or **Name**, and type in **Filter results** to narrow them.
4. Click **Download** beside a result (or double-click its name). **Add torrent** opens, as for any torrent: choose its files and the folder, and start it. **Page** opens the result's web page.

## Sources

**Internet Archive** is built in. It holds millions of public-domain and openly licensed films, music, books and programs, and every item has a torrent. Its results show *web seed* instead of a number of seeders: the Internet Archive's own servers always share the files, so they download even with no other peers.

### Add your own sources

Click **Sources...** to add a source: give it a name and its search address, with `{query}` where the search words go. Maharaj Downloader reads the answer as an RSS feed of torrents, so it works with:

- a site's **RSS search**, for sites that offer one;
- a **Torznab** indexer such as **Jackett** or **Prowlarr** running on your computer. For Jackett the address looks like
  `http://127.0.0.1:9117/api/v2.0/indexers/all/results/torznab/api?apikey=YOUR-KEY&t=search&q={query}`
  (copy it from Jackett's **Copy Torznab Feed** button and put `&t=search&q={query}` at its end).

Tick or untick the sources under the search box to choose which ones a search asks. **Remove** takes a source away. Your sources are kept in `torrent-search.json` in the data folder.

## Your responsibility

The results come from the sources, not from Maharaj Downloader, which doesn't check them. The sites you add are your own choice. Download only what you own, what is in the public domain or free, or what its owner allows you to download — see the [Terms of Use](terms-of-use.md). When you search, the sources you chose receive your search words.

## Related

- [Torrents](torrents.md): choosing files, sharing, and limits.
- [Torrents and your security](torrent-security.md): what others can see, proxies and blocklists.
