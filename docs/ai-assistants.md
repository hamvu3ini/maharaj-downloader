# AI assistants (MCP)

[← Back to docs index](README.md)

Maharaj Downloader comes with an [MCP](https://modelcontextprotocol.io) server, `MaharajDownloader-mcp`, so an AI assistant that supports MCP (Claude Desktop, Claude Code, Cursor, VS Code and others) can work with the downloads in your running Maharaj Downloader. You ask in your own words:

- "What is Maharaj Downloader downloading right now, and how long is left?"
- "Download https://example.com/big.iso to D:\ISOs tonight, don't start it yet."
- "Pause everything that's downloading."
- "Remove the finished downloads from the list but keep the files."

The assistant starts the server itself; you only tell the assistant where it is, once. The server passes each request to the app through its [automation API](command-line.md#the-automation-api), on this computer only.

## What the assistant can do

| Tool | What it does |
|---|---|
| `list_downloads` | Lists your downloads, newest first: file, address, folder, status, size, how much is done, speed. It can list only one status (Queued, Downloading, Stopped, Complete, Error) or only names and addresses that contain some text. |
| `add_download` | Adds an http, https, ftp or ftps address and starts it, or only queues it. It can say which folder and file name to use. A magnet link opens **Add torrent** in Maharaj Downloader, for you to choose the files. |
| `pause_downloads` | Pauses running downloads. |
| `resume_downloads` | Starts or resumes stopped, queued or failed downloads. |
| `remove_downloads` | Removes downloads from the list and keeps their files, or deletes the files too when asked. A running download must be paused first. |
| `start_queue` | Starts the main download queue. |

The assistant can't change Maharaj Downloader's options, see your browser, or turn the automation API on by itself.

## Set it up

### 1. Turn on the automation API

Listing and acting on downloads needs Maharaj Downloader's automation API, which is off until you turn it on. With Maharaj Downloader running, run this once (it stays on):

```
MaharajDownloader-cli api on
```

`MaharajDownloader-cli api off` turns it off again. Without the API, the assistant can still add downloads, the same as Maharaj Downloader's [command line](command-line.md#the-apps-command-line).

### 2. Find the server

`MaharajDownloader-mcp` is next to the app:

| You installed | The server |
|---|---|
| Windows, for you only (the installer's default) | `%LOCALAPPDATA%\Programs\Maharaj Downloader\MaharajDownloader-mcp.exe` |
| Windows, for everyone | `C:\Program Files\Maharaj Downloader\MaharajDownloader-mcp.exe` |
| Windows, a zip or [portable copy](portable.md) | `MaharajDownloader-mcp.exe` in its folder |
| macOS | `/Applications/Maharaj Downloader.app/Contents/MacOS/MaharajDownloader-mcp` |
| Linux, the .deb or .rpm | `/usr/bin/MaharajDownloader-mcp` |
| Linux, the .tar.gz | `MaharajDownloader-mcp` in its folder (the AppImage doesn't expose it: use the .tar.gz) |

A portable copy's server finds the portable copy's Maharaj Downloader by itself, from its own folder.

### 3. Tell your assistant

**Claude Desktop**: **Settings → Developer → Edit Config** opens `claude_desktop_config.json`. Add the server, then restart Claude Desktop:

```json
{
  "mcpServers": {
    "misterdownloadmanager": {
      "command": "C:\\Users\\you\\AppData\\Local\\Programs\\Maharaj Downloader\\MaharajDownloader-mcp.exe"
    }
  }
}
```

**Claude Code**:

```
claude mcp add misterdownloadmanager -- "C:\Users\you\AppData\Local\Programs\Maharaj Downloader\MaharajDownloader-mcp.exe"
```

**Cursor**: the same `mcpServers` entry as Claude Desktop's, in `~/.cursor/mcp.json` (or add it in Cursor's MCP settings).

**VS Code**: in `.vscode/mcp.json`, or **MCP: Add Server** in the command palette:

```json
{
  "servers": {
    "misterdownloadmanager": {
      "type": "stdio",
      "command": "C:\\Users\\you\\AppData\\Local\\Programs\\Maharaj Downloader\\MaharajDownloader-mcp.exe"
    }
  }
}
```

Use the path from step 2 (in JSON, every `\` is written `\\`). Other assistants take the same thing: a command to start, no arguments.

## Privacy

Maharaj Downloader itself still sends nothing anywhere: the server talks only to the Maharaj Downloader on this computer, over the token-protected local channel. But what the assistant reads, it passes to its own AI service: when it lists your downloads, their file names, addresses and folders go to that service, under that service's privacy terms. Turn the automation API off, or remove the server from the assistant, when you don't want that.

## When it doesn't work

| The assistant says | What to do |
|---|---|
| Maharaj Downloader isn't running | Start Maharaj Downloader. The server can't start it for you. |
| The automation API is off | Run `MaharajDownloader-cli api on` with Maharaj Downloader running. |
| Maharaj Downloader didn't answer in time | A window in Maharaj Downloader may be waiting for you (a question, **Add torrent**); answer it and ask again. |
| Maharaj Downloader refused the request | Maharaj Downloader's channel file is from an earlier start; restart Maharaj Downloader. |
| The tools don't show up at all | Check the path in the assistant's settings, and restart the assistant. Started by hand, `MaharajDownloader-mcp --help` says what it is. |
