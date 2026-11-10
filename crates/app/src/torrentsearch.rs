//! Maharaj Downloader's torrent search (Tasks → Search torrents...): one query sent to the
//! chosen sources at once, their results in one list, and a result's torrent handed to Add
//! torrent with one click.
//!
//! Sources: Internet Archive, built in (public-domain and openly licensed films, music, books
//! and software; every item has a torrent), and the sites the user adds: any address with
//! `{query}` in it that answers with an RSS feed of torrents — a site's RSS search, or a
//! Torznab indexer such as Jackett's or Prowlarr's. Nothing site-specific ships in the program;
//! the sites a user adds, and what they download from them, are the user's own responsibility
//! (Terms of Use). Searching needs the internet; everything else works offline.

use std::sync::{Arc, Mutex};

use egui::{Align, RichText, Ui, ViewportBuilder, ViewportCommand, ViewportId, vec2};
use serde::{Deserialize, Serialize};

use crate::winui;

/// The most a source's answer may weigh.
const MAX_ANSWER: u64 = 8 * 1024 * 1024;
/// Results asked from Internet Archive.
const IA_ROWS: u32 = 60;
/// The built-in source's id.
pub const ARCHIVE: &str = "archive.org";

/// A site the user added: its name and its search address, with `{query}` where the words go.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    pub url: String,
    #[serde(default = "yes")]
    pub on: bool,
}

fn yes() -> bool {
    true
}

/// What is kept between starts: the sites added and whether Internet Archive is searched.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "yes")]
    pub archive: bool,
    #[serde(default)]
    pub sites: Vec<Site>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            archive: true,
            sites: Vec::new(),
        }
    }
}

impl Config {
    fn path() -> Option<std::path::PathBuf> {
        mdm_ipc::data_dir().map(|d| d.join("torrent-search.json"))
    }

    pub fn load() -> Self {
        Self::path()
            .and_then(|p| std::fs::read(p).ok())
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let (Some(p), Ok(json)) = (Self::path(), serde_json::to_vec_pretty(self)) {
            if let Some(dir) = p.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let _ = mdm_ipc::private_write(&p, &json);
        }
    }
}

/// Is `url` a search address a site can be added with: http(s), with `{query}` in it.
pub fn valid_template(url: &str) -> Result<(), &'static str> {
    let url = url.trim();
    if !url.contains("{query}") {
        return Err("The address needs {query} where the search words go.");
    }
    match url::Url::parse(&url.replace("{query}", "test")) {
        Ok(u) if matches!(u.scheme(), "http" | "https") => Ok(()),
        _ => Err("The address must start with http:// or https://."),
    }
}

/// The address to ask: the template with the words, %-encoded, in place of `{query}`.
pub fn fill(template: &str, query: &str) -> String {
    let enc: String = url::form_urlencoded::byte_serialize(query.trim().as_bytes()).collect();
    template.trim().replace("{query}", &enc)
}

/// Internet Archive's search for `query`: items that have a torrent.
pub fn archive_url(query: &str) -> String {
    let q = format!("({}) AND format:\"Archive BitTorrent\"", query.trim());
    let mut s = url::form_urlencoded::Serializer::new(String::new());
    s.append_pair("q", &q);
    for f in ["identifier", "title", "item_size", "downloads", "mediatype"] {
        s.append_pair("fl[]", f);
    }
    s.append_pair("rows", &IA_ROWS.to_string());
    s.append_pair("output", "json");
    format!("https://archive.org/advancedsearch.php?{}", s.finish())
}

/// One result.
#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub title: String,
    /// A magnet link or a .torrent's address.
    pub link: String,
    pub size: Option<u64>,
    pub seeders: Option<u64>,
    pub peers: Option<u64>,
    /// Internet Archive: how often the item was downloaded.
    pub downloads: Option<u64>,
    /// Its page, when there is one.
    pub page: Option<String>,
    /// Which source found it.
    pub source: String,
    /// Internet Archive: movies, audio, texts, software…
    pub kind: Option<String>,
}

