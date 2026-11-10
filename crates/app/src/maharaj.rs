//! Maharaj Downloader's own windows (Sasanka Boruah, VU3INI), on top of Mister Download
//! Manager: the start-up splash with the Terms of Use (it must be accepted before the app does
//! anything), the offline, searchable Help (the whole user guide built into the program), About,
//! and the footer's texts.

use std::sync::Mutex;

use egui::{Align, Color32, RichText, Stroke, Ui, ViewportBuilder, ViewportCommand, ViewportId, vec2};

use crate::winui;

pub const NAME: &str = "Maharaj Downloader";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "Sasanka Boruah";
pub const CALLSIGN: &str = "VU3INI";
pub const EMAIL: &str = "hamvu3ini@gmail.com";
/// The main window's footer, bottom left.
pub const FOOTER: &str = "By Sasanka Boruah -VU3INI";
/// What this build is made from (GPL-3.0-or-later).
pub const UPSTREAM: &str = "Mister Download Manager 0.5.2";
pub const UPSTREAM_AUTHOR: &str = "Ahmed Nefzaoui";
pub const UPSTREAM_URL: &str = "https://github.com/anefzaoui/misterdownloadmanager";
/// The Terms of Use shown at every start (assets/brand/terms.txt).
pub const TERMS: &str = include_str!("../assets/brand/terms.txt");
pub const TERMS_VERSION: &str = "1.0";
/// The program never updates itself from the upstream project's releases: new versions of
/// Maharaj Downloader come from its author as a new installer.
pub const SELF_UPDATE: bool = false;

/// The circular logo (the author's caricature and callsign).
fn logo() -> egui::Image<'static> {
    egui::Image::from_bytes(
        "bytes://maharaj-logo.png",
        include_bytes!("../assets/brand/logo-circle-256.png"),
    )
}

/// Open a link or a file with the system (never in an off-screen test run).
fn open(target: impl Into<String>) {
    if !crate::offscreen() {
        crate::launch::open(target.into());
    }
}

/// `mailto:` the author.
pub fn mail_author(subject: &str) {
    let enc = |s: &str| -> String {
        s.bytes()
            .map(|b| {
                if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
                    (b as char).to_string()
                } else {
                    format!("%{b:02X}")
                }
            })
            .collect()
    };
    open(format!("mailto:{EMAIL}?subject={}", enc(subject)));
}

/// The folder of the program (where LICENSES, the helpers and the guide's files are).
pub fn program_dir() -> Option<std::path::PathBuf> {
    std::env::current_exe().ok()?.parent().map(|p| p.to_path_buf())
}

// ---------------------------------------------------------------------------------------------
// The splash: logo, name, credentials and the Terms of Use; closes only through its buttons.

/// Why the splash is open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplashMode {
    /// At start: the app waits for "I Agree (OK)"; "Exit" quits.
    Startup,
    /// Help → Terms of Use: read again, OK closes.
    Review,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SplashOutcome {
    Open,
    Agreed,
    Exit,
}

/// The splash's size: 4 × 3 inches at Windows' 96 points per inch (it scales with the display).
pub const SPLASH_SIZE: [f32; 2] = [384.0, 288.0];

pub struct Splash {
    pub mode: SplashMode,
    /// Shown at least once (placed and raised).
    raised: bool,
}

impl Splash {
    pub fn new(mode: SplashMode) -> Self {
        Self {
            mode,
            raised: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> SplashOutcome {
        let id = ViewportId::from_hash_of(("maharaj-splash", self.mode == SplashMode::Startup));
        let title = if self.mode == SplashMode::Startup {
            "Maharaj Downloader - Welcome"
        } else {
            "Maharaj Downloader - Terms of Use"
        };
        let mut builder = ViewportBuilder::default()
            .with_title(title)
            .with_inner_size(SPLASH_SIZE)
            .with_resizable(false)
            // A title bar to move it by, without a close button: it closes through its own
            // buttons only (Alt+F4 is turned away below).
            .with_close_button(false)
            .with_minimize_button(false)
            .with_maximize_button(false)
            .with_always_on_top()
            .with_icon(crate::app_icon());
        if let Some(p) = splash_place(ctx) {
            builder = builder.with_position(p);
        }
        crate::crash::window("maharaj-splash");
        let mode = self.mode;
        let raise = !self.raised;
        self.raised = true;
        ctx.show_viewport_immediate(id, crate::quiet(builder), move |ui, _| {
            // Alt+F4 and the like: the splash closes through its buttons only.
            if ui.input(|i| i.viewport().close_requested()) {
                ui.ctx().send_viewport_cmd(ViewportCommand::CancelClose);
            }
            if raise && !crate::offscreen() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Focus);
            }
            let bg = winui::BG();
            #[cfg(windows)]
            {
                let _ = crate::win32::caption_dark(title, crate::theme::dark_surface());
            }
            egui::CentralPanel::no_frame()
                .frame(
                    egui::Frame::new()
                        .fill(bg)
                        .inner_margin(egui::Margin::same(10)),
                )
                .show(ui, |ui| splash_body(ui, mode))
                .inner
        })
    }
}

/// Centred over the main window, else on the screen.
fn splash_place(ctx: &egui::Context) -> Option<egui::Pos2> {
    let key = egui::Id::new("maharaj-splash-place");
    if let Some(p) = ctx.data(|d| d.get_temp::<egui::Pos2>(key)) {
        return Some(p);
    }
    let p = if let Some(p) = crate::dialogs::place(ctx, key.with("main"), SPLASH_SIZE) {
        p
    } else {
        let screen = ctx.input(|i| i.viewport().monitor_size)?;
        ((screen - vec2(SPLASH_SIZE[0], SPLASH_SIZE[1])) / 2.0)
            .to_pos2()
            .round()
    };
    ctx.data_mut(|d| d.insert_temp(key, p));
    Some(p)
}

