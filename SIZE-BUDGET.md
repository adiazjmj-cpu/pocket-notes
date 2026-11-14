# Pocket Notes — Size Budget (standing rule)

_Added 2026-11-13, from Andy's question: the professional alternatives are not insanely huge, and a lot of work went into them — so how big is OneNote, and what keeps our size from ballooning while never sacrificing quality?_

## The comparison (verified 2026-11-13)

| App | Download / install size | Why |
|---|---|---|
| OneNote, Mac (Mac App Store, v16.113.1) | **626 MB** | Native Office app, large binary + resources |
| OneNote, Windows | 7.1 MB stub installer, **~330–590 MB installed** | The 7.1 MB file streams the rest during install; the executables alone are ~330–410 MB |
| Logseq, Windows (v2.0.1) | **~135 MB** | Electron — bundles a whole Chromium |
| Obsidian, desktop | **~124 MB** | Electron — bundles a whole Chromium |
| Pocket Notes, desktop (Tauri, target) | **under 20 MB** | Tauri uses the system webview; the app itself is one ~173 KB HTML file |

The point Andy made is right: the professional alternatives are not insanely huge — but they are 100–600 MB, and most of that is runtime they bundle (Chromium for Electron, the Office runtime for OneNote). Pocket Notes does not bundle a runtime. That is the size advantage, and it is structural, not accidental.

A caution from the same research: Logseq's new database version turned a 42 MB vault into **575 MB on disk** (db.sqlite 222 MB, client-ops 209 MB, search 138 MB) — nearly 14x the source. App size and data footprint are different things, and both need a budget. The data-footprint budget lives in PERF-BUDGET.md (attachments as blobs, index/query layer).

## The budget

- **Each desktop installer (Mac .dmg, Windows .msi/.exe): under 20 MB.** A generous ceiling, not a tight one — the app file is ~173 KB and the Tauri shell is a few MB, so 20 MB leaves ample room for the block model, query layer, and canvas work. If a build exceeds it, the build must not ship.
- **The app file (pocket-notes.html): under 500 KB.** It is 173 KB at v18. This leaves room to grow (blocks, queries, canvas) without ballooning.
- **The web app (PWA) total: under 1 MB** (HTML + manifest + icons). It is ~180 KB at v18.

## Guardrails (set in place, not just listed)

1. **Tauri, not Electron — standing rule.** Tauri uses the system webview (WebKit on Mac, WebView2 on Windows). Electron bundles a whole Chromium (~100 MB+) and would blow the budget by itself. Never switch to Electron.
2. **No bundled runtime, no bundled assets.** Single HTML file, inline SVG icons, system fonts. No web fonts, no bundled images, no bundled media. The PWA icons are 1–3 KB PNGs each.
3. **Release binary optimized for size.** `src-tauri/Cargo.toml` `[profile.release]`: `opt-level = "s"`, `strip = true`, `lto = true`, `codegen-units = 1`, `panic = "abort"`.
4. **Audit every dependency.** Every new Rust crate or JS library must be justified; no dependency for a trivial function. Today: zero JS dependencies (pure JS) and minimal Rust deps (Tauri + dialog plugin). Keep it that way.
5. **Measure on every build — enforced in CI.** `.github/workflows/build.yml` has a "Check installer size (20 MB budget)" step after the build: it lists the installer sizes and **fails the job** if any installer exceeds 20 MB. The size is checked automatically, not by memory. If it fails, find what ballooned it and shrink it — do not raise the budget.
6. **User data is separate from app size.** Notes live in IndexedDB (user data), not in the app bundle. The app stays small no matter how many notes the user has. The budget covers the app, not the user's data — the data footprint is governed by PERF-BUDGET.md.
7. **No feature justifies bloat.** If a feature would balloon the size (e.g., bundling a large library), find the smaller way or do not ship it. Quality is not the same as size — and a smaller, faster app is higher quality, not lower.

## How the budget is verified

- CI: the workflow size-check step (above) runs on every build and fails over 20 MB.
- Locally: after any change that adds code or a dependency, check the app file size (`wc -c pocket-notes.html`, target < 500 KB) and the dependency list (`Cargo.toml`, `package.json` if one ever appears).
- The budget is recorded here and in MEMORY.md; raising it is a decision for Andy, not a silent edit.

_Refs: OneNote Mac 626.2 MB / Windows 7.1 MB stub (techdows.com, verified 2026-09-19); OneNote installed executables ~330–410 MB (advanceduninstaller.com); Logseq 134.9 MB (logseq 2.0.1 release); Obsidian ~124 MB desktop (altapp.store); Logseq DB 42 MB vault → 575 MB on disk (computingforgeeks.com, Oct 2026)._