/// A number given as a number or as text.
fn number(v: &serde_json::Value) -> Option<u64> {
    match v {
        serde_json::Value::Number(n) => n.as_u64().or_else(|| n.as_f64().map(|f| f as u64)),
        serde_json::Value::String(s) => s.trim().parse().ok(),
        serde_json::Value::Array(a) => a.first().and_then(number),
        _ => None,
    }
}

fn text(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Array(a) => a.first().and_then(text),
        serde_json::Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// Internet Archive's answer (advancedsearch.php, JSON): each item's torrent is
/// `https://archive.org/download/<id>/<id>_archive.torrent`.
pub fn parse_archive(json: &str) -> Result<Vec<Hit>, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let docs = v
        .get("response")
        .and_then(|r| r.get("docs"))
        .and_then(|d| d.as_array())
        .ok_or("not a search answer")?;
    Ok(docs
        .iter()
        .filter_map(|d| {
            let id = text(d.get("identifier")?)?;
            if id.is_empty() || id.contains('/') {
                return None;
            }
            let enc: String = percent_encoding::utf8_percent_encode(
                &id,
                percent_encoding::NON_ALPHANUMERIC,
            )
            .to_string();
            Some(Hit {
                title: d.get("title").and_then(text).unwrap_or_else(|| id.clone()),
                link: format!("https://archive.org/download/{enc}/{enc}_archive.torrent"),
                size: d.get("item_size").and_then(number),
                seeders: None,
                peers: None,
                downloads: d.get("downloads").and_then(number),
                page: Some(format!("https://archive.org/details/{enc}")),
                source: "Internet Archive".into(),
                kind: d.get("mediatype").and_then(text),
            })
        })
        .collect())
}

fn is_torrent_link(s: &str) -> bool {
    let l = s.trim().to_ascii_lowercase();
    l.starts_with("magnet:?") || l.starts_with("http://") || l.starts_with("https://")
}

/// An RSS feed of torrents (a site's RSS search, or Torznab: Jackett, Prowlarr). Per item: its
/// title; the magnet (`torznab:attr magneturl`, a magnet in `link`) or the .torrent (the
/// enclosure, else `link`); size, seeders and peers from Torznab's attributes, the enclosure's
/// length or `<size>`. roxmltree refuses DTDs, so no entity expansion.
pub fn parse_feed(xml: &str, source: &str) -> Result<Vec<Hit>, String> {
    let opts = roxmltree::ParsingOptions {
        allow_dtd: false,
        nodes_limit: 400_000,
    };
    let doc = roxmltree::Document::parse_with_options(xml, opts).map_err(|e| e.to_string())?;
    let root = doc.root_element();
    if root.tag_name().name().eq_ignore_ascii_case("error") {
        let why = root.attribute("description").unwrap_or("the site refused the search");
        return Err(why.to_owned());
    }
    let mut hits = Vec::new();
    for item in doc
        .descendants()
        .filter(|n| n.is_element() && matches!(n.tag_name().name(), "item" | "entry"))
    {
        let child_text = |name: &str| -> Option<String> {
            item.children()
                .find(|c| c.is_element() && c.tag_name().name() == name)
                .and_then(|c| c.text())
                .map(|t| t.trim().to_owned())
                .filter(|t| !t.is_empty())
        };
        let attr = |name: &str| -> Option<String> {
            item.children()
                .filter(|c| c.is_element() && c.tag_name().name() == "attr")
                .find(|c| c.attribute("name") == Some(name))
                .and_then(|c| c.attribute("value"))
                .map(str::to_owned)
        };
        let Some(title) = child_text("title") else {
            continue;
        };
        let enclosure = item
            .children()
            .find(|c| c.is_element() && c.tag_name().name() == "enclosure");
        let link_text = child_text("link").or_else(|| {
            item.children()
                .find(|c| c.is_element() && c.tag_name().name() == "link")
                .and_then(|c| c.attribute("href"))
                .map(str::to_owned)
        });
        let magnet = attr("magneturl")
            .or_else(|| link_text.clone().filter(|l| l.starts_with("magnet:")))
            .or_else(|| child_text("magnetURI"));
        let file = enclosure
            .and_then(|e| e.attribute("url"))
            .map(str::to_owned)
            .or_else(|| link_text.clone().filter(|l| !l.starts_with("magnet:")));
        let Some(link) = magnet.or(file).filter(|l| is_torrent_link(l)) else {
            continue;
        };
        let size = attr("size")
            .and_then(|s| s.parse().ok())
            .or_else(|| enclosure.and_then(|e| e.attribute("length")).and_then(|s| s.parse().ok()))
            .or_else(|| child_text("size").and_then(|s| s.parse().ok()))
            .or_else(|| child_text("contentLength").and_then(|s| s.parse().ok()))
            .filter(|s| *s > 0);
        let seeders = attr("seeders")
            .or_else(|| child_text("seeders"))
            .and_then(|s| s.parse().ok());
        let peers = attr("peers")
            .or_else(|| attr("leechers"))
            .or_else(|| child_text("leechers"))
            .and_then(|s| s.parse().ok());
        let page = child_text("comments")
            .or_else(|| child_text("guid"))
            .filter(|g| g.starts_with("http"))
            .or_else(|| link_text.filter(|l| l.starts_with("http") && *l != link));
        hits.push(Hit {
            title,
            link,
            size,
            seeders,
            peers,
            downloads: None,
            page,
            source: source.to_owned(),
            kind: attr("category").or_else(|| child_text("category")),
        });
    }
    Ok(hits)
}