fn splash_body(ui: &mut Ui, mode: SplashMode) -> SplashOutcome {
    let text = winui::TEXT();
    let muted = winui::MUTED();
    let accent = winui::accent();
    let mut out = SplashOutcome::Open;
    ui.spacing_mut().item_spacing = vec2(6.0, 2.0);
    // Logo in its circle, the name and the author's credentials.
    ui.horizontal(|ui| {
        ui.add(logo().fit_to_exact_size(vec2(76.0, 76.0)));
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.add_space(2.0);
            ui.label(RichText::new(NAME).size(19.0).strong().color(accent));
            ui.label(
                RichText::new(format!("Version {VERSION}  ·  Download manager"))
                    .size(11.0)
                    .color(muted),
            );
            ui.add_space(3.0);
            ui.label(
                RichText::new(format!("By {AUTHOR} ({CALLSIGN})"))
                    .size(12.5)
                    .strong()
                    .color(text),
            );
            ui.label(RichText::new(EMAIL).size(11.0).color(muted));
        });
    });
    ui.add_space(4.0);
    ui.label(
        RichText::new("Terms of Use — please read carefully")
            .size(11.5)
            .strong()
            .color(text),
    );
    ui.add_space(1.0);
    // The terms in a framed, scrolling box.
    let scroll_h = 118.0;
    egui::Frame::new()
        .stroke(Stroke::new(1.0, winui::GROUP_LINE()))
        .inner_margin(egui::Margin::symmetric(5, 3))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("maharaj-terms")
                .auto_shrink([false, false])
                .max_height(scroll_h)
                .min_scrolled_height(scroll_h)
                .show(ui, |ui| terms_text(ui, 10.5));
        });
    ui.add_space(3.0);
    let note = if mode == SplashMode::Startup {
        "By clicking \u{201c}I Agree (OK)\u{201d}, you accept and agree to abide by these Terms."
    } else {
        "You accepted these Terms when Maharaj Downloader started."
    };
    ui.label(RichText::new(note).size(10.0).italics().color(muted));
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        let ok_label = if mode == SplashMode::Startup {
            "I Agree (OK)"
        } else {
            "OK"
        };
        let w = ui.available_width();
        let ok_w = 118.0;
        if mode == SplashMode::Startup {
            if ui
                .add_sized(
                    [84.0, 24.0],
                    egui::Button::new(RichText::new("Exit").size(12.0)),
                )
                .on_hover_text("Do not accept, and close Maharaj Downloader")
                .clicked()
            {
                out = SplashOutcome::Exit;
            }
            ui.add_space((w - 84.0 - ok_w - 12.0).max(0.0));
        } else {
            ui.add_space((w - ok_w - 6.0).max(0.0));
        }
        let ok = egui::Button::new(
            RichText::new(ok_label)
                .size(12.5)
                .strong()
                .color(Color32::WHITE),
        )
        .fill(accent);
        if ui.add_sized([ok_w, 24.0], ok).clicked() {
            out = SplashOutcome::Agreed;
        }
    });
    out
}

/// The terms, headings in bold.
pub fn terms_text(ui: &mut Ui, size: f32) {
    let text = winui::TEXT();
    ui.spacing_mut().item_spacing.y = 3.0;
    for (n, line) in TERMS.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        let heading = n == 0
            || line
                .split_once(". ")
                .is_some_and(|(num, rest)| {
                    num.chars().all(|c| c.is_ascii_digit())
                        && rest.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase())
                });
        let rich = RichText::new(line).size(size).color(text);
        let rich = if heading {
            rich.family(egui::FontFamily::Name("tahoma-bold".into()))
        } else {
            rich
        };
        ui.add(egui::Label::new(rich).wrap());
    }
}

/// Note the acceptance in the data folder (when, which terms, which version): a record kept on
/// the user's own computer.
pub fn record_acceptance() {
    let Some(dir) = mdm_ipc::data_dir() else {
        return;
    };
    let _ = std::fs::create_dir_all(&dir);
    let line = format!(
        "{}  accepted Terms of Use {TERMS_VERSION} (Maharaj Downloader {VERSION})\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S %z")
    );
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("terms-accepted.log"))
    {
        let _ = f.write_all(line.as_bytes());
    }
}

/// The main window while the terms wait for an answer: nothing of the app, only a word.
pub fn waiting_screen(ui: &mut Ui) {
    let rect = ui.max_rect();
    ui.painter().rect_filled(rect, 0.0, winui::BG());
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect.shrink(24.0)));
    child.vertical_centered(|ui| {
        ui.add_space((rect.height() / 2.0 - 90.0).max(0.0));
        ui.add(logo().fit_to_exact_size(vec2(96.0, 96.0)));
        ui.add_space(8.0);
        ui.label(RichText::new(NAME).size(20.0).strong().color(winui::accent()));
        ui.add_space(4.0);
        ui.label(
            RichText::new("Please read and accept the Terms of Use to continue.")
                .size(13.0)
                .color(winui::TEXT()),
        );
    });
}

// ---------------------------------------------------------------------------------------------
// About

pub struct About;

