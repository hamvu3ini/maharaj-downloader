// Link the GUI with Windows subsystem version 5.02 (D-008). Windows gives windows of programs
// linked this way the legacy frame metrics — dialogs get IDM's fixed frame (3 px borders,
// 26 px caption, measured: 566×122 outer for a 560×93 client) instead of the padded modern
// frame (8 px, 31 px). Resizable windows keep their 8 px frame, as IDM's main window has.
//
// Also embeds the exe's icon (Explorer, taskbar before the window sets its own) and version
// info.
fn main() {
    catalogs();
    help_pages();
    commit();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let windows = os == "windows";
    // `cfg(gpu)`: wgpu is built, with this system's own graphics API (renderer.rs, #89, #206):
    // Direct3D 12 on Windows, Vulkan on Linux, Metal on macOS (Cargo.toml).
    println!("cargo::rustc-check-cfg=cfg(gpu)");
    if matches!(os.as_str(), "windows" | "linux" | "macos") {
        println!("cargo::rustc-cfg=gpu");
    }
    if windows && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg-bins=/SUBSYSTEM:WINDOWS,5.02");
    }
    // Maharaj Downloader: built for Windows on Linux (MinGW), the icon and version info come
    // from MinGW's windres instead of winresource (which runs on Windows only).
    #[cfg(not(windows))]
    if windows {
        cross_resources();
    }
    // The icon and version info are for a Windows program only (not one built on Windows for
    // Linux or macOS); winresource builds on Windows only.
    #[cfg(windows)]
    if windows {
        println!("cargo:rerun-if-changed=assets/icons/app/mdm.ico");
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icons/app/mdm.ico")
            .set("ProductName", "Maharaj Downloader")
            .set("FileDescription", "Maharaj Downloader")
            .set("CompanyName", "Sasanka Boruah (VU3INI)")
            .set(
                "LegalCopyright",
                "Maharaj Downloader (C) 2026 Sasanka Boruah, based on Mister Download Manager (C) 2026 Ahmed Nefzaoui. GPL-3.0-or-later.",
            );
        if let Err(e) = res.compile() {
            // A missing resource compiler must not break the build; the window still sets
            // its icon at runtime.
            println!("cargo:warning=exe icon not embedded: {e}");
        }
    }
}

/// Embed every translation catalog, `i18n/<locale>.tsv` at the repository root (craft-i18n's
/// format; D-019), packed for `i18n::catalogs` to unpack once at start (#25):
/// `OUT_DIR/catalogs.br`, all of them in one brotli stream (quality 11: 12.4 MB of text in
/// about 0.8 MB, about 11 ms to unpack), and `OUT_DIR/catalogs.rs`, each locale's code and its
/// range in the unpacked text. The comment lines (`#`: translators' notes, `#, machine`) are
/// more than half of the text and craft-i18n skips them unread, so they are blanked, each kept as
/// an empty line so a problem's "line N" still points into the catalog file.
fn catalogs() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n");
    println!("cargo:rerun-if-changed={}", root.display());
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut text_all = String::new();
    let mut rows = Vec::new();
    if let Ok(dir) = std::fs::read_dir(&root) {
        let mut files: Vec<_> = dir.flatten().map(|e| e.path()).collect();
        files.sort();
        for path in files {
            let (Some(stem), Some("tsv")) = (
                path.file_stem().and_then(|s| s.to_str()).map(str::to_owned),
                path.extension().and_then(|s| s.to_str()),
            ) else {
                continue;
            };
            // `en.tsv` lists the source texts (for translators and checks); English needs no
            // catalog at run time. `languages.tsv` is the list of languages, not a catalog.
            if stem == "en" || stem == "languages" {
                continue;
            }
            println!("cargo:rerun-if-changed={}", path.display());
            let text = std::fs::read_to_string(&path).expect("a translation catalog");
            let rows_only: Vec<&str> = text
                .split('\n')
                .map(|line| if line.starts_with('#') { "" } else { line })
                .collect();
            let start = text_all.len();
            text_all.push_str(&rows_only.join("\n"));
            rows.push(format!("    ({stem:?}, {start}, {}),", text_all.len()));
        }
    }
    let params = brotli::enc::BrotliEncoderParams {
        quality: 11,
        lgwin: 24,
        size_hint: text_all.len(),
        ..Default::default()
    };
    let mut packed = Vec::new();
    brotli::BrotliCompress(&mut text_all.as_bytes(), &mut packed, &params)
        .expect("packing the catalogs");
    std::fs::write(out_dir.join("catalogs.br"), packed).unwrap();
    std::fs::write(
        out_dir.join("catalogs.rs"),
        format!("&[\n{}\n]\n", rows.join("\n")),
    )
    .unwrap();
    languages(&root);
}

