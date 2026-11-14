# Pocket Notes — desktop app (Tauri 2)

A Tauri 2.x shell that turns the Pocket Notes webapp into a fully functional
downloadable app for **macOS** (Apple Silicon arm64 + Intel x86_64, `.dmg`) and
**Windows** (x86_64, `.msi` + `.exe` NSIS installer). Built on GitHub Actions
from this project; nothing is built on the developer's machine.

## Feature parity — the desktop app IS the webapp

The desktop app wraps the **identical** Pocket Notes HTML file (currently v17).
It is not a cut-down build. Same file, same features, same storage behaviour,
same capability:

- Notebooks, sections, pages; the rich editor; `[[links]]` and backlinks;
  tags; pinning; search (with `tag:`, `@todo`, `@images` operators); Home,
  Board (with drag between sections), Calendar and Graph views; templates;
  Related pages and "Mentioned in"; the Modules panel; Focus Mode.
- Import/export in the open Weave (`.weave`) format plus `.md`, `.txt`, `.html`.
- Photos (with captions), voice notes (with transcripts), drawings (with
  descriptions), Scan mode — all stored inside the app's own storage.
- The v15 storage engine unchanged: IndexedDB is the primary store, localStorage
  is the fallback with the visible "Limited storage (~5 MB)" badge, failed saves
  retry once then say so and name the recovery (export a backup). History is
  capped at 200 entries. The v16 "Clear all storage" button is in the Storage
  dialog and wipes both stores, then reloads to a fresh seed.
- The v17 PWA additions (manifest, apple-touch-icon, service worker) are inert
  here: the service worker only registers over http(s), and under the desktop
  shell the page is served from the `tauri://` protocol, so registration is
  skipped. The desktop app does not need a service worker — its files are local
  and always available offline.

The native shell only **adds** two things:

1. **Native file dialogs.** Import already uses the browser file picker, which
   the webview renders as the native macOS/Windows picker. Export goes through
   the native Save As dialog (see "Export on Windows" below) instead of a
   browser download.
2. **Native storage with no purge risk.** Notes live in the desktop webview's
   own persistent data store on the machine — the OS-managed app data, not a
   browser's evictable web cache. Unlike the iPhone/iPad home-screen web app,
   which iOS can purge if unused, the desktop app's storage does not carry that
   risk. The same "export a backup (.weave)" habit is still the safety net.

## Known gaps — flagged, not hidden

### Export on Windows: OPEN — needs a small v18 edit to the live file

**The gap.** The webview silently drops the browser anchor download
(`<a download>` with a blob URL) on Windows — WebView2 has no browser download
manager to hand it to. This is documented behaviour, confirmed by several
independent Tauri 2 projects (their export had to be re-routed the same way).
So in the desktop app as it stands (wrapping v17 as-is), **Export backup
(.weave) and page/notebook export would silently do nothing on Windows**.
That is a real feature loss, and the Windows installer must not be called
"fully functional" until it is closed.

**The shell-side half of the fix is already in this project** (it is the only
reason the dialog plugin and the save command exist):

- `tauri-plugin-dialog` in `src-tauri/Cargo.toml`, initialised in `src/main.rs`,
  with the `dialog:default` permission in `src-tauri/capabilities/default.json`.
- The `save_text_file` Rust command in `src/main.rs`: writes text to a path the
  user picked. All exports are text (`.weave`, `.md`, `.html`, `.txt`), so one
  command covers them. App-defined commands need no capability entry.
- `app.withGlobalTauri: true` in `tauri.conf.json`, so the shell injects
  `window.__TAURI__` (with the dialog plugin's JS under
  `window.__TAURI__.dialog`) into the page. Tauri 2 does **not** inject it by
  default — without this the live file could never detect the shell.

**The remaining half is a small v18 edit to the live app file** (not in this
project — the live file is the single source of truth and is never edited
here). In the app's export/download helper, before creating the anchor:

```js
if (window.__TAURI__ && window.__TAURI__.dialog && window.__TAURI__.core) {
  const path = await window.__TAURI__.dialog.save({
    defaultPath: filename,
    filters: [{ name: "Weave backup", extensions: ["weave"] }] // per export type
  });
  if (!path) return false; // user cancelled
  await window.__TAURI__.core.invoke("save_text_file", { path, contents: text });
  return true;
}
// otherwise: the existing anchor download (plain browser / PWA / gizmo)
```