/// One source's answer, as it arrives.
enum Event {
    Done(String, Result<Vec<Hit>, String>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Sort {
    Best,
    Seeders,
    Size,
    Name,
}

/// What the search window asks of the app.
pub enum Out {
    Open,
    Closed,
    /// Add torrent with this magnet or .torrent address.
    Download(String),
}

pub struct SearchWindow {
    config: Config,
    query: String,
    hits: Vec<Hit>,
    /// Sources still answering, and what each said.
    waiting: usize,
    notes: Vec<(String, String)>,
    events: Arc<Mutex<Vec<Event>>>,
    sort: Sort,
    filter: String,
    sites_open: bool,
    new_name: String,
    new_url: String,
    site_error: Option<&'static str>,
    focus: bool,
    raise: bool,
}

impl Default for SearchWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchWindow {
    pub fn new() -> Self {
        Self {
            config: Config::load(),
            query: String::new(),
            hits: Vec::new(),
            waiting: 0,
            notes: Vec::new(),
            events: Arc::new(Mutex::new(Vec::new())),
            sort: Sort::Best,
            filter: String::new(),
            sites_open: false,
            new_name: String::new(),
            new_url: String::new(),
            site_error: None,
            focus: true,
            raise: true,
        }
    }

    /// Bring the window to the front at its next frame.
    pub fn raise(&mut self) {
        self.raise = true;
    }

    fn search(&mut self, rt: &tokio::runtime::Handle, ctx: &egui::Context) {
        let query = self.query.trim().to_owned();
        if query.is_empty() {
            return;
        }
        self.hits.clear();
        self.notes.clear();
        self.events = Arc::new(Mutex::new(Vec::new()));
        let mut jobs: Vec<(String, String, bool)> = Vec::new();
        if self.config.archive {
            jobs.push(("Internet Archive".into(), archive_url(&query), true));
        }
        for s in self.config.sites.iter().filter(|s| s.on) {
            jobs.push((s.name.clone(), fill(&s.url, &query), false));
        }
        if jobs.is_empty() {
            self.notes
                .push(("".into(), "Choose at least one source to search.".into()));
            return;
        }
        self.waiting = jobs.len();
        for (name, address, archive) in jobs {
            let events = self.events.clone();
            let ctx = ctx.clone();
            rt.spawn(async move {
                let res = fetch(&address).await.and_then(|body| {
                    if archive {
                        parse_archive(&body)
                    } else {
                        parse_feed(&body, &name)
                    }
                });
                if let Ok(mut e) = events.lock() {
                    e.push(Event::Done(name, res));
                }
                ctx.request_repaint();
            });
        }
    }

    fn take_events(&mut self) {
        let done: Vec<Event> = self
            .events
            .lock()
            .map(|mut e| std::mem::take(&mut *e))
            .unwrap_or_default();
        for Event::Done(name, res) in done {
            self.waiting = self.waiting.saturating_sub(1);
            match res {
                Ok(hits) => {
                    self.notes
                        .push((name, format!("{} results", hits.len())));
                    self.hits.extend(hits);
                }
                Err(e) => self.notes.push((name, format!("failed: {e}"))),
            }
        }
    }

    fn sorted(&self) -> Vec<usize> {
        let f = self.filter.trim().to_lowercase();
        let mut idx: Vec<usize> = (0..self.hits.len())
            .filter(|&k| f.is_empty() || self.hits[k].title.to_lowercase().contains(&f))
            .collect();
        let h = &self.hits;
        match self.sort {
            Sort::Best => {}
            Sort::Seeders => idx.sort_by(|a, b| {
                let key = |x: &Hit| x.seeders.or(x.downloads).unwrap_or(0);
                key(&h[*b]).cmp(&key(&h[*a]))
            }),
            Sort::Size => idx.sort_by(|a, b| h[*b].size.unwrap_or(0).cmp(&h[*a].size.unwrap_or(0))),
            Sort::Name => idx.sort_by(|a, b| h[*a].title.to_lowercase().cmp(&h[*b].title.to_lowercase())),
        }
        idx
    }

    pub fn show(&mut self, ctx: &egui::Context, rt: &tokio::runtime::Handle) -> Out {
        self.take_events();
        let id = ViewportId::from_hash_of("maharaj-torrent-search");
        let title = "Search torrents - Maharaj Downloader";
        let mut builder = ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([620.0, 380.0])
            .with_resizable(true)
            .with_icon(crate::app_icon());
        if let Some(p) = crate::dialogs::place(ctx, egui::Id::new("maharaj-search-place"), [900.0, 600.0]) {
            builder = builder.with_position(p);
        }
        crate::crash::window("maharaj-torrent-search");
        let raise = std::mem::take(&mut self.raise);
        let mut out = Out::Open;
        let rt = rt.clone();
        ctx.show_viewport_immediate(id, crate::quiet(builder), |ui, _| {
            if ui.input(|i| i.viewport().close_requested()) {
                out = Out::Closed;
            }
            if raise && !crate::offscreen() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Focus);
            }
            #[cfg(windows)]
            {
                let _ = crate::win32::caption_dark(title, crate::theme::dark_surface());
            }
            egui::CentralPanel::no_frame()
                .frame(egui::Frame::new().fill(winui::BG()).inner_margin(egui::Margin::same(12)))
                .show(ui, |ui| {
                    if let Some(link) = self.body(ui, &rt) {
                        out = Out::Download(link);
                    }
                });
        });
        if self.waiting > 0 {
            ctx.request_repaint_after(std::time::Duration::from_millis(250));
        }
        out
    }