impl About {
    pub fn show(&mut self, ctx: &egui::Context) -> crate::dialogs::Outcome<AboutLink> {
        use crate::dialogs::Outcome;
        crate::dialogs::window(
            ctx,
            "maharaj-about",
            "About Maharaj Downloader",
            [470.0, 336.0],
            false,
            winui::BG(),
            |ui, o| {
                let rect = egui::Rect::from_min_size(o + vec2(16.0, 14.0), vec2(438.0, 310.0));
                let mut ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(rect)
                        .layout(egui::Layout::top_down(Align::Min)),
                );
                let text = winui::TEXT();
                let muted = winui::MUTED();
                let mut out = Outcome::Open;
                ui.horizontal(|ui| {
                    ui.add(logo().fit_to_exact_size(vec2(92.0, 92.0)));
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.add_space(4.0);
                        ui.label(RichText::new(NAME).size(20.0).strong().color(winui::accent()));
                        let version = format!("Version {VERSION}");
                        ui.label(RichText::new(&version).size(12.0).color(muted))
                            .on_hover_text(format!("Build {}", crate::help::COMMIT));
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(format!("By {AUTHOR} ({CALLSIGN})"))
                                .size(13.0)
                                .strong()
                                .color(text),
                        );
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Contact:").size(12.0).color(text));
                            if ui
                                .link(RichText::new(EMAIL).size(12.0).color(winui::LINK()))
                                .on_hover_text("Write an email")
                                .clicked()
                            {
                                mail_author("Maharaj Downloader");
                            }
                            if ui.small_button("Copy").clicked() {
                                crate::copy_text(ui.ctx(), EMAIL.to_owned());
                            }
                        });
                    });
                });
                ui.add_space(10.0);
                ui.add(
                    egui::Label::new(
                        RichText::new(format!(
                            "A free download manager for personal use. Based on {UPSTREAM} by \
                             {UPSTREAM_AUTHOR}, free and open source under the GNU GPL, version 3 \
                             or later. Videos and audio (MP3) through yt-dlp, converted by FFmpeg; \
                             image galleries through gallery-dl; torrents on librqbit; 7z and RAR \
                             archives through 7-Zip. All helpers are included and work offline."
                        ))
                        .size(11.5)
                        .color(text),
                    )
                    .wrap(),
                );
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.link(RichText::new("Terms of Use").color(winui::LINK())).clicked() {
                        out = Outcome::Done(AboutLink::Terms);
                    }
                    ui.label(RichText::new("·").color(muted));
                    if ui.link(RichText::new("Help").color(winui::LINK())).clicked() {
                        out = Outcome::Done(AboutLink::Help);
                    }
                    ui.label(RichText::new("·").color(muted));
                    if ui.link(RichText::new("Licences and credits").color(winui::LINK())).clicked() {
                        if let Some(dir) = program_dir() {
                            let licences = dir.join("LICENSES");
                            open(if licences.is_dir() {
                                licences.display().to_string()
                            } else {
                                dir.join("LICENSE.txt").display().to_string()
                            });
                        }
                    }
                    ui.label(RichText::new("·").color(muted));
                    if ui.link(RichText::new("Source code").color(winui::LINK())).on_hover_text(UPSTREAM_URL).clicked() {
                        open(UPSTREAM_URL);
                    }
                });
                ui.add_space(6.0);
                ui.label(
                    RichText::new("© 2026 Sasanka Boruah (VU3INI). Mister Download Manager © 2026 Ahmed Nefzaoui.")
                        .size(10.0)
                        .color(muted),
                );
                ui.with_layout(egui::Layout::bottom_up(Align::Max), |ui| {
                    if ui.add_sized([86.0, 26.0], egui::Button::new("OK")).clicked()
                        || ui.input(|i| {
                            i.key_pressed(egui::Key::Escape) || i.key_pressed(egui::Key::Enter)
                        })
                    {
                        out = Outcome::Closed;
                    }
                });
                out
            },
        )
    }
}

/// A link in About that the app acts on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AboutLink {
    Terms,
    Help,
}

// ---------------------------------------------------------------------------------------------
// Help: the whole user guide inside the program, searchable, offline.

static REQUEST: Mutex<Option<String>> = Mutex::new(None);

/// Ask the main window to open Help at `doc` (a guide page's file, with an optional `#section`;
/// `""` for the contents). help::open_page calls this instead of the website.
pub fn request_help(doc: &str) {
    if let Ok(mut r) = REQUEST.lock() {
        *r = Some(doc.to_owned());
    }
}

pub fn take_help_request() -> Option<String> {
    REQUEST.lock().ok()?.take()
}

/// The guide's pages: Maharaj Downloader's own (assets/brand/help) and the user guide (docs/),
/// embedded by build.rs.
static EMBEDDED: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/help_pages.rs"));

#[derive(Clone, Debug, PartialEq)]
enum Inline {
    Text(String),
    Bold(String),
    Code(String),
    Link(String, String),
}

#[derive(Clone, Debug)]
enum Block {
    Heading(u8, Vec<Inline>, String),
    Para(Vec<Inline>),
    Item(usize, Option<String>, Vec<Inline>),
    Code(String),
    Quote(Vec<Inline>),
    Table(Vec<Vec<Vec<Inline>>>),
    Rule,
}

struct Page {
    file: String,
    title: String,
    blocks: Vec<Block>,
    /// For search: (block index of the section's heading, its heading, its plain text).
    sections: Vec<(usize, String, String)>,
}

