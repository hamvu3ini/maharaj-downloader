#!/usr/bin/env python3
"""Make the browser extension's offline help page (extension/help.html) and the install guide
the app opens (INSTALL.html) from the guide's Markdown: Maharaj Downloader's own page about the
extension, then the user guide's page for a browser that isn't connected (its sections keep their
ids: the extension links to them, e.g. help.html#not-installed).

    python3 make_ext_help.py SRC_ROOT OUT_HELP_HTML [OUT_INSTALL_HTML]
"""
import base64
import pathlib
import re
import sys

import markdown

root = pathlib.Path(sys.argv[1])
own = (root / "crates/app/assets/brand/help/02c-browser-extension.md").read_text(encoding="utf-8")
guide = (root / "docs/browser-help.md").read_text(encoding="utf-8")
logo = base64.b64encode((root / "extension/icons/icon-128.png").read_bytes()).decode()

def tidy(md: str) -> str:
    # Links to other pages of the guide read as text (they are in the app's Help, F1);
    # web links stay links. Pictures of the guide aren't in the extension.
    md = re.sub(r"!\[[^\]]*\]\([^)]*\)", "", md)
    md = re.sub(r"\[([^\]]+)\]\((?!https?:|#)[^)]*\)", r"\1", md)
    md = re.sub(r"(?m)^\[← Back[^\n]*\n", "", md)
    md = re.sub(r"(?s)<!--.*?-->", "", md)
    return md

own = tidy(own)
guide = tidy(guide)
title = own.splitlines()[0].lstrip("# ").strip()
own_body = "\n".join(own.splitlines()[1:])
guide = re.sub(r"(?m)^# ", "## ", guide, count=1)
guide = re.sub(r"(?m)^## The browser and Maharaj Downloader aren't connected",
               "## If the browser and Maharaj Downloader aren't connected {#troubleshooting}", guide)
# The guide's sections become one level lower under it, keeping their ids.
guide = re.sub(r"(?m)^## (?!If the browser)", "### ", guide)
md = markdown.Markdown(extensions=["attr_list", "tables", "fenced_code", "toc"])
body = md.convert(own_body + "\n\n" + guide)

page = f"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
:root {{ --bg:#fff; --text:#1a1a1a; --muted:#5f5f5f; --edge:#d6d6d6; --field:#f5f5f5; --accent:#0067c0; color-scheme: light dark; }}
@media (prefers-color-scheme: dark) {{ :root {{ --bg:#202020; --text:#f3f3f3; --muted:#b5b5b5; --edge:#3d3d3d; --field:#2b2b2b; --accent:#4cc2ff; }} }}
body {{ margin:0; background:var(--bg); color:var(--text); font:15px/1.6 "Segoe UI", system-ui, sans-serif; }}
main {{ max-width: 820px; margin: 0 auto; padding: 24px 16px 48px; }}
header {{ display:flex; align-items:center; gap:16px; padding-bottom:16px; border-bottom:1px solid var(--edge); }}
header img {{ width:72px; height:72px; }}
header h1 {{ margin:0; font-size:26px; color:var(--accent); }}
header p {{ margin:2px 0 0; color:var(--muted); }}
h2 {{ margin-top:32px; padding-bottom:4px; border-bottom:1px solid var(--edge); font-size:20px; }}
h3 {{ margin-top:24px; font-size:16px; }}
a {{ color:var(--accent); }}
code {{ background:var(--field); border:1px solid var(--edge); border-radius:4px; padding:1px 5px; font-size:13px; }}
blockquote {{ margin:16px 0; padding:8px 14px; border-left:3px solid var(--accent); background:var(--field); }}
table {{ border-collapse:collapse; }} td, th {{ border:1px solid var(--edge); padding:4px 8px; }}
footer {{ margin-top:40px; padding-top:12px; border-top:1px solid var(--edge); color:var(--muted); font-size:13px; display:flex; justify-content:space-between; flex-wrap:wrap; gap:8px; }}
:target {{ scroll-margin-top: 12px; background: color-mix(in srgb, var(--accent) 14%, transparent); }}
</style>
</head>
<body>
<main>
<header>
  <img src="data:image/png;base64,{logo}" alt="">
  <div><h1>{title}</h1><p>Maharaj Downloader &middot; offline help</p></div>
</header>
{body}
<footer>
  <span>By Sasanka Boruah -VU3INI</span>
  <span>Contact: <a href="mailto:hamvu3ini@gmail.com">hamvu3ini@gmail.com</a> &middot; More help: press F1 in Maharaj Downloader</span>
</footer>
</main>
</body>
</html>
"""
for out in sys.argv[2:]:
    pathlib.Path(out).write_text(page, encoding="utf-8")
    print("wrote", out)