    fn body(&mut self, ui: &mut Ui, rt: &tokio::runtime::Handle) -> Option<String> {
        let text = winui::TEXT();
        let muted = winui::MUTED();
        let mut chosen = None;
        // The query.
        let mut go = false;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Search for").color(text));
            let edit = ui.add(
                egui::TextEdit::singleline(&mut self.query)
                    .hint_text("e.g. charlie chaplin, nasa apollo, ubuntu")
                    .desired_width((ui.available_width() - 190.0).max(160.0)),
            );
            if self.focus {
                edit.request_focus();
                self.focus = false;
            }
            if edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                go = true;
            }
            let label = if self.waiting > 0 { "Searching..." } else { "Search" };
            if ui
                .add_enabled(
                    self.waiting == 0 && !self.query.trim().is_empty(),
                    egui::Button::new(RichText::new(label).strong()).min_size(vec2(90.0, 24.0)),
                )
                .clicked()
            {
                go = true;
            }
            if ui
                .button(if self.sites_open { "Hide sources" } else { "Sources..." })
                .clicked()
            {
                self.sites_open = !self.sites_open;
            }
        });
        if go && self.waiting == 0 {
            let ctx = ui.ctx().clone();
            self.search(rt, &ctx);
        }
        // The sources: which are searched; the sites added.
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Sources:").size(12.0).color(muted));
            let mut changed = ui
                .checkbox(&mut self.config.archive, "Internet Archive (public domain)")
                .changed();
            for s in &mut self.config.sites {
                changed |= ui.checkbox(&mut s.on, &s.name).changed();
            }
            if changed {
                self.config.save();
            }
        });
        if self.sites_open {
            self.sites_panel(ui);
        }
        ui.add(
            egui::Label::new(
                RichText::new(
                    "Results come from the sources above, not from Maharaj Downloader. Download only \
                     what you own, what is free, or what its owner allows; you are responsible for \
                     what you download (Terms of Use).",
                )
                .size(11.0)
                .italics()
                .color(muted),
            )
            .wrap(),
        );
        ui.separator();
        // Status and sorting.
        ui.horizontal(|ui| {
            let status = if self.waiting > 0 {
                format!("Searching {} source(s)...", self.waiting)
            } else if self.notes.is_empty() {
                "Type words and press Enter.".to_owned()
            } else {
                self.notes
                    .iter()
                    .map(|(n, s)| if n.is_empty() { s.clone() } else { format!("{n}: {s}") })
                    .collect::<Vec<_>>()
                    .join("   ·   ")
            };
            ui.add(egui::Label::new(RichText::new(status).size(11.5).color(text)).truncate());
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                egui::ComboBox::from_id_salt("maharaj-search-sort")
                    .selected_text(match self.sort {
                        Sort::Best => "Best match",
                        Sort::Seeders => "Most seeders",
                        Sort::Size => "Largest",
                        Sort::Name => "Name",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.sort, Sort::Best, "Best match");
                        ui.selectable_value(&mut self.sort, Sort::Seeders, "Most seeders");
                        ui.selectable_value(&mut self.sort, Sort::Size, "Largest");
                        ui.selectable_value(&mut self.sort, Sort::Name, "Name");
                    });
                ui.label(RichText::new("Sort").size(12.0).color(muted));
                ui.add(
                    egui::TextEdit::singleline(&mut self.filter)
                        .hint_text("Filter results")
                        .desired_width(150.0),
                );
            });
        });
        ui.add_space(4.0);
        // The results.
        let order = self.sorted();
        egui::ScrollArea::vertical()
            .id_salt("maharaj-search-results")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if order.is_empty() && self.waiting == 0 && !self.notes.is_empty() {
                    ui.label(RichText::new("No results.").color(muted));
                }
                let width = ui.available_width();
                egui::Grid::new("maharaj-search-grid")
                    .striped(true)
                    .num_columns(5)
                    .min_col_width(40.0)
                    .spacing(vec2(12.0, 6.0))
                    .show(ui, |ui| {
                        for h in ["Name", "Size", "Seeders", "Source", ""] {
                            ui.label(RichText::new(h).strong().color(text));
                        }
                        ui.end_row();
                        for &k in &order {
                            let hit = &self.hits[k];
                            ui.vertical(|ui| {
                                ui.set_max_width((width - 380.0).max(220.0));
                                let name = ui.add(
                                    egui::Label::new(RichText::new(&hit.title).color(text))
                                        .wrap()
                                        .sense(egui::Sense::click()),
                                );
                                if name.double_clicked() {
                                    chosen = Some(hit.link.clone());
                                }
                                let mut sub = Vec::new();
                                if let Some(kind) = &hit.kind {
                                    sub.push(kind.clone());
                                }
                                if let Some(d) = hit.downloads {
                                    sub.push(format!("{d} downloads"));
                                }
                                if !sub.is_empty() {
                                    ui.label(RichText::new(sub.join(" · ")).size(10.5).color(muted));
                                }
                            });
                            ui.label(
                                RichText::new(hit.size.map_or("?".into(), crate::model::size_list))
                                    .color(text),
                            );
                            ui.label(
                                RichText::new(match (hit.seeders, hit.peers) {
                                    (Some(s), Some(p)) => format!("{s} / {p}"),
                                    (Some(s), None) => s.to_string(),
                                    _ if hit.source == "Internet Archive" => "web seed".into(),
                                    _ => "?".into(),
                                })
                                .color(text),
                            )
                            .on_hover_text("Seeders / peers");
                            ui.label(RichText::new(&hit.source).size(11.5).color(muted));
                            ui.horizontal(|ui| {
                                if ui
                                    .button("Download")
                                    .on_hover_text("Open it in Add torrent, to choose its files")
                                    .clicked()
                                {
                                    chosen = Some(hit.link.clone());
                                }
                                if let Some(page) = &hit.page
                                    && ui.small_button("Page").on_hover_text(page).clicked()
                                    && !crate::offscreen()
                                {
                                    crate::launch::open(page.clone());
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
        chosen
    }

    fn sites_panel(&mut self, ui: &mut Ui) {
        let text = winui::TEXT();
        let muted = winui::MUTED();
        egui::Frame::new()
            .stroke(egui::Stroke::new(1.0, winui::GROUP_LINE()))
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.label(RichText::new("Your own sources").strong().color(text));
                ui.add(
                    egui::Label::new(
                        RichText::new(
                            "Add a site's RSS search address, or a Torznab indexer (Jackett, Prowlarr), \
                             with {query} where the search words go. Example: \
                             http://127.0.0.1:9117/api/v2.0/indexers/all/results/torznab/api?apikey=KEY&t=search&q={query}",
                        )
                        .size(11.0)
                        .color(muted),
                    )
                    .wrap(),
                );
                let mut remove = None;
                for (k, s) in self.config.sites.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(&s.name).strong().color(text));
                        ui.label(RichText::new(&s.url).size(11.0).color(muted));
                        if ui.small_button("Remove").clicked() {
                            remove = Some(k);
                        }
                    });
                }
                if let Some(k) = remove {
                    self.config.sites.remove(k);
                    self.config.save();
                }
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_name)
                            .hint_text("Name")
                            .desired_width(130.0),
                    );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.new_url)
                            .hint_text("https://example.org/rss?q={query}")
                            .desired_width((ui.available_width() - 70.0).max(160.0)),
                    );
                    if ui.button("Add").clicked() {
                        match valid_template(&self.new_url) {
                            Ok(()) if !self.new_name.trim().is_empty() => {
                                self.config.sites.push(Site {
                                    name: self.new_name.trim().to_owned(),
                                    url: self.new_url.trim().to_owned(),
                                    on: true,
                                });
                                self.config.save();
                                self.new_name.clear();
                                self.new_url.clear();
                                self.site_error = None;
                            }
                            Ok(()) => self.site_error = Some("Give the source a name."),
                            Err(e) => self.site_error = Some(e),
                        }
                    }
                });
                if let Some(e) = self.site_error {
                    ui.label(RichText::new(e).size(11.0).color(winui::ERROR()));
                }
                ui.label(
                    RichText::new(
                        "Sites you add are your choice and your responsibility: Maharaj Downloader \
                         doesn't check them or what they list.",
                    )
                    .size(10.5)
                    .italics()
                    .color(muted),
                );
            });
    }
}

