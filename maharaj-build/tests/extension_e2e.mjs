// The Maharaj Downloader extension in a real Chrome: loaded unpacked from the package's
// chromium folder, connected through the real Windows browser host (under Wine) to the running
// Maharaj Downloader. Checks its ID, the popup's connection, the offline help, the download
// hand-off from a clicked link, and the video button.
import puppeteer from "puppeteer-core";
import { execSync } from "node:child_process";

const EXT = "/home/claude/work/e2e/ext/chromium";
const ID = "hpmflmangdjofkncmfkchhahehlcjbho";
const results = [];
const check = (name, ok, detail = "") => {
  results.push({ name, ok });
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}${detail ? "  — " + detail : ""}`);
};
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const windows = () =>
  execSync(`DISPLAY=:99 bash -c 'for w in $(xdotool search --name . 2>/dev/null); do xdotool getwindowname $w 2>/dev/null; done'`)
    .toString().split("\n").filter(Boolean);

const browser = await puppeteer.launch({
  executablePath: "/usr/bin/google-chrome",
  headless: true,
  userDataDir: "/home/claude/work/e2e/profile",
  pipe: true,
  enableExtensions: [EXT],
  args: ["--no-sandbox", "--no-proxy-server", "--no-first-run", "--window-size=1200,900"],
});
try {
  // 1. Loaded, with its own ID.
  const sw = await browser.waitForTarget(
    (t) => t.type() === "service_worker" && t.url().startsWith("chrome-extension://"),
    { timeout: 20000 },
  );
  const id = new URL(sw.url()).host;
  check("extension loads, with its own ID", id === ID, id);

  // 2. The popup: name, footer, and connected to Maharaj Downloader through the host.
  const popup = await browser.newPage();
  await popup.setViewport({ width: 330, height: 560, deviceScaleFactor: 2 });
  await popup.goto(`chrome-extension://${ID}/popup.html`);
  let status = "";
  for (let i = 0; i < 60 && !/Connected/.test(status); i++) {
    await sleep(1000);
    status = await popup.$eval("#status-text", (e) => e.textContent).catch(() => "");
  }
  check("popup: connected to the app through the browser host", /Connected to Maharaj Downloader/.test(status), status);
  const header = await popup.$eval("header", (e) => e.textContent.trim());
  const credit = await popup.$eval(".credit", (e) => e.textContent.trim());
  const contact = await popup.$eval("#support", (e) => e.href);
  check("popup: name, footer and contact", header === "Maharaj Downloader" && credit === "By Sasanka Boruah -VU3INI" && contact.startsWith("mailto:hamvu3ini@gmail.com"), `${header} | ${credit}`);
  await popup.screenshot({ path: "/home/claude/work/e2e/popup.png", fullPage: true });

  // 3. The offline help page, with the sections the extension links to.
  const help = await browser.newPage();
  await help.setViewport({ width: 900, height: 900 });
  await help.goto(`chrome-extension://${ID}/help.html#not-installed`);
  const ids = await help.$$eval("[id]", (els) => els.map((e) => e.id));
  check("offline help page with its sections", ["not-installed", "not-running", "host-failed", "store-youtube"].every((x) => ids.includes(x)), `${ids.length} ids`);
  await help.goto(`chrome-extension://${ID}/help.html`);
  await help.screenshot({ path: "/home/claude/work/e2e/help.png" });

  // 4. A clicked download link goes to Maharaj Downloader (Download File Info opens there).
  const page = await browser.newPage();
  await page.setViewport({ width: 1100, height: 700 });
  await page.goto("http://127.0.0.1:8099/links.html");
  await sleep(1500);
  await page.click("#zip");
  let handed = false;
  for (let i = 0; i < 40 && !handed; i++) {
    await sleep(1000);
    handed = windows().includes("Download File Info");
  }
  check("clicked link handed to Maharaj Downloader", handed);

  // 5. The video button over a video: its element (a closed shadow tree inside) shows while the
  // pointer moves over the video.
  const box = await (await page.$("#v")).boundingBox();
  let shown = null;
  for (let i = 0; i < 12 && !(shown && shown.display !== "none"); i++) {
    await page.mouse.move(box.x + 100 + i * 20, box.y + 80 + i * 10);
    await sleep(400);
    shown = await page.evaluate(() => {
      const b = document.querySelector("mdm-video-button");
      if (!b) return null;
      const r = b.getBoundingClientRect();
      return { display: getComputedStyle(b).display, x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width) };
    });
  }
  check("Download video button over the video", !!shown && shown.display !== "none", JSON.stringify(shown));
  await page.screenshot({ path: "/home/claude/work/e2e/video.png" });
} catch (e) {
  check("run", false, String(e));
} finally {
  await browser.close();
}
const failed = results.filter((r) => !r.ok).length;
console.log(`extension e2e: ${results.length - failed} passed, ${failed} failed`);
process.exit(failed ? 1 : 0);