That branch runs only inside the desktop shell; in a plain browser, the PWA
and the Pocket gizmo `window.__TAURI__` is absent and the existing anchor path
runs unchanged. Until that v18 edit lands and is verified on a Windows build,
treat Windows export as **not working**.

### Export on macOS: expected to work, NOT verified on hardware

Tauri's WKWebView download delegate should handle the anchor download on macOS
(unlike WebView2, which drops it). That is an expectation, not a verified
result — it needs a real macOS build to confirm. The same v18 native-save
wiring above would cover macOS too, and would give a real Save As dialog
instead of a silent save to Downloads, so wiring it is the safer path either
way.

### Microphone (voice notes) on macOS: Info.plist added, needs a real signed build to verify

Voice notes use the webview's `MediaRecorder`. On macOS that needs the usage
description, which is in `src-tauri/Info.plist`
(`NSMicrophoneUsageDescription` — Tauri merges this file into the generated
bundle). The OS then asks the user at the first voice-note recording, not at
launch. This can only truly be proven by a signed macOS build; the unsigned CI
builds here are not code-signed. If the app is ever code-signed/notarized for
distribution, the `com.apple.security.device.audio-input` entitlement must also
be added. On Windows, WebView2 prompts for the microphone in-app and needs no
extra config.

### Deliberately inert on desktop (not a loss)

- `navigator.vibrate` (tap haptics) does nothing on desktop — there is no
  haptic hardware. Phone-only, by nature.
- "Add to Home Screen" / PWA install is not applicable — the desktop app IS
  the install.

## The Logseq-level capability bar

Andy set the bar higher than "match the webapp": the downloadable apps must be
"just as capable as logseq and whatnot." That capability — Logseq's block-based
outliner (blocks, not just pages; block references/embeds) and the Bases/Dataview
live-query layer — lives in the **app code itself**, not in this desktop
wrapper. It ships to the webapp AND the desktop app together (same file) once
it is built. This Tauri project delivers the downloadable app; the Logseq-level
capability is the next app upgrade after it, not a packaging concern.

## How to update the app

The live file `../pocket-notes.html` is the single source of truth. Never edit
`dist/index.html` by hand — regenerate it from the live file:

```sh
cp ../pocket-notes.html dist/index.html
sha256sum ../pocket-notes.html dist/index.html   # the two hashes must match
```

Then bump `version` in `src-tauri/Cargo.toml` and in `src-tauri/tauri.conf.json`
to match the app version (currently `17.0.0` for v17), and commit.

## How to build (GitHub Actions)

1. Create a new GitHub repository (pick any name — the name is yours to set;
   nothing in this project hard-codes one) and push the contents of this
   `desktop/` folder as the repository root, so the workflow lands at
   `.github/workflows/build.yml` in the repo.
2. Push to `main` (or run the workflow manually from the Actions tab).
   The workflow builds on GitHub's own runners — macOS for the two `.dmg`
   files (Apple Silicon arm64 and Intel x86_64) and Windows for the `.msi` and
   the NSIS `.exe` — and uploads them as run artifacts
   (`pocket-notes-aarch64-apple-darwin`, `pocket-notes-x86_64-apple-darwin`,
   `pocket-notes-x86_64-pc-windows-msvc`).
3. Download the installers from the completed run's Artifacts section.

Local builds need the Rust toolchain (`rustup`) and the Tauri CLI; the CI path
above is the intended one, since a Linux machine cannot produce the macOS
`.dmg` or the Windows installers.

## iOS is parked

There is deliberately no iOS target here. Publishing to the App Store needs a
paid Apple Developer Program membership ($99/year), which the project does not
have. The free path to iPhone/iPad is the PWA home-screen app (`../pwa/`):
Add to Home Screen in Safari, full-screen, no App Store, no fee — with the
honest limit that iOS can purge a home-screen web app's data if unused, so
`.weave` backups matter even more there.

## Project layout

```
desktop/
  dist/index.html                  copy of ../pocket-notes.html (regenerate, never edit)
  src-tauri/
    Cargo.toml                     package + tauri-plugin-dialog
    tauri.conf.json                productName, identifier, window, bundle targets, CSP, withGlobalTauri
    Info.plist                     NSMicrophoneUsageDescription (merged into the macOS bundle)
    build.rs                       tauri_build::build()
    src/main.rs                    minimal shell + save_text_file command
    capabilities/default.json      core:default + dialog:default
    icons/                         icon.png (512), icon.ico (Win), icon.icns (Mac)
  .github/workflows/build.yml      macOS (arm64 + Intel) + Windows builds, artifacts uploaded
  README.md                        this file
```