/// Fetch a source's answer with the app's own downloader (its proxy, DNS and adapter settings).
async fn fetch(address: &str) -> Result<String, String> {
    let url = url::Url::parse(address).map_err(|e| e.to_string())?;
    let remote = crate::torrents::Remote {
        url,
        parts: crate::app::RequestParts {
            headers: Default::default(),
            opts: crate::settings::own_net(),
        },
    };
    match crate::torrents::fetch_small(remote, MAX_ANSWER).await? {
        Some(bytes) => Ok(String::from_utf8_lossy(&bytes).into_owned()),
        None => Err("the answer is too large".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_need_query_and_http() {
        assert!(valid_template("https://x.org/rss?q={query}").is_ok());
        assert!(valid_template("https://x.org/rss?q=").is_err());
        assert!(valid_template("ftp://x.org/{query}").is_err());
        assert!(valid_template("not a url {query}").is_err());
        assert_eq!(
            fill("http://h/api?t=search&q={query}", "night of the living dead"),
            "http://h/api?t=search&q=night+of+the+living+dead"
        );
        assert_eq!(fill("http://h/?q={query}", "a&b=c"), "http://h/?q=a%26b%3Dc");
    }

    #[test]
    fn archive_search_address() {
        let u = archive_url("charlie chaplin");
        assert!(u.starts_with("https://archive.org/advancedsearch.php?q="));
        assert!(u.contains("Archive+BitTorrent"));
        assert!(u.contains("output=json"));
        assert!(u.contains("fl%5B%5D=identifier"));
    }

    #[test]
    fn archive_answer() {
        let json = r#"{"responseHeader":{"status":0},"response":{"numFound":2,"start":0,"docs":[
            {"identifier":"night_of_the_living_dead","title":"Night of the Living Dead","item_size":"1234567","downloads":98765,"mediatype":"movies"},
            {"identifier":"ote","item_size":139803,"mediatype":["texts"]},
            {"title":"no identifier"}]}}"#;
        let hits = parse_archive(json).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].title, "Night of the Living Dead");
        assert_eq!(
            hits[0].link,
            "https://archive.org/download/night%5Fof%5Fthe%5Fliving%5Fdead/night%5Fof%5Fthe%5Fliving%5Fdead_archive.torrent"
        );
        assert_eq!(hits[0].size, Some(1234567));
        assert_eq!(hits[0].downloads, Some(98765));
        assert_eq!(hits[1].title, "ote");
        assert_eq!(hits[1].kind.as_deref(), Some("texts"));
        assert!(parse_archive("not json").is_err());
        assert!(parse_archive("{}").is_err());
    }

    #[test]
    fn torznab_and_rss_answers() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:torznab="http://torznab.com/schemas/2015/feed">
