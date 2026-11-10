// Maharaj Downloader's download button over videos (a content script, in every page and
// frame). Our own design: a small pill at the video's edge that appears while the pointer is
// over the video (or it has the keyboard's focus) and fades when the pointer rests or leaves.
// Clicking it opens the app's Download video window for this page (an embedded player's own
// address, which yt-dlp understands), with the video's own address when it has a plain one. It can be dragged along the video's edge, and stays where
// it was put on that site. Its menu hides it on this video, keeps it off the site (the app's
// Options → General → Download panels lists those sites), or opens those settings. The app
// decides where it may show at all (the native host: integration on, this browser captured,
// the button on, the site not excluded).
(() => {
  // One button per page: a copy cut off from the extension (after an update) gives way to the
  // new one.
  const prev = globalThis.__mdmVideoButton;
  if (prev && typeof prev === "object" && prev.alive()) return;
  if (prev && typeof prev.teardown === "function") prev.teardown();
  const ext = globalThis.browser ?? globalThis.chrome;

  // A video worth a button: big enough to be content, not a short muted looping clip (an
  // animated picture or a background). A sound player (<audio> with controls) gets one too.
  const MIN_W = 200;
  const MIN_H = 112;
  const MIN_AUDIO_H = 24;
  const CLIP_S = 15;
  // Shown this long after the pointer last moved over the video, kept while it is on the button.
  const IDLE_MS = 2600;
  const INSET = 10;
  const DRAG_PX = 4;
  // Our app icon (extension/icons/icon-48.png, as data: no extension file is exposed to pages).
  const LOGO = "data:image/png;base64,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";

  const t = (key, ...subs) => ext.i18n.getMessage(key, subs) || FALLBACK[key] || key;
  const FALLBACK = {
    video_download: "Download video",
    video_more: "More options",
    video_hide_here: "Hide on this video",
    video_settings: "Button settings...",
    video_sent: "Sent to Maharaj Downloader",
    video_failed: "Maharaj Downloader isn't installed or didn't start",
    video_off: "The download button is off on this site",
    video_drm: "Protected video: it can't be saved",
    video_unnamed: "Can't tell which video this is: open its own page and try there",
    video_drag: "Drag to move it along the video's edge",
  };
  // To the background. After the extension was updated or removed, this page's copy of the script
  // is cut off from it (calls throw at once): then it takes itself away.
  const send = (m) => {
    try {
      if (!ext.runtime?.id) throw new Error("gone");
      return ext.runtime.sendMessage(m).catch(() => ({ ok: false }));
    } catch {
      teardown();
      return Promise.resolve({ ok: false, gone: true });
    }
  };
  const host = location.hostname;
  // The address the app looks the video up by — the video's, not the page's. Most pages that play
  // a video are not the video's page: YouTube's home, a search, a channel or a playlist carries a
  // list in its address ("recommended", the results, the channel's videos), and yt-dlp reads such
  // an address as the list, so the app offered the whole feed instead of the video the button was
  // on. A watch page opened from a mix or a playlist has the same problem one level down
  // (…&list=…&index=…): its own page is watch?v=<id>, and nothing else.
  //
  // Null when the page plays a video but its DOM doesn't name it: YouTube's channel masthead player
  // has no video-id attribute, no link to the video inside the player and a blob: source. The page
  // itself never goes to the app then (a channel is a whole list to yt-dlp): the background asks
  // the player what it plays, in the page's own world (background.js playingVideo), and this
  // answer is only what it falls back on.
  function videoPage(v) {
    const watch = (id) =>
      /^[\w-]+$/.test(id || "") ? `https://www.youtube.com/watch?v=${id}` : null;
    try {
      const url = new URL(location.href);
      const name = url.hostname.replace(/^(www|m|music)\./, "");
      if (name === "youtu.be") {
        const id = url.pathname.split("/")[1];
        if (watch(id)) return `https://youtu.be/${id}`;
      }
      if (name === "youtube.com" || name === "youtube-nocookie.com") {
        // An embed's own page is the video's (…/embed/<id>).
        if (url.pathname.startsWith("/embed/")) {
          const id = watch(url.pathname.split("/")[2]);
          if (id) return id;
        }
        // A Short's or a live stream's own page (/shorts/<id>, /live/<id>). yt-dlp reads these as
        // the same one video as watch?v=<id>.
        const [, kind, pathId] = url.pathname.split("/");
        if (kind === "shorts" || kind === "live") {
          const id = watch(pathId);
          if (id) return id;
        }
        // The page is this video's own: a watch page, with or without a list on it.
        const own = watch(url.searchParams.get("v"));
        if (own && url.pathname.startsWith("/watch")) return own;
        const shown = watch(document.querySelector("ytd-watch-flexy")?.getAttribute("video-id"));
        if (shown && url.pathname.startsWith("/watch")) return shown;
        const player = watch(document.querySelector("#movie_player")?.getAttribute("video-id"));
        if (player) return player;
        // A player inside the card it belongs to (a feed, a search, a playlist): the card links to
        // the video. Only a container that names exactly one video counts — walking on into the
        // page's column would find some other card's link and send the wrong video, which is worse
        // than sending none.
        for (let node = v, i = 0; node && i < 12; i += 1, node = node.parentElement) {
          const ids = new Set();
          for (const a of node.querySelectorAll?.('a[href*="/watch?"], a[href*="youtu.be/"]') || []) {
            const link = new URL(a.href, location.href);
            const id = link.hostname.endsWith("youtu.be")
              ? link.pathname.split("/")[1]
              : link.searchParams.get("v");
            if (watch(id)) ids.add(id);
          }
          if (ids.size === 1) return watch([...ids][0]);
          if (ids.size > 1) break;
        }
        // A page that plays a video and names none: not the video's page, so not the app's.
        return null;
      }
    } catch {
      // Not an address this page can reason about: its own follows.
    }
    return location.href;
  }

  // The address this page first loaded at (`download`: a page that moves in place keeps its first
  // page's og:title).
  const landedAt = location.href;
  const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

  // "Hide on this video": the video's address (players reuse one element for the next video),
  // else the element.
  const dismissed = new WeakSet();
  const dismissedSrc = new Set();
  const isDismissed = (v) => dismissed.has(v) || (v.currentSrc && dismissedSrc.has(v.currentSrc));
  let allowed = null; // asked once per page: may it show here?
  let place = { x: 1, y: 0 }; // where on the edge: fractions of the room, top right by default
  let chosen = false; // the user put it where it is (dragged it, on this site): theirs, never moved for a page's control
  let auto = { v: null, at: null, t: 0 }; // the spot found for this video, kept for a moment
  const AUTO_MS = 400;
  let video = null; // the video it is over
  let lastMove = 0;
  let pointer = { x: -1, y: -1 };
  let menuOpen = false;
  let dragging = null;
  let busy = false;
  let flashing = false;
  let raf = 0;
  let scan = 0;
  // Kept on screen while the keyboard is on it (not after a mouse click: that leaves focus too),
  // or on the video's player (its own controls).
  const keyboardOn = () =>
    Boolean(shadow.activeElement && shadow.activeElement.matches(":focus-visible")) || playerFocused();
  let focusedPlayer = null;
  const playerFocused = () =>
    Boolean(video && focusedPlayer && document.activeElement === focusedPlayer &&
      focusedPlayer.contains(video) && focusedPlayer.matches(":focus-visible"));

  // ---- the button: a closed shadow tree the page's styles and scripts can't reach ----------
  const root = document.createElement("mdm-video-button");
  // Each with !important: a page's own rules can't hide or move it.
  for (const [k, v] of Object.entries({
    all: "initial", position: "fixed", "z-index": "2147483647", left: "0", top: "0", display: "none",
    contain: "layout style", opacity: "1", visibility: "visible", "pointer-events": "auto",
  })) root.style.setProperty(k, v, "important");
  const shadow = root.attachShadow({ mode: "closed" });
  const CSS = `
:host { all: initial; }
* { box-sizing: border-box; }
/* A light, glossy panel that stands out over a dark video, with our icon: the classic download
   button's feel, made modern (rounded, crisp type, soft shadow and motion). */
.pill {
  display: flex; align-items: stretch; height: 34px; border-radius: 9px;
  background: linear-gradient(#ffffff, #f1f4f8 48%, #e3e8ef 52%, #eef2f6);
  color: #17324d; border: 1px solid rgba(23, 50, 77, .28);
  box-shadow: 0 4px 14px rgba(0, 0, 0, .32), 0 1px 0 rgba(255, 255, 255, .9) inset;
  font: 600 13px/1 "Segoe UI", system-ui, -apple-system, Roboto, sans-serif;
  opacity: 0; transform: translateY(-3px) scale(.98);
  transition: opacity .16s ease-out, transform .16s ease-out, filter .12s;
  user-select: none; -webkit-user-select: none; touch-action: none; white-space: nowrap;
}
.pill.shown { opacity: 1; transform: none; }
.pill.dragging { cursor: grabbing; box-shadow: 0 10px 28px rgba(0, 0, 0, .45); transition: none; }
.pill.dragging .go { cursor: grabbing; }
.pill.ok { background: linear-gradient(#f2fff6, #d4f3df); border-color: #2f9a5b; color: #135c33; }
.pill.err { background: linear-gradient(#fff4f4, #f6d6d6); border-color: #c24848; color: #7d1f1f; }
button {
  /* all: unset resets box-sizing too, past the rule above: without it a menu item's 100 % width
     plus its padding ran past the menu's edge (#81). */
  all: unset; box-sizing: border-box; display: flex; align-items: center; gap: 8px;
  color: inherit; cursor: pointer; font: inherit;
}
.go { padding: 0 12px 0 8px; border-radius: 8px 0 0 8px; cursor: grab; }
.more { padding: 0 9px; border-left: 1px solid rgba(23, 50, 77, .18); border-radius: 0 8px 8px 0; }
.go:hover, .more:hover { background: linear-gradient(rgba(214, 232, 255, .9), rgba(182, 212, 250, .9)); }
.go:active, .more:active { background: rgba(160, 196, 242, .9); }
button:focus-visible { outline: 2px solid #2a78d6; outline-offset: -2px; }
.go img { width: 22px; height: 22px; flex: none; filter: drop-shadow(0 1px 1px rgba(0, 0, 0, .25)); }
.more svg { width: 12px; height: 12px; color: #3b5876; }
.compact .label { display: none; }
.compact .go { padding: 0 6px; border-radius: 8px 0 0 8px; }
.menu {
  position: absolute; min-width: 210px; padding: 4px; border-radius: 9px;
  background: #fbfcfe; color: #17324d; border: 1px solid rgba(23, 50, 77, .22);
  box-shadow: 0 12px 32px rgba(0, 0, 0, .35);
  font: 400 13px/1.2 "Segoe UI", system-ui, -apple-system, Roboto, sans-serif;
  display: none;
}
.menu.open { display: block; }
.menu button { display: block; width: 100%; padding: 8px 12px; border-radius: 6px; cursor: default; }
.menu button:hover, .menu button:focus-visible { background: #dceafb; outline: none; }
.menu hr { border: 0; border-top: 1px solid rgba(23, 50, 77, .12); margin: 4px 2px; }
@media (prefers-reduced-motion: reduce) { .pill { transition: none; transform: none; } }
`;
  // A constructed sheet: a page whose Content-Security-Policy refuses inline styles can't strip
  // it (Firefox applies the page's policy to what a content script adds).
  try {
    const sheet = new CSSStyleSheet();
    sheet.replaceSync(CSS);
    shadow.adoptedStyleSheets = [sheet];
  } catch {
    const style = document.createElement("style");
    style.textContent = CSS;
    shadow.appendChild(style);
  }
  shadow.innerHTML += `
<div class="pill" part="pill">
  <button class="go" type="button">
    <img alt="" draggable="false">
    <span class="label"></span>
  </button>
  <button class="more" type="button" aria-haspopup="menu" aria-expanded="false">
    <svg viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.6"
      stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M3 4.5 6 7.5l3-3"/></svg>
  </button>
</div>
<div class="menu" role="menu">
  <button role="menuitem" data-act="download"></button>
  <hr>
  <button role="menuitem" data-act="hide"></button>
  <button role="menuitem" data-act="never"></button>
  <hr>
  <button role="menuitem" data-act="settings"></button>
</div>`;
  shadow.querySelector(".go img").src = LOGO;
  const pill = shadow.querySelector(".pill");
  const go = shadow.querySelector(".go");
  const label = shadow.querySelector(".label");
  const more = shadow.querySelector(".more");
  const menu = shadow.querySelector(".menu");
  const item = (act) => menu.querySelector(`[data-act="${act}"]`);
  pill.dir = ext.i18n.getMessage("@@bidi_dir") || "ltr";
  menu.dir = pill.dir;
  const idleLabel = t("video_download");
  label.textContent = idleLabel;
  go.title = `${idleLabel} — ${t("video_drag")}`;
  go.setAttribute("aria-label", idleLabel);
  more.title = t("video_more");
  more.setAttribute("aria-label", t("video_more"));
  item("download").textContent = idleLabel;
  item("hide").textContent = t("video_hide_here");
  item("never").textContent = ext.i18n.getMessage("video_never_site", [host]) || `Never on ${host}`;
  item("settings").textContent = t("video_settings");

  // ---- which video ----------------------------------------------------------------------------
  const videos = document.getElementsByTagName("video");
  const audios = document.getElementsByTagName("audio");

  // Videos inside open shadow roots (players built as web components), looked for again every
  // few seconds at most.
  let shadowed = [];
  let shadowedAt = 0;
  function shadowVideos() {
    if (Date.now() - shadowedAt < 3000) return shadowed;
    shadowedAt = Date.now();
    const found = [];
    const walk = (root, depth) => {
      if (depth > 4) return;
      const w = document.createTreeWalker(root, NodeFilter.SHOW_ELEMENT);
      for (let n = w.nextNode(), seen = 0; n && seen < 20000; n = w.nextNode(), seen++) {
        if (n.shadowRoot) {
          found.push(...n.shadowRoot.querySelectorAll("video"));
          walk(n.shadowRoot, depth + 1);
        }
      }
    };
    walk(document.documentElement, 0);
    shadowed = found;
    return found;
  }

  function eligible(v) {
    if (!v.isConnected || isDismissed(v)) return false;
    const r = v.getBoundingClientRect();
    const audio = v instanceof HTMLAudioElement;
    if (audio ? !v.controls || r.width < MIN_W || r.height < MIN_AUDIO_H : r.width < MIN_W || r.height < MIN_H) {
      return false;
    }
    // A short muted loop without controls: a moving picture, not something to save.
    if (v.muted && v.loop && !v.controls && !(v.duration > CLIP_S)) return false;
    const s = getComputedStyle(v);
    return s.visibility !== "hidden" && s.display !== "none" && Number(s.opacity) > 0.05;
  }

  // The topmost eligible video under (x, y): players lay their controls over the video, so the
  // video itself rarely gets the pointer's events.
  function videoAt(x, y) {
    let best = null;
    for (const v of [...videos, ...audios, ...shadowVideos()]) {
      const r = v.getBoundingClientRect();
      if (x < r.left || x > r.right || y < r.top || y > r.bottom || !eligible(v)) continue;
      if (!best || r.width * r.height < best.r.width * best.r.height) best = { v, r };
    }
    return best && best.v;
  }

  // ---- placing it -------------------------------------------------------------------------
  // `place` is where it sits in the room the video leaves around it: x and y from 0 to 1, one of
  // them 0 or 1 (on an edge).
  function rects() {
    const r = video.getBoundingClientRect();
    const w = pill.offsetWidth || 150;
    const h = pill.offsetHeight || 32;
    return { r, w, h, roomX: Math.max(0, r.width - w - 2 * INSET), roomY: Math.max(0, r.height - h - 2 * INSET) };
  }

  // Is a control of the page under this spot (a viewport rectangle)? What counts is something
  // on top of the video that the user clicks and that is much smaller than the video: a button,
  // a small link, a watch-later or menu button on a preview (#421); a link or layer over the
  // whole video isn't one. On the video itself, and below it, is nothing to cover.
  function onControl(b, video, videoRect) {
    const area = (x) => Math.max(0, x.width) * Math.max(0, x.height);
    const small = (el) => area(el.getBoundingClientRect()) < area(videoRect) / 5;
    const points = [
      [(b.left + b.right) / 2, (b.top + b.bottom) / 2],
      [b.left + 4, b.top + 4],
      [b.right - 4, b.top + 4],
      [b.left + 4, b.bottom - 4],
      [b.right - 4, b.bottom - 4],
    ];
    for (const [x, y] of points) {
      for (const el of document.elementsFromPoint(x, y)) {
        if (el === root) continue;
        if (el === video || el.contains(video)) break;
        const control = el.closest("button, [role=button], a[href], input, select, [onclick]");
        if (control && !control.contains(video) && small(control)) return true;
        if (small(el) && getComputedStyle(el).cursor === "pointer") return true;
      }
    }
    return false;
  }

  // Where it goes now: the place the user gave it, else the default corner, or the next corner
  // free of the page's own controls (a thumbnail's watch-later button at the top right, #421).
  function spot(r, w, h, roomX, roomY) {
    if (chosen) return place;
    const now = Date.now();
    if (auto.v === video && now - auto.t < AUTO_MS) return auto.at;
    let at = place;
    for (const c of [place, { x: 0, y: 0 }, { x: 0, y: 1 }, { x: 1, y: 1 }]) {
      const left = r.left + INSET + c.x * roomX;
      const top = r.top + INSET + c.y * roomY;
      if (!onControl({ left, top, right: left + w, bottom: top + h }, video, r)) {
        at = c;
        break;
      }
    }
    auto = { v: video, at, t: now };
    return at;
  }

  function position() {
    if (!video) return;
    const { r, w, roomX, roomY } = rects();
    // A narrow video gets the icon only.
    pill.classList.toggle("compact", r.width < 360);
    const at = dragging ? dragging.at : spot(r, w, pill.offsetHeight || 32, roomX, roomY);
    const left = r.left + INSET + at.x * roomX;
    // In the part of the video on screen: one scrolled half out keeps its button in view.
    const h = pill.offsetHeight || 32;
    const visTop = Math.max(r.top, 0) + INSET;
    const visBottom = Math.min(r.bottom, innerHeight) - INSET - h;
    const top = Math.min(Math.max(r.top + INSET + at.y * roomY, visTop), Math.max(visTop, visBottom));
    root.style.setProperty("transform", `translate(${Math.round(left)}px, ${Math.round(top)}px)`, "important");
    if (menuOpen) placeMenu(top, left, w);
  }

  // The menu opens below the button, or above it near the bottom of the window; aligned with
  // whichever of its sides is nearer the video's middle.
  function placeMenu(top, left, w) {
    const h = menu.offsetHeight;
    const below = top + 36 + h < innerHeight;
    menu.style.top = below ? "36px" : `${-h - 4}px`;
    const { r } = rects();
    const rightHalf = left + w / 2 > r.left + r.width / 2;
    menu.style.left = rightHalf ? "" : "0";
    menu.style.right = rightHalf ? "0" : "";
  }

  // Snap a free spot (fractions, after a drag) to the nearest edge.
  function snap(at) {
    const d = { left: at.x, right: 1 - at.x, top: at.y, bottom: 1 - at.y };
    const edge = Object.keys(d).reduce((a, b) => (d[a] <= d[b] ? a : b));
    // Along the edge it stays where it was dropped, but close to a corner it goes into it.
    const c = (n) => (n < 0.08 ? 0 : n > 0.92 ? 1 : n);
    if (edge === "left") return { x: 0, y: c(at.y) };
    if (edge === "right") return { x: 1, y: c(at.y) };
    if (edge === "top") return { x: c(at.x), y: 0 };
    return { x: c(at.x), y: 1 };
  }

  // ---- showing and hiding ---------------------------------------------------------------------
  function attach() {
    // Over the video in fullscreen: inside the fullscreen element (a fullscreen <video> itself
    // can't hold it: then it hides).
    const fs = document.fullscreenElement;
    const parent = fs && fs !== video && fs.contains(video) ? fs : document.documentElement;
    if (root.parentNode !== parent) parent.appendChild(root);
    return !(fs && (fs === video || !fs.contains(video)));
  }

  function show(v) {
    if (video !== v) {
      video = v;
      closeMenu();
    }
    if (!attach()) return hide(true);
    root.style.setProperty("display", "block", "important");
    position();
    if (!pill.classList.contains("shown")) requestAnimationFrame(() => pill.classList.add("shown"));
    if (!raf) raf = requestAnimationFrame(tick);
  }

  // `now`: at once and whatever has the focus (dismissed, the video gone).
  function hide(now) {
    if (!now && (menuOpen || dragging || keyboardOn())) return;
    if (now) {
      closeMenu();
      shadow.activeElement?.blur();
    }
    pill.classList.remove("shown");
    cancelAnimationFrame(raf);
    raf = 0;
    const was = video;
    setTimeout(() => {
      if (!pill.classList.contains("shown") && video === was) root.style.setProperty("display", "none", "important");
    }, now || reduced ? 0 : 180);
  }

  // While shown: follow the video (scrolling, layout) and fade when the pointer rests.
  function tick() {
    raf = 0;
    if (!video || !video.isConnected || !eligible(video)) {
      video = null;
      return hide(true);
    }
    const r = video.getBoundingClientRect();
    const over =
      pointer.x >= r.left && pointer.x <= r.right && pointer.y >= r.top && pointer.y <= r.bottom;
    const onButton = root.matches(":hover");
    if (!onButton && !menuOpen && !dragging && !keyboardOn() && (!over || Date.now() - lastMove > IDLE_MS)) {
      return hide();
    }
    position();
    raf = requestAnimationFrame(tick);
  }

  // At most one look per frame, however fast the pointer moves.
  function onMove(e) {
    pointer = { x: e.clientX, y: e.clientY };
    lastMove = Date.now();
    if (!scan) scan = requestAnimationFrame(look);
  }

  function look() {
    scan = 0;
    if (dragging || allowed === false) return;
    const v = videoAt(pointer.x, pointer.y);
    if (!v) return;
    if (allowed === null) return void ask().then(() => allowed && look());
    show(v);
  }

  async function ask() {
    if (allowed !== null) return;
    allowed = false; // until answered: one question per page
    const r = await send({ type: "panel" });
    allowed = Boolean(r && r.on);
    if (allowed) {
      try {
        const { positions } = await ext.storage.local.get({ positions: {} });
        if (positions[host]) {
          place = snap(positions[host]);
          chosen = true;
        }
      } catch {
        // Default spot.
      }
    }
  }

  // ---- acting ---------------------------------------------------------------------------------
  async function download() {
    if (busy || !video) return;
    busy = true;
    try {
      closeMenu();
      // Encrypted (DRM): no tool can save it; say so rather than send it.
      if (video.mediaKeys) return flash("err", t("video_drm"));
      const src = /^https?:/i.test(video.currentSrc || "") ? video.currentSrc : "";
      // A page that moved in place keeps its first page's og:title; its document title follows.
      const og = location.href === landedAt ? document.querySelector('meta[property="og:title"]')?.content : "";
      const title = (og || document.title || "").trim();
      // The middle of the video: where the background asks the page's own player what it plays.
      const box = video.getBoundingClientRect();
      const at = { x: box.left + box.width / 2, y: box.top + box.height / 2 };
      // Asked now, not kept: one <video> plays many videos at the same address (a feed's previews).
      const r = await send({ type: "video", src, title, url: videoPage(video), at });
      if (r.gone) return;
      flash(r.ok ? "ok" : "err", t(r.ok ? "video_sent" : r.why === "options" ? "video_off" : r.why === "unnamed" ? "video_unnamed" : "video_failed"));
    } finally {
      busy = false;
    }
  }

  // A short confirmation on the button itself, then back to normal.
  function flash(kind, text) {
    flashing = true;
    pill.classList.remove("ok", "err");
    pill.classList.add(kind);
    label.textContent = text;
    lastMove = Date.now();
    position();
    setTimeout(() => {
      flashing = false;
      pill.classList.remove(kind);
      label.textContent = idleLabel;
      if (video) position();
    }, 2200);
  }

  function openMenu() {
    menuOpen = true;
    menu.classList.add("open");
    more.setAttribute("aria-expanded", "true");
    position();
    menu.querySelector("button").focus();
  }

  function closeMenu() {
    if (!menuOpen) return;
    menuOpen = false;
    menu.classList.remove("open");
    more.setAttribute("aria-expanded", "false");
  }

  menu.addEventListener("click", (e) => {
    if (!e.isTrusted) return;
    const act = e.target.closest("[data-act]")?.dataset.act;
    if (act) run(act);
  });

  async function run(act) {
    if (act === "download") return download();
    closeMenu();
    if (act === "hide") {
      dismissed.add(video);
      if (video.currentSrc) dismissedSrc.add(video.currentSrc);
      video = null;
      return hide(true);
    }
    if (act === "never") {
      allowed = false;
      video = null;
      hide(true);
      return send({ type: "hide-site" });
    }
    if (act === "settings") send({ type: "options" });
  }

  more.addEventListener("click", () => (menuOpen ? closeMenu() : openMenu()));

  // Keys: Escape closes the menu, arrows move through it.
  shadow.addEventListener("keydown", (e) => {
    if (e.key === "Escape" && menuOpen) {
      closeMenu();
      more.focus();
      e.stopPropagation();
    } else if (menuOpen && (e.key === "Enter" || e.key === " ") && menu.contains(shadow.activeElement)) {
      // The focused command, whatever the browser does with a key on a button.
      run(shadow.activeElement.dataset.act);
      e.preventDefault();
    } else if (menuOpen && (e.key === "ArrowDown" || e.key === "ArrowUp")) {
      const items = [...menu.querySelectorAll("button")];
      const i = items.indexOf(shadow.activeElement);
      items[(i + (e.key === "ArrowDown" ? 1 : items.length - 1)) % items.length].focus();
      e.preventDefault();
    }
  });

  // The main part: a click downloads, a drag moves it (and is not a click).
  // Only the user's own clicks and keys (never events a page script makes up).
  go.addEventListener("pointerdown", (e) => {
    if (e.button !== 0 || !video || !e.isTrusted) return;
    const { r, roomX, roomY } = rects();
    dragging = {
      id: e.pointerId,
      startX: e.clientX,
      startY: e.clientY,
      // Where in the button it was taken.
      dx: e.clientX - (r.left + INSET + (chosen ? place : auto.at || place).x * roomX),
      dy: e.clientY - (r.top + INSET + (chosen ? place : auto.at || place).y * roomY),
      at: chosen ? place : auto.at || place,
      moved: false,
    };
    go.setPointerCapture(e.pointerId);
    e.preventDefault();
  });
  go.addEventListener("pointermove", (e) => {
    if (!dragging || e.pointerId !== dragging.id) return;
    if (!dragging.moved && Math.hypot(e.clientX - dragging.startX, e.clientY - dragging.startY) < DRAG_PX) return;
    if (!dragging.moved) {
      dragging.moved = true;
      closeMenu();
      pill.classList.add("dragging");
    }
    const { r, roomX, roomY } = rects();
    const c = (n) => Math.min(1, Math.max(0, n));
    dragging.at = {
      x: roomX ? c((e.clientX - dragging.dx - r.left - INSET) / roomX) : 0,
      y: roomY ? c((e.clientY - dragging.dy - r.top - INSET) / roomY) : 0,
    };
    position();
  });
  const endDrag = (e) => {
    if (!dragging || e.pointerId !== dragging.id) return;
    const d = dragging;
    dragging = null;
    pill.classList.remove("dragging");
    if (!d.moved) {
      if (e.type === "pointerup") download();
      return;
    }
    place = snap(d.at);
    chosen = true;
    position();
    lastMove = Date.now();
    // Remembered per site, not in a private window (it would keep the site's name), and the
    // 200 sites used last only.
    if (ext.extension?.inIncognitoContext) return;
    try {
      ext.storage.local
        .get({ positions: {} })
        .then(({ positions }) => {
          delete positions[host];
          positions[host] = place;
          const sites = Object.keys(positions);
          for (const s of sites.slice(0, Math.max(0, sites.length - 200))) delete positions[s];
          return ext.storage.local.set({ positions });
        })
        .catch(() => {});
    } catch {
      teardown();
    }
  };
  go.addEventListener("pointerup", endDrag);
  go.addEventListener("pointercancel", endDrag);
  // Keyboard: Enter / Space (a real button's click without a pointer).
  go.addEventListener("click", (e) => {
    if (e.detail === 0 && e.isTrusted) download();
  });

  // ---- wiring -----------------------------------------------------------------------------------
  // The extension updated or removed under this page: the button and its listeners go (a fresh
  // copy of this script comes with the new extension).
  function teardown() {
    cancelAnimationFrame(raf);
    cancelAnimationFrame(scan);
    removeEventListener("pointermove", onMove, true);
    removeEventListener("keydown", onTab, true);
    root.remove();
    allowed = false;
    video = null;
  }
  globalThis.__mdmVideoButton = { alive: () => Boolean(ext.runtime?.id), teardown };
  addEventListener("pointermove", onMove, { capture: true, passive: true });
  addEventListener("pointerdown", (e) => {
    if (menuOpen && !e.composedPath().includes(root)) closeMenu();
  }, { capture: true, passive: true });
  // A video with the keyboard's focus, or a player around one (its controls), gets the button
  // too, kept while the focus stays there; Tab goes from there to the button.
  addEventListener("focusin", (e) => {
    if (!(e.target instanceof Element) || e.composedPath().includes(root)) return;
    const t = e.target;
    const v =
      t instanceof HTMLMediaElement ? t : [...t.querySelectorAll("video, audio")].find(eligible) || null;
    if (!v || !eligible(v)) return;
    focusedPlayer = t;
    ask().then(() => {
      if (!allowed) return;
      lastMove = Date.now();
      show(v);
    });
  }, true);
  // Tab on the focused player moves into the button first; Tab from the button goes back to
  // the player and on from there (its own controls next), as if the button sat at its start.
  let tabOn = 0;
  function onTab(e) {
    if (e.key !== "Tab" || e.shiftKey) return;
    // The Tab that just left the button: on from the player, as the browser goes.
    if (Date.now() - tabOn < 100) {
      tabOn = 0;
      return;
    }
    if (video && playerFocused() && root.style.getPropertyValue("display") === "block") {
      go.focus();
      e.preventDefault();
    }
  }
  addEventListener("keydown", onTab, true);
  go.addEventListener("keydown", (e) => {
    if (e.key === "Tab" && !e.shiftKey && focusedPlayer && focusedPlayer.isConnected) {
      tabOn = Date.now();
      focusedPlayer.focus();
    }
  });
  document.addEventListener("fullscreenchange", () => video && show(video));
  addEventListener("blur", () => hide());
})();