fn pages() -> &'static Vec<Page> {
    static P: std::sync::OnceLock<Vec<Page>> = std::sync::OnceLock::new();
    P.get_or_init(|| {
        let mut list: Vec<Page> = Vec::new();
        for (file, text) in EMBEDDED {
            list.push(parse_page(file, text));
        }
        // The Terms of Use, as a page too.
        let mut terms = String::from("# Terms of Use\n\n");
        for (n, line) in TERMS.lines().enumerate() {
            if n == 0 || line.trim().is_empty() {
                continue;
            }
            let heading = line.split_once(". ").is_some_and(|(num, rest)| {
                num.chars().all(|c| c.is_ascii_digit())
                    && rest.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase())
            });
            if heading {
                terms.push_str(&format!("## {}\n\n", line.trim()));
            } else {
                terms.push_str(&format!("{}\n\n", line.trim()));
            }
        }
        list.push(parse_page("terms-of-use.md", &terms));
        list
    })
}

fn page_index(file: &str) -> Option<usize> {
    let file = file.trim_start_matches("./").to_ascii_lowercase();
    pages().iter().position(|p| p.file.to_ascii_lowercase() == file)
}

/// GitHub's anchor for a heading: lower case, letters, digits, spaces as dashes.
fn slug(s: &str) -> String {
    s.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_lowercase().next().unwrap_or(c))
            } else if c == ' ' || c == '-' {
                Some('-')
            } else if c == '_' {
                Some('_')
            } else {
                None
            }
        })
        .collect()
}

fn plain(inl: &[Inline]) -> String {
    inl.iter()
        .map(|i| match i {
            Inline::Text(s) | Inline::Bold(s) | Inline::Code(s) => s.as_str(),
            Inline::Link(t, _) => t.as_str(),
        })
        .collect()
}

/// Inline markdown: **bold**, `code`, [links](target), *emphasis* (shown plain), images dropped.
fn inline(s: &str) -> Vec<Inline> {
    let mut out = Vec::new();
    let mut text = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let flush = |text: &mut String, out: &mut Vec<Inline>| {
        if !text.is_empty() {
            out.push(Inline::Text(std::mem::take(text)));
        }
    };
    let find = |from: usize, pat: &str| -> Option<usize> {
        let pc: Vec<char> = pat.chars().collect();
        (from..chars.len()).find(|&k| chars[k..].starts_with(&pc))
    };
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            text.push(chars[i + 1]);
            i += 2;
            continue;
        }
        if c == '*' && chars.get(i + 1) == Some(&'*') {
            if let Some(end) = find(i + 2, "**") {
                flush(&mut text, &mut out);
                let inner: String = chars[i + 2..end].iter().collect();
                // A link inside bold stays a link.
                for part in inline(&inner) {
                    out.push(match part {
                        Inline::Text(t) => Inline::Bold(t),
                        other => other,
                    });
                }
                i = end + 2;
                continue;
            }
        }
        if c == '`' {
            if let Some(end) = find(i + 1, "`") {
                flush(&mut text, &mut out);
                out.push(Inline::Code(chars[i + 1..end].iter().collect()));
                i = end + 1;
                continue;
            }
        }
        let image = c == '!' && chars.get(i + 1) == Some(&'[');
        if c == '[' || image {
            let open = if image { i + 1 } else { i };
            if let Some(close) = find(open + 1, "](")
                && let Some(end) = find(close + 2, ")")
            {
                let label: String = chars[open + 1..close].iter().collect();
                let target: String = chars[close + 2..end].iter().collect();
                flush(&mut text, &mut out);
                if !image {
                    out.push(Inline::Link(plain(&inline(&label)), target));
                }
                i = end + 1;
                continue;
            }
        }
        if (c == '*' || c == '_')
            && chars.get(i + 1).is_some_and(|n| !n.is_whitespace())
            && (i == 0 || !chars[i - 1].is_alphanumeric())
            && let Some(end) = find(i + 1, &c.to_string())
            && end > i + 1
        {
            let inner: String = chars[i + 1..end].iter().collect();
            text.push_str(&inner);
            i = end + 1;
            continue;
        }
        text.push(c);
        i += 1;
    }
    flush(&mut text, &mut out);
    out
}