<channel><title>Jackett</title>
<item><title>Big Buck Bunny 1080p</title>
  <guid>https://indexer.example/details/1</guid>
  <link>https://indexer.example/dl/1.torrent</link>
  <size>276445467</size>
  <enclosure url="https://indexer.example/dl/1.torrent" length="276445467" type="application/x-bittorrent"/>
  <torznab:attr name="seeders" value="42"/><torznab:attr name="peers" value="50"/>
  <torznab:attr name="magneturl" value="magnet:?xt=urn:btih:dd8255ecdc7ca55fb0bbf81323d87062db1f6d1c&amp;dn=Big+Buck+Bunny"/>
</item>
<item><title>Sintel</title><link>https://site.example/sintel.torrent</link></item>
<item><title>Just a page</title></item>
</channel></rss>"#;
        let hits = parse_feed(xml, "My indexer").unwrap();
        assert_eq!(hits.len(), 2);
        assert!(hits[0].link.starts_with("magnet:?xt=urn:btih:dd8255"));
        assert_eq!(hits[0].seeders, Some(42));
        assert_eq!(hits[0].peers, Some(50));
        assert_eq!(hits[0].size, Some(276445467));
        assert_eq!(hits[0].page.as_deref(), Some("https://indexer.example/details/1"));
        assert_eq!(hits[1].link, "https://site.example/sintel.torrent");
        assert_eq!(hits[1].source, "My indexer");
        // A Torznab error answer is said, not shown as no results.
        let err = parse_feed(r#"<error code="100" description="Invalid API Key"/>"#, "x");
        assert_eq!(err, Err("Invalid API Key".into()));
        // No DTDs (no entity expansion).
        assert!(parse_feed(r#"<!DOCTYPE x [<!ENTITY a "b">]><rss/>"#, "x").is_err());
    }

    #[test]
    fn the_config_reads_what_it_writes() {
        let c = Config {
            archive: false,
            sites: vec![Site {
                name: "Mine".into(),
                url: "https://x.org/?q={query}".into(),
                on: true,
            }],
        };
        let back: Config = serde_json::from_slice(&serde_json::to_vec(&c).unwrap()).unwrap();
        assert_eq!(back, c);
        let empty: Config = serde_json::from_str("{}").unwrap();
        assert!(empty.archive && empty.sites.is_empty());
    }
}