/// `i18n/languages.tsv` (D-019) as `(code, English name, own name, flag SVG)` rows, the flags
/// from `assets/flags/` embedded: `OUT_DIR/languages.rs`, for the language picker.
fn languages(root: &std::path::Path) {
    let table = root.join("languages.tsv");
    println!("cargo:rerun-if-changed={}", table.display());
    let flags = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/flags");
    println!("cargo:rerun-if-changed={}", flags.display());
    let text = std::fs::read_to_string(&table).expect("i18n/languages.tsv");
    let mut header: Vec<&str> = Vec::new();
    let mut rows = Vec::new();
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("# code\t") {
            header = std::iter::once("code").chain(h.split('\t')).collect();
            continue;
        }
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        assert_eq!(cols.len(), header.len(), "i18n/languages.tsv: {line}");
        let col = |name: &str| cols[header.iter().position(|h| *h == name).expect(name)];
        let flag = flags.join(format!("{}.svg", col("flag")));
        assert!(flag.exists(), "missing flag {}", flag.display());
        let flag = flag.canonicalize().unwrap_or(flag);
        rows.push(format!(
            "    ({:?}, {:?}, {:?}, include_bytes!({:?})),",
            col("code"),
            col("english"),
            col("native"),
            flag.display().to_string()
        ));
    }
    let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("languages.rs");
    std::fs::write(out, format!("&[\n{}\n]\n", rows.join("\n"))).unwrap();
}

/// The commit this is built from (`MDM_COMMIT`, "unknown" outside a git checkout), for Help →
/// About and problem reports.
fn commit() {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    let hash = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .filter(|h| !h.is_empty())
        .unwrap_or_else(|| "unknown".into());
    println!("cargo:rustc-env=MDM_COMMIT={hash}");
}