fn parse_page(file: &str, md: &str) -> Page {
    let mut blocks: Vec<Block> = Vec::new();
    let mut para: Vec<String> = Vec::new();
    let mut quote: Vec<String> = Vec::new();
    let mut table: Vec<Vec<Vec<Inline>>> = Vec::new();
    let mut code: Option<String> = None;
    let mut in_html_comment = false;
    fn end_para(para: &mut Vec<String>, blocks: &mut Vec<Block>) {
        if !para.is_empty() {
            blocks.push(Block::Para(inline(&para.join(" "))));
            para.clear();
        }
    }
    fn end_quote(quote: &mut Vec<String>, blocks: &mut Vec<Block>) {
        if !quote.is_empty() {
            let text = quote
                .iter()
                .map(|l| l.trim())
                .filter(|l| !l.starts_with("[!"))
                .collect::<Vec<_>>()
                .join(" ");
            if !text.is_empty() {
                blocks.push(Block::Quote(inline(&text)));
            }
            quote.clear();
        }
    }
    fn end_table(table: &mut Vec<Vec<Vec<Inline>>>, blocks: &mut Vec<Block>) {
        if !table.is_empty() {
            blocks.push(Block::Table(std::mem::take(table)));
        }
    }
    for raw in md.lines() {
        let line = raw.trim_end();
        if let Some(buf) = code.as_mut() {
            if line.trim_start().starts_with("```") {
                blocks.push(Block::Code(std::mem::take(buf)));
                code = None;
            } else {
                buf.push_str(line);
                buf.push('\n');
            }
            continue;
        }
        let t = line.trim_start();
        if in_html_comment {
            if t.contains("-->") {
                in_html_comment = false;
            }
            continue;
        }
        if t.starts_with("<!--") {
            if !t.contains("-->") {
                in_html_comment = true;
            }
            continue;
        }
        if t.starts_with("```") {
            end_para(&mut para, &mut blocks);
            end_quote(&mut quote, &mut blocks);
            end_table(&mut table, &mut blocks);
            code = Some(String::new());
            continue;
        }
        // HTML (pictures, centred blocks) and images on their own line: not shown here.
        if t.starts_with('<') || (t.starts_with("![") && t.ends_with(')')) {
            continue;
        }
        if t.is_empty() {
            end_para(&mut para, &mut blocks);
            end_quote(&mut quote, &mut blocks);
            end_table(&mut table, &mut blocks);
            continue;
        }
        if t.starts_with('|') {
            end_para(&mut para, &mut blocks);
            let cells: Vec<&str> = t.trim_matches('|').split('|').map(|c| c.trim()).collect();
            if cells
                .iter()
                .all(|c| !c.is_empty() && c.chars().all(|ch| ch == '-' || ch == ':'))
            {
                continue; // the header's underline
            }
            table.push(cells.into_iter().map(inline).collect());
            continue;
        }
        end_table(&mut table, &mut blocks);
        if let Some(rest) = t.strip_prefix('>') {
            end_para(&mut para, &mut blocks);
            quote.push(rest.trim().to_owned());
            continue;
        }
        end_quote(&mut quote, &mut blocks);
        if t.starts_with('#') {
            end_para(&mut para, &mut blocks);
            let level = t.chars().take_while(|c| *c == '#').count().min(6) as u8;
            let text = t[level as usize..].trim();
            let inl = inline(text);
            let anchor = slug(&plain(&inl));
            blocks.push(Block::Heading(level, inl, anchor));
            continue;
        }
        if t == "---" || t == "***" || t == "___" {
            end_para(&mut para, &mut blocks);
            blocks.push(Block::Rule);
            continue;
        }
        let indent = (line.len() - t.len()) / 2;
        if let Some(rest) = t
            .strip_prefix("- ")
            .or_else(|| t.strip_prefix("* "))
            .or_else(|| t.strip_prefix("+ "))
        {
            end_para(&mut para, &mut blocks);
            blocks.push(Block::Item(indent, None, inline(rest)));
            continue;
        }
        if let Some((num, rest)) = t.split_once(". ")
            && !num.is_empty()
            && num.len() <= 3
            && num.chars().all(|c| c.is_ascii_digit())
        {
            end_para(&mut para, &mut blocks);
            blocks.push(Block::Item(indent, Some(format!("{num}.")), inline(rest)));
            continue;
        }
        // A list item's continuation line.
        if indent > 0
            && para.is_empty()
            && let Some(Block::Item(_, _, inl)) = blocks.last_mut()
        {
            inl.push(Inline::Text(" ".into()));
            inl.extend(inline(t));
            continue;
        }
        para.push(t.to_owned());
    }
    end_para(&mut para, &mut blocks);
    end_quote(&mut quote, &mut blocks);
    end_table(&mut table, &mut blocks);
    if let Some(buf) = code {
        blocks.push(Block::Code(buf));
    }
    let title = blocks
        .iter()
        .find_map(|b| match b {
            Block::Heading(1, inl, _) => Some(plain(inl)),
            _ => None,
        })
        .unwrap_or_else(|| file.trim_end_matches(".md").replace('-', " "));
    // Sections for search: from each heading to the next.
    let mut sections: Vec<(usize, String, String)> = Vec::new();
    let mut current = (0usize, title.clone(), String::new());
    for (k, b) in blocks.iter().enumerate() {
        let text = match b {
            Block::Heading(_, inl, _) => {
                if !current.2.trim().is_empty() || k > 0 {
                    sections.push(std::mem::take(&mut current));
                }
                current = (k, plain(inl), String::new());
                continue;
            }
            Block::Para(inl) | Block::Quote(inl) | Block::Item(_, _, inl) => plain(inl),
            Block::Code(s) => s.clone(),
            Block::Table(rows) => rows
                .iter()
                .map(|r| r.iter().map(|c| plain(c)).collect::<Vec<_>>().join(" "))
                .collect::<Vec<_>>()
                .join(" "),
            Block::Rule => String::new(),
        };
        current.2.push_str(&text);
        current.2.push(' ');
    }
    sections.push(current);
    sections.retain(|s| !s.1.is_empty() || !s.2.trim().is_empty());
    Page {
        file: file.to_owned(),
        title,
        blocks,
        sections,
    }
}

/// One search hit: a page's section.
#[derive(Clone, Debug)]
pub struct Hit {
    page: usize,
    block: usize,
    heading: String,
    snippet: String,
    score: usize,
}

fn terms_of(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|t| t.to_lowercase())
        .filter(|t| !t.is_empty())
        .collect()
}

/// Every section that holds all the words of `query`, best first.
fn search(query: &str) -> Vec<Hit> {
    let terms = terms_of(query);
    if terms.is_empty() {
        return Vec::new();
    }
    let mut hits = Vec::new();
    for (pi, page) in pages().iter().enumerate() {
        for (block, heading, text) in &page.sections {
            let low_text = text.to_lowercase();
            let low_head = heading.to_lowercase();
            let low_title = page.title.to_lowercase();
            if !terms.iter().all(|t| {
                low_text.contains(t.as_str()) || low_head.contains(t.as_str()) || low_title.contains(t.as_str())
            }) {
                continue;
            }
            let score: usize = terms
                .iter()
                .map(|t| {
                    low_text.matches(t.as_str()).count()
                        + 8 * low_head.matches(t.as_str()).count()
                        + 4 * low_title.matches(t.as_str()).count()
                })
                .sum();
            // A few words around the first match.
            let snippet = match terms.iter().filter_map(|t| low_text.find(t.as_str())).min() {
                Some(at) => {
                    let start = low_text[..at]
                        .char_indices()
                        .rev()
                        .nth(50)
                        .map_or(0, |(k, _)| k);
                    let start = text.char_indices().map(|(k, _)| k).find(|k| *k >= start).unwrap_or(0);
                    let s: String = text[start..].chars().take(140).collect();
                    format!("{}{}…", if start > 0 { "…" } else { "" }, s.trim())
                }
                None => text.chars().take(140).collect::<String>() + "…",
            };
            hits.push(Hit {
                page: pi,
                block: *block,
                heading: heading.clone(),
                snippet,
                score,
            });
        }
    }
    hits.sort_by(|a, b| b.score.cmp(&a.score));
    hits
}

pub struct HelpWindow {
    page: usize,
    query: String,
    hits: Vec<Hit>,
    searched: String,
    /// A block to scroll to (a section's heading).
    jump: Option<usize>,
    /// An anchor to scroll to, found when the page is drawn.
    anchor: Option<String>,
    back: Vec<usize>,
    to_top: bool,
    focus_search: bool,
    /// Brought to the front at its next frame (opened again, or a link to it followed).
    raise: bool,
}

impl HelpWindow {
    /// Help at `doc` (`"video.md#playlists-and-channels"`; `""`: the first page, Welcome).
    pub fn new(doc: &str) -> Self {
        let mut w = Self {
            page: 0,
            query: String::new(),
            hits: Vec::new(),
            searched: String::new(),
            jump: None,
            anchor: None,
            back: Vec::new(),
            to_top: true,
            focus_search: true,
            raise: true,
        };
        w.go(doc);
        w.back.clear();
        w
    }

    /// Show `doc` (a page, a `#section`, or both).
    pub fn go(&mut self, doc: &str) {
        let (file, anchor) = match doc.split_once('#') {
            Some((f, a)) => (f, Some(a.to_owned())),
            None => (doc, None),
        };
        if !file.is_empty() {
            let file = file.rsplit('/').next().unwrap_or(file);
            let idx = page_index(file).or_else(|| match file {
                "README.md" => page_index("README.md"),
                _ => None,
            });
            if let Some(idx) = idx {
                if idx != self.page {
                    self.back.push(self.page);
                }
                self.page = idx;
                self.to_top = anchor.is_none();
            }
        } else if anchor.is_none() {
            if self.page != 0 {
                self.back.push(self.page);
            }
            self.page = 0;
            self.to_top = true;
        }
        self.anchor = anchor;
        self.jump = None;
        self.raise = true;
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let id = ViewportId::from_hash_of("maharaj-help");
        let title = "Maharaj Downloader Help";
        let mut builder = ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([940.0, 640.0])
            .with_min_inner_size([620.0, 400.0])
            .with_resizable(true)
            .with_icon(crate::app_icon());
        if let Some(p) = crate::dialogs::place(ctx, egui::Id::new("maharaj-help-place"), [940.0, 640.0]) {
            builder = builder.with_position(p);
        }
        crate::crash::window("maharaj-help");
        let mut open = true;
        let raise = std::mem::take(&mut self.raise);
        ctx.show_viewport_immediate(id, crate::quiet(builder), |ui, _| {
            if ui.input(|i| i.viewport().close_requested()) {
                open = false;
            }
            if raise && !crate::offscreen() {
                ui.ctx().send_viewport_cmd(ViewportCommand::Focus);
            }
            #[cfg(windows)]
            {
                let _ = crate::win32::caption_dark(title, crate::theme::dark_surface());
            }
            egui::CentralPanel::no_frame()
                .frame(egui::Frame::new().fill(winui::BG()))
                .show(ui, |ui| self.body(ui));
        });
        open
    }