/// The exe's icon and version info when cross-compiling for Windows (MinGW's windres).
#[cfg(not(windows))]
fn cross_resources() {
    println!("cargo:rerun-if-changed=assets/icons/app/mdm.ico");
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let ico = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/icons/app/mdm.ico");
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let mut parts: Vec<u32> = version.split('.').filter_map(|p| p.parse().ok()).collect();
    parts.resize(4, 0);
    let comma = parts.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(",");
    let rc = format!(
        r#"1 ICON "{ico}"
1 VERSIONINFO
FILEVERSION {comma}
PRODUCTVERSION {comma}
FILEOS 0x40004
FILETYPE 0x1
BEGIN
  BLOCK "StringFileInfo"
  BEGIN
    BLOCK "040904B0"
    BEGIN
      VALUE "CompanyName", "Sasanka Boruah (VU3INI)"
      VALUE "FileDescription", "Maharaj Downloader"
      VALUE "FileVersion", "{version}"
      VALUE "InternalName", "MaharajDownloader"
      VALUE "LegalCopyright", "Maharaj Downloader (C) 2026 Sasanka Boruah, based on Mister Download Manager (C) 2026 Ahmed Nefzaoui. GPL-3.0-or-later."
      VALUE "OriginalFilename", "MaharajDownloader.exe"
      VALUE "ProductName", "Maharaj Downloader"
      VALUE "ProductVersion", "{version}"
    END
  END
  BLOCK "VarFileInfo"
  BEGIN
    VALUE "Translation", 0x409, 1200
  END
END
"#,
        ico = ico.display().to_string().replace('\\', "/")
    );
    let rc_path = out.join("maharaj.rc");
    let obj = out.join("maharaj-res.o");
    if std::fs::write(&rc_path, rc).is_err() {
        return;
    }
    let windres = std::env::var("WINDRES").unwrap_or_else(|_| "x86_64-w64-mingw32-windres".into());
    // crash-handler calls MSVC's `_invoke_watson`, which MinGW's C runtime lacks: the same as
    // MSVC's, end the process with STATUS_STACK_BUFFER_OVERRUN (the crash monitor reports it).
    let shim_c = out.join("watson.c");
    let shim_o = out.join("watson.o");
    let _ = std::fs::write(
        &shim_c,
        "#include <windows.h>\n#include <stdint.h>\n\
         __declspec(noreturn) void _invoke_watson(const wchar_t *e, const wchar_t *f, \
         const wchar_t *file, unsigned int line, uintptr_t r) {\n\
         (void)e; (void)f; (void)file; (void)line; (void)r;\n\
         TerminateProcess(GetCurrentProcess(), 0xC0000409u);\n for (;;) {}\n}\n",
    );
    let gcc = std::env::var("CC_x86_64_pc_windows_gnu").unwrap_or_else(|_| "x86_64-w64-mingw32-gcc".into());
    match std::process::Command::new(gcc)
        .args(["-c", "-O2", "-o"])
        .arg(&shim_o)
        .arg(&shim_c)
        .status()
    {
        Ok(s) if s.success() => println!("cargo:rustc-link-arg-bins={}", shim_o.display()),
        other => println!("cargo:warning=_invoke_watson shim not built: {other:?}"),
    }
    match std::process::Command::new(windres)
        .arg(&rc_path)
        .args(["-O", "coff", "-o"])
        .arg(&obj)
        .status()
    {
        Ok(s) if s.success() => println!("cargo:rustc-link-arg-bins={}", obj.display()),
        other => println!("cargo:warning=exe icon not embedded (windres: {other:?})"),
    }
}

/// Maharaj Downloader's Help: its own pages (assets/brand/help/*.md, in file order), then the
/// user guide (docs/*.md: the contents first, then the rest by file name), embedded as
/// `OUT_DIR/help_pages.rs` for maharaj.rs to search and show offline.
fn help_pages() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let own = manifest.join("assets/brand/help");
    let docs = manifest.join("../../docs");
    println!("cargo:rerun-if-changed={}", own.display());
    println!("cargo:rerun-if-changed={}", docs.display());
    let list = |dir: &std::path::Path| -> Vec<std::path::PathBuf> {
        let mut v: Vec<_> = std::fs::read_dir(dir)
            .map(|d| d.flatten().map(|e| e.path()).collect())
            .unwrap_or_default();
        v.retain(|p: &std::path::PathBuf| p.extension().is_some_and(|e| e == "md"));
        v.sort();
        v
    };
    let mut files = list(&own);
    let mut guide = list(&docs);
    guide.retain(|p| p.file_name().is_some_and(|n| n != "ARCHITECTURE.md"));
    guide.sort_by_key(|p| p.file_name().is_none_or(|n| n != "README.md"));
    files.extend(guide);
    let mut code = String::from("&[\n");
    for f in files {
        println!("cargo:rerun-if-changed={}", f.display());
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        let abs = std::fs::canonicalize(&f).unwrap_or(f);
        code.push_str(&format!("    ({name:?}, include_str!({:?})),\n", abs.display().to_string()));
    }
    code.push(']');
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(out.join("help_pages.rs"), code).unwrap();
}