    fn body(&mut self, ui: &mut Ui) {
        let text = winui::TEXT();
        let muted = winui::MUTED();
        egui::Panel::left("maharaj-help-left")
            .resizable(true)
            .default_size(290.0)
            .min_size(200.0)
            .frame(
                egui::Frame::new()
                    .fill(winui::LIST_HEADER())
                    .inner_margin(egui::Margin::same(8)),
            )
            .show(ui, |ui| {
                ui.label(RichText::new("Search the Help").strong().color(text));
                let edit = ui.add(
                    egui::TextEdit::singleline(&mut self.query)
                        .hint_text("Type words to find, e.g. mp3 playlist")
                        .desired_width(f32::INFINITY),
                );
                if self.focus_search {
                    edit.request_focus();
                    self.focus_search = false;
                }
                if self.query != self.searched {
                    self.hits = search(&self.query);
                    self.searched = self.query.clone();
                }
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if !self.query.trim().is_empty() {
                        ui.label(
                            RichText::new(format!("{} results", self.hits.len()))
                                .size(11.0)
                                .color(muted),
                        );
                        if ui.small_button("Clear").clicked() {
                            self.query.clear();
                        }
                    } else {
                        ui.label(RichText::new("Topics").size(11.0).color(muted));
                    }
                });
                ui.separator();
                egui::ScrollArea::vertical()
                    .id_salt("maharaj-help-nav")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.query.trim().is_empty() {
                            for (k, p) in pages().iter().enumerate() {
                                let chosen = k == self.page;
                                let label = RichText::new(&p.title).color(text);
                                if ui
                                    .selectable_label(chosen, if chosen { label.strong() } else { label })
                                    .clicked()
                                    && !chosen
                                {
                                    self.back.push(self.page);
                                    self.page = k;
                                    self.to_top = true;
                                    self.anchor = None;
                                }
                            }
                        } else if self.hits.is_empty() {
                            ui.label(
                                RichText::new("Nothing found. Try fewer or other words.")
                                    .color(muted),
                            );
                        } else {
                            let mut go = None;
                            for (k, h) in self.hits.iter().take(80).enumerate() {
                                let page = &pages()[h.page];
                                let head = if h.heading == page.title {
                                    page.title.clone()
                                } else {
                                    format!("{} › {}", page.title, h.heading)
                                };
                                let r = ui.add(
                                    egui::Button::selectable(
                                        false,
                                        RichText::new(head).strong().color(text),
                                    )
                                    .wrap(),
                                );
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(&h.snippet).size(11.0).color(muted),
                                    )
                                    .wrap(),
                                );
                                if k + 1 < self.hits.len().min(80) {
                                    ui.add_space(4.0);
                                }
                                if r.clicked() {
                                    go = Some((h.page, h.block));
                                }
                            }
                            if let Some((page, block)) = go {
                                if page != self.page {
                                    self.back.push(self.page);
                                }
                                self.page = page;
                                self.jump = Some(block);
                                self.anchor = None;
                                self.to_top = block == 0;
                            }
                        }
                    });
            });
        egui::CentralPanel::no_frame()
            .frame(egui::Frame::new().fill(winui::BG()).inner_margin(egui::Margin::same(14)))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(!self.back.is_empty(), egui::Button::new("Back"))
                        .clicked()
                        && let Some(p) = self.back.pop()
                    {
                        self.page = p;
                        self.to_top = true;
                        self.anchor = None;
                    }
                    if ui.button("Contents").clicked() {
                        self.go("README.md");
                    }
                    if ui.button("Welcome").clicked() {
                        self.go("");
                    }
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if ui
                            .link(RichText::new(format!("Contact: {EMAIL}")).size(11.0).color(winui::LINK()))
                            .clicked() {
                            mail_author("Maharaj Downloader help");
                        }
                    });
                });
                ui.separator();
                let page = self.page;
                let mut area = egui::ScrollArea::vertical()
                    .id_salt(("maharaj-help-page", page))
                    .auto_shrink([false, false]);
                if self.to_top {
                    area = area.vertical_scroll_offset(0.0);
                    self.to_top = false;
                }
                let terms = terms_of(&self.query);
                let mut nav: Option<String> = None;
                area.show(ui, |ui| {
                    ui.set_max_width(ui.available_width().min(820.0));
                    for (k, b) in pages()[page].blocks.iter().enumerate() {
                        let r = draw_block(ui, b, &terms, &mut nav);
                        let target = self.jump == Some(k)
                            || matches!((b, &self.anchor), (Block::Heading(_, _, a), Some(want)) if a == want);
                        if target {
                            r.scroll_to_me(Some(Align::TOP));
                            self.jump = None;
                            self.anchor = None;
                        }
                    }
                    ui.add_space(24.0);
                });
                if let Some(link) = nav {
                    self.follow(&link);
                }
            });
    }

    fn follow(&mut self, link: &str) {
        if link.starts_with("http://") || link.starts_with("https://") || link.starts_with("mailto:") {
            open(link.to_owned());
        } else if let Some(anchor) = link.strip_prefix('#') {
            self.anchor = Some(anchor.to_owned());
        } else {
            let file = link.split('#').next().unwrap_or(link);
            let name = file.rsplit('/').next().unwrap_or(file);
            if name.ends_with(".md") && page_index(name).is_some() {
                let doc = match link.split_once('#') {
                    Some((_, a)) => format!("{name}#{a}"),
                    None => name.to_owned(),
                };
                self.go(&doc);
            } else if name.ends_with(".md") {
                // A page of the upstream project's repository (not part of the guide).
                open(format!("{UPSTREAM_URL}/blob/main/docs/{name}"));
            }
        }
    }
}

/// Paint one block; links clicked land in `nav`.
fn draw_block(ui: &mut Ui, b: &Block, terms: &[String], nav: &mut Option<String>) -> egui::Response {
    let text = winui::TEXT();
    match b {
        Block::Heading(level, inl, _) => {
            let size = match level {
                1 => 22.0,
                2 => 17.0,
                3 => 14.5,
                _ => 13.0,
            };
            ui.add_space(if *level <= 2 { 10.0 } else { 6.0 });
            let r = ui
                .horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    spans(ui, inl, size, true, terms, nav, Some(winui::accent()));
                })
                .response;
            if *level <= 2 {
                ui.add(egui::Separator::default().spacing(4.0));
            }
            r
        }
        Block::Para(inl) => {
            let r = ui
                .horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    spans(ui, inl, 13.0, false, terms, nav, None);
                })
                .response;
            ui.add_space(6.0);
            r
        }
        Block::Item(depth, num, inl) => {
            let r = ui
                .horizontal(|ui| {
                    ui.add_space(8.0 + 16.0 * *depth as f32);
                    ui.label(RichText::new(num.as_deref().unwrap_or("•")).size(13.0).color(text));
                    ui.add_space(4.0);
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            spans(ui, inl, 13.0, false, terms, nav, None);
                        });
                    });
                })
                .response;
            ui.add_space(2.0);
            r
        }
        Block::Code(code) => {
            let r = egui::Frame::new()
                .fill(winui::LIST_HEADER())
                .stroke(Stroke::new(1.0, winui::GROUP_LINE()))
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            RichText::new(code.trim_end()).monospace().size(12.0).color(text),
                        )
                        .wrap(),
                    );
                })
                .response;
            ui.add_space(6.0);
            r
        }
        Block::Quote(inl) => {
            let r = egui::Frame::new()
                .fill(winui::LIST_HEADER())
                .stroke(Stroke::new(1.0, winui::accent()))
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        spans(ui, inl, 12.5, false, terms, nav, None);
                    });
                })
                .response;
            ui.add_space(6.0);
            r
        }
        Block::Table(rows) => {
            let cols = rows.iter().map(|r| r.len()).max().unwrap_or(1);
            let r = egui::Frame::new()
                .stroke(Stroke::new(1.0, winui::GROUP_LINE()))
                .inner_margin(egui::Margin::same(4))
                .show(ui, |ui| {
                    egui::Grid::new(ui.next_auto_id())
                        .striped(true)
                        .num_columns(cols)
                        .spacing(vec2(14.0, 4.0))
                        .show(ui, |ui| {
                            for (ri, row) in rows.iter().enumerate() {
                                for cell in row {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.set_max_width(320.0);
                                        ui.spacing_mut().item_spacing.x = 0.0;
                                        spans(ui, cell, 12.0, ri == 0, terms, nav, None);
                                    });
                                }
                                ui.end_row();
                            }
                        });
                })
                .response;
            ui.add_space(6.0);
            r
        }
        Block::Rule => ui.separator(),
    }
}

/// Inline spans, word by word so the paragraph wraps; words of the search highlighted.
fn spans(
    ui: &mut Ui,
    inl: &[Inline],
    size: f32,
    strong: bool,
    terms: &[String],
    nav: &mut Option<String>,
    color: Option<Color32>,
) {
    let text = color.unwrap_or_else(winui::TEXT);
    let mark = Color32::from_rgba_unmultiplied(255, 200, 0, 110);
    for span in inl {
        let (s, bold, code, link) = match span {
            Inline::Text(s) => (s.as_str(), strong, false, None),
            Inline::Bold(s) => (s.as_str(), true, false, None),
            Inline::Code(s) => (s.as_str(), strong, true, None),
            Inline::Link(t, target) => (t.as_str(), strong, false, Some(target.as_str())),
        };
        // Words with the space after them, so line breaks fall between words.
        let mut words: Vec<String> = Vec::new();
        let mut cur = String::new();
        for ch in s.chars() {
            cur.push(ch);
            if ch == ' ' {
                words.push(std::mem::take(&mut cur));
            }
        }
        if !cur.is_empty() {
            words.push(cur);
        }
        for w in words {
            let low = w.to_lowercase();
            let hit = terms.iter().any(|t| low.contains(t.as_str()));
            let mut rich = RichText::new(&w).size(size);
            if code {
                rich = rich.monospace().background_color(winui::LIST_HEADER());
            }
            if bold {
                rich = rich.family(egui::FontFamily::Name("tahoma-bold".into()));
            }
            if hit {
                rich = rich.background_color(mark);
            }
            match link {
                Some(target) => {
                    let r = ui.link(rich.color(winui::LINK())).on_hover_text(target);
                    if r.clicked() {
                        *nav = Some(target.to_owned());
                    }
                }
                None => {
                    ui.label(rich.color(text));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_page_parses_with_a_title() {
        assert!(pages().len() > 30, "the guide and Maharaj's own pages are built in");
        for p in pages() {
            assert!(!p.title.is_empty(), "{} has a title", p.file);
            assert!(!p.blocks.is_empty(), "{} has content", p.file);
        }
        assert!(page_index("README.md").is_some());
        assert!(page_index("video.md").is_some());
        assert!(page_index("terms-of-use.md").is_some());
    }

    #[test]
    fn search_finds_mp3_and_ranks_headings() {
        let hits = search("mp3");
        assert!(!hits.is_empty());
        let hits = search("terms of use");
        assert!(hits.iter().any(|h| pages()[h.page].file == "terms-of-use.md"));
        assert!(search("zzqqxx-not-a-word").is_empty());
        assert!(search("   ").is_empty());
    }

    #[test]
    fn inline_markdown() {
        assert_eq!(
            inline("a **b** `c` [d](e.md#f) ![img](x.png) *g*"),
            vec![
                Inline::Text("a ".into()),
                Inline::Bold("b".into()),
                Inline::Text(" ".into()),
                Inline::Code("c".into()),
                Inline::Text(" ".into()),
                Inline::Link("d".into(), "e.md#f".into()),
                Inline::Text(" ".into()),
                Inline::Text(" g".into()),
            ]
        );
        assert_eq!(slug("Playlists and channels"), "playlists-and-channels");
        assert_eq!(slug("yt-dlp and ffmpeg"), "yt-dlp-and-ffmpeg");
    }

    #[test]
    fn the_terms_say_what_they_must() {
        for must in [
            "PERSONAL, NON-COMMERCIAL USE ONLY",
            "YOU ARE SOLELY RESPONSIBLE",
            "I AGREE (OK)",
            "AS IS",
            "LIMITATION OF LIABILITY",
            "hamvu3ini@gmail.com",
            "GNU General Public License",
        ] {
            assert!(TERMS.contains(must), "terms mention {must}");
        }
    }

    #[test]
    fn help_requests_are_taken_once() {
        request_help("video.md#playlists-and-channels");
        assert_eq!(
            take_help_request().as_deref(),
            Some("video.md#playlists-and-channels")
        );
        assert_eq!(take_help_request(), None);
    }
}
